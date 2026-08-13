use crate::commands::extract::{ExtractedTiffMetadata, extract_tiff_metadata};
use crate::commands::validate::{ValidationReport, validate_json_value_report_with_source};
use crate::schema::SchemaVersion;
use crate::utils::{load_json, write_bytes_atomically};
use chrono::{NaiveDateTime, SecondsFormat, Utc};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapErrorKind {
    Mapping,
    Operational,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapError {
    kind: MapErrorKind,
    message: String,
}

impl MapError {
    fn mapping(message: impl Into<String>) -> Self {
        Self {
            kind: MapErrorKind::Mapping,
            message: message.into(),
        }
    }

    fn operational(message: impl Into<String>) -> Self {
        Self {
            kind: MapErrorKind::Operational,
            message: message.into(),
        }
    }

    pub fn exit_code(&self) -> u8 {
        match self.kind {
            MapErrorKind::Mapping => 1,
            MapErrorKind::Operational => 2,
        }
    }
}

impl fmt::Display for MapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for MapError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedMapping {
    pub target: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct MapReport {
    pub output_path: String,
    pub mappings_applied: usize,
    pub skipped: Vec<SkippedMapping>,
    pub validation: ValidationReport,
}

#[derive(Debug)]
struct Connector {
    target_schema_version: String,
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    source: SourceSpec,
    target: String,
    required: bool,
    transform: Option<TransformInvocation>,
}

#[derive(Debug, Clone)]
enum SourceSpec {
    TiffTag { id: u16 },
    Constant { value: Value },
    Runtime { field: RuntimeField },
}

#[derive(Debug, Clone, Copy)]
enum RuntimeField {
    ImageFileName,
    MappingTimestamp,
}

#[derive(Debug, Clone)]
struct RuntimeValues {
    image_file_name: String,
    mapping_timestamp: String,
}

#[derive(Debug)]
struct TransformInvocation {
    id: String,
    primary_input: String,
    inputs: BTreeMap<String, SourceSpec>,
    parameters: Map<String, Value>,
}

#[derive(Debug)]
struct TransformFailure {
    configuration: bool,
    message: String,
}

impl TransformFailure {
    fn configuration(message: impl Into<String>) -> Self {
        Self {
            configuration: true,
            message: message.into(),
        }
    }

    fn value(message: impl Into<String>) -> Self {
        Self {
            configuration: false,
            message: message.into(),
        }
    }
}

/// Apply a connector to TIFF IFD 0 and write the resulting validated FAMH file.
pub async fn map_tiff_file(
    image_path: &str,
    connector_path: &str,
    out_path: &str,
    connector_schema_path: Option<&str>,
    no_cache: bool,
    revision: Option<&str>,
) -> Result<MapReport, MapError> {
    let connector_value = load_json(connector_path)
        .map_err(|error| MapError::operational(format!("Could not read connector: {error}")))?;
    let connector_path = Path::new(connector_path);
    let schema_path =
        resolve_connector_schema_path(connector_path, connector_schema_path, &connector_value)?;
    let schema_path_string = schema_path
        .to_str()
        .ok_or_else(|| MapError::operational("Connector schema path is not valid UTF-8"))?;
    let connector_schema = load_json(schema_path_string).map_err(|error| {
        MapError::operational(format!(
            "Could not read connector schema '{}': {error}",
            schema_path.display()
        ))
    })?;
    validate_connector(&connector_value, &connector_schema)?;
    let connector = parse_connector(&connector_value).map_err(MapError::mapping)?;
    let schema_version = parse_target_schema_version(&connector.target_schema_version)?;
    let runtime = runtime_values(image_path)?;

    let extracted = extract_tiff_metadata(image_path).map_err(|error| {
        MapError::operational(format!("Could not extract TIFF metadata: {error}"))
    })?;
    reject_extraction_diagnostics(&extracted)?;
    let tags = index_tags(&extracted)?;
    let transform_directory = schema_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("transforms");

    let mut output = Value::Object(Map::new());
    let mut applied = 0;
    let mut skipped = Vec::new();
    let mut targets = HashSet::new();

    for mapping in connector.mappings {
        let target_tokens = decode_pointer(&mapping.target).map_err(MapError::mapping)?;
        if !targets.insert(target_tokens.clone()) {
            return Err(MapError::mapping(format!(
                "Connector contains multiple mappings for target '{}'",
                mapping.target
            )));
        }

        let Some(source_value) = resolve_source(&mapping.source, &tags, &runtime) else {
            if mapping.required {
                return Err(MapError::mapping(format!(
                    "Required source for target '{}' is missing",
                    mapping.target
                )));
            }
            skipped.push(SkippedMapping {
                target: mapping.target,
                reason: "source value is absent".to_owned(),
            });
            continue;
        };

        let value = if let Some(transform) = &mapping.transform {
            match apply_transform(
                transform,
                source_value,
                &tags,
                &runtime,
                &transform_directory,
            ) {
                Ok(value) => value,
                Err(failure) if !failure.configuration && !mapping.required => {
                    skipped.push(SkippedMapping {
                        target: mapping.target,
                        reason: failure.message,
                    });
                    continue;
                }
                Err(failure) => {
                    return Err(MapError::mapping(format!(
                        "Could not apply transform for target '{}': {}",
                        mapping.target, failure.message
                    )));
                }
            }
        } else {
            source_value
        };

        set_json_pointer(&mut output, &target_tokens, value).map_err(|error| {
            MapError::mapping(format!(
                "Could not write target '{}': {error}",
                mapping.target
            ))
        })?;
        applied += 1;
    }

    let validation =
        validate_json_value_report_with_source(&output, schema_version, no_cache, false, revision)
            .await
            .map_err(|error| {
                MapError::operational(format!("Could not validate mapped FAMH metadata: {error}"))
            })?;
    if validation.has_errors() {
        return Err(MapError::mapping(format!(
            "Mapped FAMH metadata is invalid:\n{}",
            validation.render_text()
        )));
    }

    let output_bytes = serde_json::to_vec_pretty(&output).map_err(|error| {
        MapError::operational(format!("Could not serialize mapped FAMH metadata: {error}"))
    })?;
    write_bytes_atomically(out_path, &output_bytes).map_err(|error| {
        MapError::operational(format!(
            "Could not write mapped FAMH metadata to '{}': {error}",
            out_path
        ))
    })?;

    Ok(MapReport {
        output_path: out_path.to_owned(),
        mappings_applied: applied,
        skipped,
        validation,
    })
}

fn resolve_connector_schema_path(
    connector_path: &Path,
    explicit_path: Option<&str>,
    connector: &Value,
) -> Result<PathBuf, MapError> {
    if let Some(path) = explicit_path {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }
        return Err(MapError::operational(format!(
            "Connector schema does not exist: {}",
            path.display()
        )));
    }

    let connector_directory = connector_path.parent().unwrap_or_else(|| Path::new("."));
    let mut candidates = Vec::new();
    if let Some(schema_reference) = connector.get("$schema").and_then(Value::as_str) {
        let reference = Path::new(schema_reference);
        if !reference.is_absolute() && !schema_reference.contains("://") {
            candidates.push(connector_directory.join(reference));
        }
    }
    candidates.push(connector_directory.join("connector-schema.json"));
    candidates.push(
        connector_directory
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("connector-schema.json"),
    );

    candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            MapError::operational(format!(
                "Could not find connector-schema.json beside '{}'; pass --connector-schema PATH",
                connector_path.display()
            ))
        })
}

fn validate_connector(connector: &Value, schema: &Value) -> Result<(), MapError> {
    let validator = jsonschema::validator_for(schema).map_err(|error| {
        MapError::operational(format!(
            "Could not prepare connector schema validation: {error}"
        ))
    })?;
    let errors = validator
        .iter_errors(connector)
        .map(|error| format!("{}: {error}", error.instance_path))
        .collect::<Vec<_>>();
    if errors.is_empty() {
        return Ok(());
    }

    Err(MapError::mapping(format!(
        "Connector does not conform to connector-schema.json:\n{}",
        errors.join("\n")
    )))
}

fn parse_connector(value: &Value) -> Result<Connector, String> {
    let object = value.as_object().ok_or("Connector must be a JSON object")?;
    let target_schema_version = required_string(object, "targetSchemaVersion")?;
    let mapping_values = object
        .get("mappings")
        .and_then(Value::as_array)
        .ok_or("Connector field 'mappings' must be an array")?;
    let mappings = mapping_values
        .iter()
        .map(parse_mapping)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Connector {
        target_schema_version,
        mappings,
    })
}

fn parse_mapping(value: &Value) -> Result<Mapping, String> {
    let object = value
        .as_object()
        .ok_or("Connector mapping must be a JSON object")?;
    let target = required_string(object, "target")?;
    let required = object
        .get("required")
        .map(|value| {
            value
                .as_bool()
                .ok_or("Connector mapping field 'required' must be boolean")
        })
        .transpose()?
        .unwrap_or(false);
    let transform = object
        .get("transform")
        .map(parse_transform_invocation)
        .transpose()?;

    Ok(Mapping {
        source: parse_source(
            object
                .get("source")
                .ok_or("Connector mapping field 'source' is missing")?,
        )?,
        target,
        required,
        transform,
    })
}

fn parse_source(value: &Value) -> Result<SourceSpec, String> {
    let object = value
        .as_object()
        .ok_or("Connector source must be a JSON object")?;
    match object
        .get("type")
        .and_then(Value::as_str)
        .ok_or("Connector source field 'type' is missing")?
    {
        "tiff-tag" => {
            let id = object
                .get("id")
                .and_then(Value::as_u64)
                .ok_or("TIFF tag source field 'id' must be an integer")?;
            let id = u16::try_from(id).map_err(
                |_| "TIFF tag source field 'id' must fit in an unsigned 16-bit TIFF tag ID",
            )?;
            Ok(SourceSpec::TiffTag { id })
        }
        "constant" => Ok(SourceSpec::Constant {
            value: object
                .get("value")
                .cloned()
                .ok_or("Constant source field 'value' is missing")?,
        }),
        "runtime" => match object
            .get("field")
            .and_then(Value::as_str)
            .ok_or("Runtime source field 'field' is missing")?
        {
            "image-file-name" => Ok(SourceSpec::Runtime {
                field: RuntimeField::ImageFileName,
            }),
            "mapping-timestamp" => Ok(SourceSpec::Runtime {
                field: RuntimeField::MappingTimestamp,
            }),
            field => Err(format!("Unsupported runtime source field '{field}'")),
        },
        source_type => Err(format!("Unsupported connector source type '{source_type}'")),
    }
}

fn parse_transform_invocation(value: &Value) -> Result<TransformInvocation, String> {
    match value {
        Value::String(id) => Ok(TransformInvocation {
            id: id.clone(),
            primary_input: "value".to_owned(),
            inputs: BTreeMap::new(),
            parameters: Map::new(),
        }),
        Value::Object(object) => {
            let inputs = object
                .get("inputs")
                .map(|value| {
                    let inputs = value
                        .as_object()
                        .ok_or("Transform invocation field 'inputs' must be an object")?;
                    inputs
                        .iter()
                        .map(|(name, source)| Ok((name.clone(), parse_source(source)?)))
                        .collect::<Result<BTreeMap<_, _>, String>>()
                })
                .transpose()?
                .unwrap_or_default();
            let parameters = object
                .get("parameters")
                .map(|value| {
                    value
                        .as_object()
                        .cloned()
                        .ok_or("Transform invocation field 'parameters' must be an object")
                })
                .transpose()?
                .unwrap_or_default();

            Ok(TransformInvocation {
                id: required_string(object, "id")?,
                primary_input: required_string(object, "primaryInput")?,
                inputs,
                parameters,
            })
        }
        _ => Err("Transform invocation must be a string or object".to_owned()),
    }
}

fn required_string(object: &Map<String, Value>, field: &str) -> Result<String, String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| format!("Connector field '{field}' must be a string"))
}

fn parse_target_schema_version(version: &str) -> Result<SchemaVersion, MapError> {
    match version {
        "1" | "1.1" => Ok(SchemaVersion::V1),
        "2" => Err(MapError::mapping(
            "Connector targetSchemaVersion '2' is not supported by famdo map yet",
        )),
        _ => Err(MapError::mapping(format!(
            "Unsupported connector targetSchemaVersion '{version}'"
        ))),
    }
}

fn reject_extraction_diagnostics(metadata: &ExtractedTiffMetadata) -> Result<(), MapError> {
    if metadata.diagnostics.is_empty() {
        return Ok(());
    }

    let diagnostics = metadata
        .diagnostics
        .iter()
        .map(|diagnostic| format!("IFD {}: {}", diagnostic.ifd, diagnostic.message))
        .collect::<Vec<_>>()
        .join("\n");
    Err(MapError::mapping(format!(
        "TIFF extraction produced diagnostic(s); mapping was not attempted:\n{diagnostics}"
    )))
}

fn index_tags(metadata: &ExtractedTiffMetadata) -> Result<BTreeMap<u16, Value>, MapError> {
    let first_ifd = metadata.ifds.first().ok_or_else(|| {
        MapError::operational("TIFF extraction did not produce an image directory")
    })?;
    let mut tags = BTreeMap::new();
    for tag in &first_ifd.tags {
        if tags.insert(tag.id, tag.value.clone()).is_some() {
            return Err(MapError::mapping(format!(
                "TIFF IFD 0 contains duplicate tag ID {} ({})",
                tag.id, tag.name
            )));
        }
    }
    Ok(tags)
}

fn runtime_values(image_path: &str) -> Result<RuntimeValues, MapError> {
    let image_file_name = Path::new(image_path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .ok_or_else(|| {
            MapError::operational(format!(
                "Could not determine the input image filename from '{image_path}'"
            ))
        })?;
    Ok(RuntimeValues {
        image_file_name,
        mapping_timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    })
}

fn resolve_source(
    source: &SourceSpec,
    tags: &BTreeMap<u16, Value>,
    runtime: &RuntimeValues,
) -> Option<Value> {
    match source {
        SourceSpec::TiffTag { id } => tags.get(id).cloned(),
        SourceSpec::Constant { value } => Some(value.clone()),
        SourceSpec::Runtime { field } => Some(match field {
            RuntimeField::ImageFileName => json!(runtime.image_file_name),
            RuntimeField::MappingTimestamp => json!(runtime.mapping_timestamp),
        }),
    }
}

fn apply_transform(
    invocation: &TransformInvocation,
    primary_value: Value,
    tags: &BTreeMap<u16, Value>,
    runtime: &RuntimeValues,
    transform_directory: &Path,
) -> Result<Value, TransformFailure> {
    if invocation.id.is_empty()
        || invocation.id == "."
        || invocation.id == ".."
        || invocation.id.contains('/')
        || invocation.id.contains('\\')
    {
        return Err(TransformFailure::configuration(
            "Transform ID must be a non-empty file name without path separators",
        ));
    }

    let definition_path = transform_directory.join(format!("{}.json", invocation.id));
    let definition = load_json(definition_path.to_str().ok_or_else(|| {
        TransformFailure::configuration("Transform definition path is not valid UTF-8")
    })?)
    .map_err(|error| {
        TransformFailure::configuration(format!(
            "Could not load transform definition '{}': {error}",
            definition_path.display()
        ))
    })?;
    let declared_inputs = validate_transform_definition(&definition, &invocation.id)?;
    if !declared_inputs
        .iter()
        .any(|name| name == &invocation.primary_input)
    {
        return Err(TransformFailure::configuration(format!(
            "Transform invocation primary input '{}' is not declared by '{}'",
            invocation.primary_input, invocation.id
        )));
    }
    if let Some(name) = invocation
        .inputs
        .keys()
        .find(|name| !declared_inputs.iter().any(|declared| declared == *name))
    {
        return Err(TransformFailure::configuration(format!(
            "Transform input '{}' is not declared by '{}'",
            name, invocation.id
        )));
    }

    let mut inputs = BTreeMap::new();
    if invocation.inputs.contains_key(&invocation.primary_input) {
        return Err(TransformFailure::configuration(format!(
            "Transform invocation binds primary input '{}' twice",
            invocation.primary_input
        )));
    }
    inputs.insert(invocation.primary_input.clone(), primary_value);
    for (name, source) in &invocation.inputs {
        let Some(value) = resolve_source(source, tags, runtime) else {
            return Err(TransformFailure::value(format!(
                "Transform input '{name}' is missing"
            )));
        };
        inputs.insert(name.clone(), value);
    }

    match invocation.id.as_str() {
        "datetime-tiff-to-iso8601" => transform_datetime(&inputs, &invocation.parameters),
        "rational-to-float" => transform_rational(&inputs),
        "resolution-to-nm-per-px" => transform_resolution(&inputs),
        "photometric-to-color-mode" => transform_photometric(&inputs),
        _ => Err(TransformFailure::configuration(format!(
            "Transform '{}' is not implemented by famdo",
            invocation.id
        ))),
    }
}

fn validate_transform_definition(
    definition: &Value,
    expected_id: &str,
) -> Result<Vec<String>, TransformFailure> {
    let object = definition.as_object().ok_or_else(|| {
        TransformFailure::configuration("Transform definition must be a JSON object")
    })?;
    if object.get("id").and_then(Value::as_str) != Some(expected_id) {
        return Err(TransformFailure::configuration(format!(
            "Transform definition ID does not match '{}'",
            expected_id
        )));
    }
    let inputs = object
        .get("inputs")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            TransformFailure::configuration("Transform definition field 'inputs' must be an array")
        })?;
    let mut names = Vec::new();
    for input in inputs {
        let name = input
            .as_object()
            .and_then(|input| input.get("name"))
            .and_then(Value::as_str)
            .ok_or_else(|| {
                TransformFailure::configuration(
                    "Transform definition inputs must contain string names",
                )
            })?;
        if names.iter().any(|existing| existing == name) {
            return Err(TransformFailure::configuration(format!(
                "Transform definition declares input '{name}' more than once"
            )));
        }
        names.push(name.to_owned());
    }
    Ok(names)
}

fn transform_datetime(
    inputs: &BTreeMap<String, Value>,
    parameters: &Map<String, Value>,
) -> Result<Value, TransformFailure> {
    let value = inputs
        .get("value")
        .and_then(Value::as_str)
        .ok_or_else(|| TransformFailure::value("TIFF DateTime input must be a string"))?;
    let timestamp = NaiveDateTime::parse_from_str(value, "%Y:%m:%d %H:%M:%S")
        .map_err(|error| TransformFailure::value(format!("Invalid TIFF DateTime: {error}")))?;
    let timezone_offset = parameters
        .get("timezone_offset")
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| TransformFailure::value("timezone_offset must be a string"))
        })
        .transpose()?
        .unwrap_or("");
    if timezone_offset.is_empty() {
        return Err(TransformFailure::value(
            "TIFF DateTime has no explicit timezone_offset; mapping was skipped rather than guessing a timezone",
        ));
    }
    if !is_valid_timezone_offset(timezone_offset) {
        return Err(TransformFailure::value(format!(
            "Invalid timezone_offset '{timezone_offset}'"
        )));
    }

    Ok(json!(format!(
        "{}{}",
        timestamp.format("%Y-%m-%dT%H:%M:%S"),
        timezone_offset
    )))
}

fn transform_rational(inputs: &BTreeMap<String, Value>) -> Result<Value, TransformFailure> {
    let value = inputs
        .get("value")
        .ok_or_else(|| TransformFailure::value("Rational transform input 'value' is missing"))?;
    Ok(json!(parse_fraction(value)?))
}

fn transform_resolution(inputs: &BTreeMap<String, Value>) -> Result<Value, TransformFailure> {
    let resolution = inputs
        .get("resolution")
        .ok_or_else(|| TransformFailure::value("Resolution input is missing"))?;
    let resolution = parse_fraction(resolution)?;
    if resolution <= 0.0 {
        return Err(TransformFailure::value(
            "Resolution must be greater than zero",
        ));
    }
    let unit = inputs
        .get("resolution_unit")
        .and_then(Value::as_u64)
        .ok_or_else(|| TransformFailure::value("ResolutionUnit must be a non-negative integer"))?;
    let nm_per_pixel = match unit {
        2 => 25_400_000.0 / resolution,
        3 => 10_000_000.0 / resolution,
        1 => {
            return Err(TransformFailure::value(
                "ResolutionUnit=1 has no absolute unit and cannot be converted to nm/px",
            ));
        }
        _ => {
            return Err(TransformFailure::value(format!(
                "Unsupported TIFF ResolutionUnit value {unit}"
            )));
        }
    };

    Ok(json!(nm_per_pixel))
}

fn transform_photometric(inputs: &BTreeMap<String, Value>) -> Result<Value, TransformFailure> {
    let value = inputs.get("value").and_then(Value::as_u64).ok_or_else(|| {
        TransformFailure::value("PhotometricInterpretation must be a non-negative integer")
    })?;
    let mode = match value {
        0 | 1 => "Grayscale".to_owned(),
        2 => "RGB".to_owned(),
        3 => "Palette".to_owned(),
        6 => "YCbCr".to_owned(),
        other => other.to_string(),
    };
    Ok(Value::String(mode))
}

fn parse_fraction(value: &Value) -> Result<f64, TransformFailure> {
    let text = value.as_str().ok_or_else(|| {
        TransformFailure::value("Rational input must be a 'numerator/denominator' string")
    })?;
    let (numerator, denominator) = text.split_once('/').ok_or_else(|| {
        TransformFailure::value(format!(
            "Invalid rational '{text}'; expected 'numerator/denominator'"
        ))
    })?;
    let numerator = numerator.parse::<f64>().map_err(|_| {
        TransformFailure::value(format!("Invalid rational numerator '{numerator}'"))
    })?;
    let denominator = denominator.parse::<f64>().map_err(|_| {
        TransformFailure::value(format!("Invalid rational denominator '{denominator}'"))
    })?;
    if denominator == 0.0 {
        return Err(TransformFailure::value(
            "Rational denominator must not be zero",
        ));
    }
    let result = numerator / denominator;
    if !result.is_finite() {
        return Err(TransformFailure::value(
            "Rational result is not a finite number",
        ));
    }
    Ok(result)
}

fn is_valid_timezone_offset(offset: &str) -> bool {
    if offset == "Z" {
        return true;
    }
    let bytes = offset.as_bytes();
    if bytes.len() != 6 || !matches!(bytes[0], b'+' | b'-') || bytes[3] != b':' {
        return false;
    }
    let Ok(hours) = offset[1..3].parse::<u8>() else {
        return false;
    };
    let Ok(minutes) = offset[4..6].parse::<u8>() else {
        return false;
    };
    hours <= 23 && minutes <= 59
}

fn decode_pointer(pointer: &str) -> Result<Vec<String>, String> {
    if !pointer.starts_with('/') {
        return Err(format!(
            "target '{pointer}' is not an RFC 6901 JSON Pointer"
        ));
    }

    pointer
        .split('/')
        .skip(1)
        .map(decode_pointer_token)
        .collect()
}

fn decode_pointer_token(token: &str) -> Result<String, String> {
    let mut decoded = String::with_capacity(token.len());
    let mut characters = token.chars();
    while let Some(character) = characters.next() {
        if character != '~' {
            decoded.push(character);
            continue;
        }

        match characters.next() {
            Some('0') => decoded.push('~'),
            Some('1') => decoded.push('/'),
            Some(other) => {
                return Err(format!(
                    "target contains invalid JSON Pointer escape '~{other}'"
                ));
            }
            None => return Err("target ends with an incomplete JSON Pointer escape".to_owned()),
        }
    }
    Ok(decoded)
}

fn set_json_pointer(document: &mut Value, tokens: &[String], value: Value) -> Result<(), String> {
    if tokens.is_empty() {
        return Err("mapping targets the document root, which is not supported".to_owned());
    }
    set_json_pointer_tokens(document, tokens, value)
}

fn set_json_pointer_tokens(
    current: &mut Value,
    tokens: &[String],
    value: Value,
) -> Result<(), String> {
    let token = &tokens[0];
    let is_last = tokens.len() == 1;

    match current {
        Value::Object(object) => {
            if is_last {
                if object.contains_key(token) {
                    return Err(format!("target is written more than once at '/{token}'"));
                }
                object.insert(token.clone(), value);
                return Ok(());
            }

            if !object.contains_key(token) {
                object.insert(
                    token.clone(),
                    if next_token_is_array_index(&tokens[1]) {
                        Value::Array(Vec::new())
                    } else {
                        Value::Object(Map::new())
                    },
                );
            }
            let child = object
                .get_mut(token)
                .ok_or_else(|| format!("could not create object property '{token}'"))?;
            if child.is_null() {
                return Err(format!(
                    "cannot descend through null object property '{token}'"
                ));
            }
            set_json_pointer_tokens(child, &tokens[1..], value)
        }
        Value::Array(array) => {
            let index = parse_array_index(token)?;
            if index > array.len() {
                return Err(format!(
                    "array target index {index} is beyond the next writable position"
                ));
            }
            if index == array.len() {
                array.push(if is_last {
                    Value::Null
                } else if next_token_is_array_index(&tokens[1]) {
                    Value::Array(Vec::new())
                } else {
                    Value::Object(Map::new())
                });
            }
            if is_last {
                if !array[index].is_null() {
                    return Err(format!("array target index {index} is already written"));
                }
                array[index] = value;
                Ok(())
            } else {
                set_json_pointer_tokens(&mut array[index], &tokens[1..], value)
            }
        }
        _ => Err("target path conflicts with a scalar JSON value".to_owned()),
    }
}

fn next_token_is_array_index(token: &str) -> bool {
    token == "0" || (!token.starts_with('0') && token.parse::<usize>().is_ok())
}

fn parse_array_index(token: &str) -> Result<usize, String> {
    if !next_token_is_array_index(token) {
        return Err(format!(
            "array target token '{token}' is not a valid non-negative index"
        ));
    }
    token
        .parse::<usize>()
        .map_err(|_| format!("array target index '{token}' is too large"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_decode_pointer_escapes_tokens() {
        assert_eq!(
            decode_pointer("/General Section/Compressed Bits~1Pixel/value").unwrap(),
            vec![
                "General Section".to_owned(),
                "Compressed Bits/Pixel".to_owned(),
                "value".to_owned()
            ]
        );
        assert_eq!(
            decode_pointer("/raw~0name").unwrap(),
            vec!["raw~name".to_owned()]
        );
    }

    #[test]
    fn test_decode_pointer_rejects_invalid_escape() {
        assert!(decode_pointer("/invalid~2escape").is_err());
        assert!(decode_pointer("/incomplete~").is_err());
        assert!(decode_pointer("not-a-pointer").is_err());
    }

    #[test]
    fn test_set_json_pointer_creates_nested_objects() {
        let mut document = Value::Object(Map::new());
        let tokens = decode_pointer("/General Section/Image Width/Value").unwrap();
        set_json_pointer(&mut document, &tokens, json!(640)).unwrap();
        assert_eq!(
            document,
            json!({
                "General Section": {
                    "Image Width": {
                        "Value": 640
                    }
                }
            })
        );
    }

    #[test]
    fn test_set_json_pointer_creates_arrays() {
        let mut document = Value::Object(Map::new());
        let tokens = decode_pointer("/Data Evaluation/POI/0/Name").unwrap();
        set_json_pointer(&mut document, &tokens, json!("center")).unwrap();
        assert_eq!(
            document,
            json!({
                "Data Evaluation": {
                    "POI": [{"Name": "center"}]
                }
            })
        );
    }

    #[test]
    fn test_set_json_pointer_rejects_conflicts_and_duplicate_targets() {
        let mut document = Value::Object(Map::new());
        let first = decode_pointer("/a").unwrap();
        set_json_pointer(&mut document, &first, json!(1)).unwrap();
        let child = decode_pointer("/a/b").unwrap();
        assert!(set_json_pointer(&mut document, &child, json!(2)).is_err());
        assert!(set_json_pointer(&mut document, &first, json!(3)).is_err());
    }

    #[test]
    fn test_timezone_offset_validation() {
        assert!(is_valid_timezone_offset("Z"));
        assert!(is_valid_timezone_offset("+01:00"));
        assert!(is_valid_timezone_offset("-05:30"));
        assert!(!is_valid_timezone_offset(""));
        assert!(!is_valid_timezone_offset("+24:00"));
        assert!(!is_valid_timezone_offset("+01"));
    }

    #[test]
    fn test_transform_datetime_requires_explicit_timezone() {
        let mut inputs = BTreeMap::new();
        inputs.insert("value".to_owned(), json!("2025:12:09 14:30:00"));
        let error = transform_datetime(&inputs, &Map::new()).unwrap_err();
        assert!(!error.configuration);
        assert!(error.message.contains("timezone_offset"));
    }

    #[test]
    fn test_transform_datetime_converts_with_timezone() {
        let mut inputs = BTreeMap::new();
        inputs.insert("value".to_owned(), json!("2025:12:09 14:30:00"));
        let mut parameters = Map::new();
        parameters.insert("timezone_offset".to_owned(), json!("+01:00"));
        let output = transform_datetime(&inputs, &parameters).unwrap();
        assert_eq!(output, json!("2025-12-09T14:30:00+01:00"));
    }

    #[test]
    fn test_transform_rational_and_resolution() {
        let mut rational_inputs = BTreeMap::new();
        rational_inputs.insert("value".to_owned(), json!("96000/1000"));
        assert_eq!(transform_rational(&rational_inputs).unwrap(), json!(96.0));

        let mut resolution_inputs = BTreeMap::new();
        resolution_inputs.insert("resolution".to_owned(), json!("96000/1000"));
        resolution_inputs.insert("resolution_unit".to_owned(), json!(2));
        assert_eq!(
            transform_resolution(&resolution_inputs).unwrap(),
            json!(264583.3333333333)
        );
    }

    #[test]
    fn test_transform_resolution_rejects_unit_without_absolute_measurement() {
        let mut inputs = BTreeMap::new();
        inputs.insert("resolution".to_owned(), json!("1/1"));
        inputs.insert("resolution_unit".to_owned(), json!(1));
        let error = transform_resolution(&inputs).unwrap_err();
        assert!(error.message.contains("no absolute unit"));
    }

    #[test]
    fn test_transform_photometric_lookup_and_fallback() {
        let mut inputs = BTreeMap::new();
        inputs.insert("value".to_owned(), json!(1));
        assert_eq!(transform_photometric(&inputs).unwrap(), json!("Grayscale"));
        inputs.insert("value".to_owned(), json!(99));
        assert_eq!(transform_photometric(&inputs).unwrap(), json!("99"));
    }

    #[test]
    fn test_validate_connector_reports_schema_errors() {
        let schema = json!({
            "type": "object",
            "required": ["name"],
            "properties": {
                "name": {"type": "string"}
            },
            "additionalProperties": false
        });
        let error = validate_connector(&json!({"mappings": []}), &schema).unwrap_err();
        assert_eq!(error.exit_code(), 1);
        assert!(error.to_string().contains("connector-schema.json"));
    }

    #[test]
    fn test_transform_invocation_checks_definition_inputs() {
        let directory = tempdir().unwrap();
        let transforms = directory.path().join("transforms");
        fs::create_dir(&transforms).unwrap();
        fs::write(
            transforms.join("rational-to-float.json"),
            json!({
                "id": "rational-to-float",
                "inputs": [{"name": "value"}]
            })
            .to_string(),
        )
        .unwrap();

        let invocation = TransformInvocation {
            id: "rational-to-float".to_owned(),
            primary_input: "wrong".to_owned(),
            inputs: BTreeMap::new(),
            parameters: Map::new(),
        };
        let error = apply_transform(
            &invocation,
            json!("1/2"),
            &BTreeMap::new(),
            &RuntimeValues {
                image_file_name: "sample.tif".to_owned(),
                mapping_timestamp: "2026-08-13T08:00:00Z".to_owned(),
            },
            &transforms,
        )
        .unwrap_err();
        assert!(error.configuration);
        assert!(error.message.contains("not declared"));
    }

    #[test]
    fn test_parse_connector_sources_and_transform_forms() {
        let connector = json!({
            "targetSchemaVersion": "1.1",
            "mappings": [
                {
                    "source": {"type": "tiff-tag", "id": 256},
                    "target": "/General Section/Image Width/Value"
                },
                {
                    "source": {"type": "constant", "value": "px"},
                    "target": "/General Section/Image Width/Unit",
                    "transform": "rational-to-float"
                },
                {
                    "source": {"type": "tiff-tag", "id": 282},
                    "target": "/General Section/Pixel Width/Value",
                    "transform": {
                        "id": "resolution-to-nm-per-px",
                        "primaryInput": "resolution",
                        "inputs": {
                            "resolution_unit": {"type": "tiff-tag", "id": 296}
                        }
                    }
                },
                {
                    "source": {"type": "runtime", "field": "image-file-name"},
                    "target": "/General Section/File Name"
                },
                {
                    "source": {"type": "runtime", "field": "mapping-timestamp"},
                    "target": "/General Section/Time Stamp"
                }
            ]
        });
        let parsed = parse_connector(&connector).unwrap();
        assert_eq!(parsed.target_schema_version, "1.1");
        assert_eq!(parsed.mappings.len(), 5);
        assert!(matches!(
            parsed.mappings[0].source,
            SourceSpec::TiffTag { id: 256 }
        ));
        assert!(matches!(
            parsed.mappings[3].source,
            SourceSpec::Runtime {
                field: RuntimeField::ImageFileName
            }
        ));
        assert!(matches!(
            parsed.mappings[4].source,
            SourceSpec::Runtime {
                field: RuntimeField::MappingTimestamp
            }
        ));
        assert_eq!(
            parsed.mappings[2].transform.as_ref().unwrap().primary_input,
            "resolution"
        );
    }

    #[test]
    fn test_runtime_sources_resolve_generated_values() {
        let runtime = RuntimeValues {
            image_file_name: "sample.tiff".to_owned(),
            mapping_timestamp: "2026-08-13T08:00:00Z".to_owned(),
        };
        let tags = BTreeMap::new();
        assert_eq!(
            resolve_source(
                &SourceSpec::Runtime {
                    field: RuntimeField::ImageFileName
                },
                &tags,
                &runtime
            ),
            Some(json!("sample.tiff"))
        );
        assert_eq!(
            resolve_source(
                &SourceSpec::Runtime {
                    field: RuntimeField::MappingTimestamp
                },
                &tags,
                &runtime
            ),
            Some(json!("2026-08-13T08:00:00Z"))
        );
    }

    #[test]
    fn test_resolve_connector_schema_path_supports_mapping_directory_layout() {
        let directory = tempdir().unwrap();
        let mappings = directory.path().join("mappings");
        fs::create_dir(&mappings).unwrap();
        let schema_path = directory.path().join("connector-schema.json");
        fs::write(&schema_path, "{}").unwrap();
        let connector_path = mappings.join("connector.json");
        let connector = json!({
            "$schema": "./connector-schema.json"
        });
        let resolved = resolve_connector_schema_path(&connector_path, None, &connector).unwrap();
        assert_eq!(resolved, schema_path);
    }
}
