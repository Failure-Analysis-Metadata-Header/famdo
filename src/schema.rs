use chrono::Utc;
use clap::ValueEnum;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use tokio::fs;

const SCHEMA_BASE_URL: &str =
    "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema";
const DEFAULT_SCHEMA_REVISION: &str = "master";
const CACHE_METADATA_FILE: &str = "metadata.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SchemaVersion {
    #[value(name = "v1")]
    V1,
    #[value(name = "v2", alias = "v2-draft")]
    V2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaSource {
    pub family: SchemaVersion,
    pub revision: String,
    pub source_url: String,
}

impl SchemaSource {
    pub fn for_version(version: SchemaVersion, revision: Option<&str>) -> Self {
        let revision = revision
            .filter(|revision| !revision.trim().is_empty())
            .unwrap_or(DEFAULT_SCHEMA_REVISION)
            .to_owned();
        let source_url = format!(
            "{}/{}/schema/{}",
            SCHEMA_BASE_URL,
            revision,
            version.folder()
        );
        Self {
            family: version,
            revision,
            source_url,
        }
    }

    fn cache_directory_name(&self) -> String {
        if self.revision == DEFAULT_SCHEMA_REVISION {
            return self.family.cache_dir_name().to_owned();
        }

        format!(
            "{}/revisions/{}",
            self.family.cache_dir_name(),
            sha256_hex(self.revision.as_bytes())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaFileMetadata {
    pub name: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaCacheMetadata {
    pub requested_family: String,
    pub source_url: String,
    pub resolved_revision: String,
    pub retrieved_at: String,
    pub files: Vec<SchemaFileMetadata>,
}

impl SchemaCacheMetadata {
    fn new(source: &SchemaSource, files: BTreeMap<String, String>) -> Self {
        Self {
            requested_family: source.family.label().to_owned(),
            source_url: source.source_url.clone(),
            resolved_revision: source.revision.clone(),
            retrieved_at: Utc::now().to_rfc3339(),
            files: files
                .into_iter()
                .map(|(name, sha256)| SchemaFileMetadata { name, sha256 })
                .collect(),
        }
    }

    fn empty(source: &SchemaSource) -> Self {
        Self::new(source, BTreeMap::new())
    }

    fn to_json(&self) -> Value {
        serde_json::json!({
            "requested_family": self.requested_family,
            "source_url": self.source_url,
            "resolved_revision": self.resolved_revision,
            "retrieved_at": self.retrieved_at,
            "files": self.files.iter().map(|file| serde_json::json!({
                "name": file.name,
                "sha256": file.sha256,
            })).collect::<Vec<_>>(),
        })
    }

    fn from_json(value: Value) -> Result<Self, Box<dyn std::error::Error>> {
        let object = value
            .as_object()
            .ok_or("Schema cache metadata must be a JSON object")?;
        let required_string = |name: &str| -> Result<String, Box<dyn std::error::Error>> {
            object
                .get(name)
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| format!("Schema cache metadata field '{name}' is missing").into())
        };
        let files = object
            .get("files")
            .and_then(Value::as_array)
            .ok_or("Schema cache metadata field 'files' is missing")?
            .iter()
            .map(|file| {
                let file = file
                    .as_object()
                    .ok_or("Schema cache file metadata must be a JSON object")?;
                let name = file
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or("Schema cache file metadata name is missing")?;
                let sha256 = file
                    .get("sha256")
                    .and_then(Value::as_str)
                    .ok_or("Schema cache file metadata SHA-256 is missing")?;
                Ok(SchemaFileMetadata {
                    name: name.to_owned(),
                    sha256: sha256.to_owned(),
                })
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

        Ok(Self {
            requested_family: required_string("requested_family")?,
            source_url: required_string("source_url")?,
            resolved_revision: required_string("resolved_revision")?,
            retrieved_at: required_string("retrieved_at")?,
            files,
        })
    }

    fn matches(&self, source: &SchemaSource) -> bool {
        self.requested_family == source.family.label()
            && self.source_url == source.source_url
            && self.resolved_revision == source.revision
    }

    pub fn render_text(&self, cache_directory: &Path) -> String {
        let mut output = format!(
            "Schema cache: {}\nFamily: {}\nSource: {}\nRevision: {}\nRetrieved: {}\n",
            cache_directory.display(),
            self.requested_family,
            self.source_url,
            self.resolved_revision,
            self.retrieved_at,
        );
        output.push_str("Files:\n");
        for file in &self.files {
            output.push_str(&format!("  {}  {}\n", file.name, file.sha256));
        }
        output
    }
}

pub struct SchemaCacheLoad {
    pub cache: SchemaCache,
    pub metadata: SchemaCacheMetadata,
    pub warnings: Vec<String>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

impl SchemaVersion {
    pub fn label(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "v1",
            SchemaVersion::V2 => "v2-draft",
        }
    }

    pub fn is_draft(&self) -> bool {
        matches!(self, SchemaVersion::V2)
    }

    fn folder(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "v1",
            SchemaVersion::V2 => "v2",
        }
    }

    fn cache_dir_name(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "v1",
            SchemaVersion::V2 => "v2",
        }
    }
}

// Trait for schema type operations
trait SchemaTypeTrait: Copy {
    fn file_name(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn url_for_source(&self, source: &SchemaSource) -> String {
        format!("{}/{}", source.source_url, self.file_name())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum V1SchemaType {
    General,
    Customer,
    Tool,
    Method,
    DataEvaluation,
    History,
}

impl SchemaTypeTrait for V1SchemaType {
    fn file_name(&self) -> &'static str {
        match self {
            V1SchemaType::General => "General Section.json",
            V1SchemaType::Customer => "Customer Section.json",
            V1SchemaType::Tool => "Tool Specific.json",
            V1SchemaType::Method => "Method Specific.json",
            V1SchemaType::DataEvaluation => "Data Evaluation.json",
            V1SchemaType::History => "History.json",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            V1SchemaType::General => "General Section",
            V1SchemaType::Customer => "Customer Specific",
            V1SchemaType::Tool => "Tool Specific",
            V1SchemaType::Method => "Method Specific",
            V1SchemaType::DataEvaluation => "Data Evaluation",
            V1SchemaType::History => "History",
        }
    }
}

impl V1SchemaType {
    pub fn all() -> &'static [V1SchemaType] {
        &[
            V1SchemaType::General,
            V1SchemaType::Customer,
            V1SchemaType::Tool,
            V1SchemaType::Method,
            V1SchemaType::DataEvaluation,
            V1SchemaType::History,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum V2SchemaType {
    General,
    Customer,
    Tool,
    Method,
    DataEvaluation,
    History,
}

impl SchemaTypeTrait for V2SchemaType {
    fn file_name(&self) -> &'static str {
        match self {
            V2SchemaType::General => "generalSection.json",
            V2SchemaType::Customer => "customerSection.json",
            V2SchemaType::Tool => "toolSpecific.json",
            V2SchemaType::Method => "methodSpecific.json",
            V2SchemaType::DataEvaluation => "dataEvaluation.json",
            V2SchemaType::History => "historySection.json",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            V2SchemaType::General => "general",
            V2SchemaType::Customer => "customer",
            V2SchemaType::Tool => "tool",
            V2SchemaType::Method => "method",
            V2SchemaType::DataEvaluation => "data evaluation",
            V2SchemaType::History => "history",
        }
    }
}

impl V2SchemaType {
    pub fn all() -> &'static [V2SchemaType] {
        &[
            V2SchemaType::General,
            V2SchemaType::Customer,
            V2SchemaType::Tool,
            V2SchemaType::Method,
            V2SchemaType::DataEvaluation,
            V2SchemaType::History,
        ]
    }
}

impl fmt::Display for V1SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl fmt::Display for V2SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

// V1-specific schema cache
pub struct V1SchemaCache {
    pub general: Value,
    pub customer: Value,
    pub tool: Value,
    pub method: Value,
    pub data_evaluation: Value,
    pub history: Value,
}

impl V1SchemaCache {
    async fn download_all(source: &SchemaSource) -> Result<Self, Box<dyn std::error::Error>> {
        let (general, customer, tool, method, data_evaluation, history) = tokio::join!(
            download_and_parse_schema(V1SchemaType::General, source),
            download_and_parse_schema(V1SchemaType::Customer, source),
            download_and_parse_schema(V1SchemaType::Tool, source),
            download_and_parse_schema(V1SchemaType::Method, source),
            download_and_parse_schema(V1SchemaType::DataEvaluation, source),
            download_and_parse_schema(V1SchemaType::History, source),
        );

        let cache = V1SchemaCache {
            general: general?,
            customer: customer?,
            tool: tool?,
            method: method?,
            data_evaluation: data_evaluation?,
            history: history?,
        };

        Ok(cache)
    }

    pub fn get(&self, schema_type: V1SchemaType) -> &Value {
        match schema_type {
            V1SchemaType::General => &self.general,
            V1SchemaType::Customer => &self.customer,
            V1SchemaType::Tool => &self.tool,
            V1SchemaType::Method => &self.method,
            V1SchemaType::DataEvaluation => &self.data_evaluation,
            V1SchemaType::History => &self.history,
        }
    }

    async fn load_from_cache(cache_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let general = load_schema_from_file(cache_dir, V1SchemaType::General).await?;
        let customer = load_schema_from_file(cache_dir, V1SchemaType::Customer).await?;
        let tool = load_schema_from_file(cache_dir, V1SchemaType::Tool).await?;
        let method = load_schema_from_file(cache_dir, V1SchemaType::Method).await?;
        let data_evaluation =
            load_schema_from_file(cache_dir, V1SchemaType::DataEvaluation).await?;
        let history = load_schema_from_file(cache_dir, V1SchemaType::History).await?;

        Ok(V1SchemaCache {
            general,
            customer,
            tool,
            method,
            data_evaluation,
            history,
        })
    }

    async fn save_to_cache(
        &self,
        cache_dir: &Path,
    ) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
        fs::create_dir_all(cache_dir).await?;
        let mut hashes = BTreeMap::new();
        hashes.insert(
            V1SchemaType::General.file_name().to_owned(),
            save_schema_to_file(cache_dir, V1SchemaType::General, &self.general).await?,
        );
        hashes.insert(
            V1SchemaType::Customer.file_name().to_owned(),
            save_schema_to_file(cache_dir, V1SchemaType::Customer, &self.customer).await?,
        );
        hashes.insert(
            V1SchemaType::Tool.file_name().to_owned(),
            save_schema_to_file(cache_dir, V1SchemaType::Tool, &self.tool).await?,
        );
        hashes.insert(
            V1SchemaType::Method.file_name().to_owned(),
            save_schema_to_file(cache_dir, V1SchemaType::Method, &self.method).await?,
        );
        hashes.insert(
            V1SchemaType::DataEvaluation.file_name().to_owned(),
            save_schema_to_file(
                cache_dir,
                V1SchemaType::DataEvaluation,
                &self.data_evaluation,
            )
            .await?,
        );
        hashes.insert(
            V1SchemaType::History.file_name().to_owned(),
            save_schema_to_file(cache_dir, V1SchemaType::History, &self.history).await?,
        );
        Ok(hashes)
    }
}

// V2-specific schema cache
pub struct V2SchemaCache {
    pub general: Value,
    pub customer: Value,
    pub tool: Value,
    pub method: Value,
    pub data_evaluation: Value,
    pub history: Value,
}

impl V2SchemaCache {
    async fn download_all(source: &SchemaSource) -> Result<Self, Box<dyn std::error::Error>> {
        let (general, customer, tool, method, data_evaluation, history) = tokio::join!(
            download_and_parse_schema(V2SchemaType::General, source),
            download_and_parse_schema(V2SchemaType::Customer, source),
            download_and_parse_schema(V2SchemaType::Tool, source),
            download_and_parse_schema(V2SchemaType::Method, source),
            download_and_parse_schema(V2SchemaType::DataEvaluation, source),
            download_and_parse_schema(V2SchemaType::History, source),
        );

        let cache = V2SchemaCache {
            general: general?,
            customer: customer?,
            tool: tool?,
            method: method?,
            data_evaluation: data_evaluation?,
            history: history?,
        };

        Ok(cache)
    }

    pub fn get(&self, schema_type: V2SchemaType) -> &Value {
        match schema_type {
            V2SchemaType::General => &self.general,
            V2SchemaType::Customer => &self.customer,
            V2SchemaType::Tool => &self.tool,
            V2SchemaType::Method => &self.method,
            V2SchemaType::DataEvaluation => &self.data_evaluation,
            V2SchemaType::History => &self.history,
        }
    }

    async fn load_from_cache(cache_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let general = load_schema_from_file(cache_dir, V2SchemaType::General).await?;
        let customer = load_schema_from_file(cache_dir, V2SchemaType::Customer).await?;
        let tool = load_schema_from_file(cache_dir, V2SchemaType::Tool).await?;
        let method = load_schema_from_file(cache_dir, V2SchemaType::Method).await?;
        let data_evaluation =
            load_schema_from_file(cache_dir, V2SchemaType::DataEvaluation).await?;
        let history = load_schema_from_file(cache_dir, V2SchemaType::History).await?;

        Ok(V2SchemaCache {
            general,
            customer,
            tool,
            method,
            data_evaluation,
            history,
        })
    }

    async fn save_to_cache(
        &self,
        cache_dir: &Path,
    ) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
        fs::create_dir_all(cache_dir).await?;
        let mut hashes = BTreeMap::new();
        hashes.insert(
            V2SchemaType::General.file_name().to_owned(),
            save_schema_to_file(cache_dir, V2SchemaType::General, &self.general).await?,
        );
        hashes.insert(
            V2SchemaType::Customer.file_name().to_owned(),
            save_schema_to_file(cache_dir, V2SchemaType::Customer, &self.customer).await?,
        );
        hashes.insert(
            V2SchemaType::Tool.file_name().to_owned(),
            save_schema_to_file(cache_dir, V2SchemaType::Tool, &self.tool).await?,
        );
        hashes.insert(
            V2SchemaType::Method.file_name().to_owned(),
            save_schema_to_file(cache_dir, V2SchemaType::Method, &self.method).await?,
        );
        hashes.insert(
            V2SchemaType::DataEvaluation.file_name().to_owned(),
            save_schema_to_file(
                cache_dir,
                V2SchemaType::DataEvaluation,
                &self.data_evaluation,
            )
            .await?,
        );
        hashes.insert(
            V2SchemaType::History.file_name().to_owned(),
            save_schema_to_file(cache_dir, V2SchemaType::History, &self.history).await?,
        );
        Ok(hashes)
    }
}

// Unified schema cache enum
pub enum SchemaCache {
    V1(V1SchemaCache),
    V2(V2SchemaCache),
}

#[derive(Debug, Clone, Copy)]
pub struct SectionDefinition<'a> {
    pub name: &'static str,
    pub schema: &'a Value,
    pub required: bool,
    pub extension: bool,
    pub aliases: &'static [&'static str],
}

impl SchemaCache {
    pub async fn download_all(
        version: SchemaVersion,
        use_cache: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::download_all_with_source(version, use_cache, None)
            .await?
            .cache)
    }

    pub async fn download_all_with_source(
        version: SchemaVersion,
        use_cache: bool,
        revision: Option<&str>,
    ) -> Result<SchemaCacheLoad, Box<dyn std::error::Error>> {
        let source = SchemaSource::for_version(version, revision);
        let cache_dir = get_cache_dir(&source)?;
        let mut warnings = Vec::new();

        if use_cache {
            match load_cache_metadata(&cache_dir).await {
                Ok(metadata) if metadata.matches(&source) => {
                    match verify_cached_files(&cache_dir, &metadata).await {
                        Ok(()) => {
                            let cache = match version {
                                SchemaVersion::V1 => {
                                    SchemaCache::V1(V1SchemaCache::load_from_cache(&cache_dir).await?)
                                }
                                SchemaVersion::V2 => {
                                    SchemaCache::V2(V2SchemaCache::load_from_cache(&cache_dir).await?)
                                }
                            };
                            return Ok(SchemaCacheLoad {
                                cache,
                                metadata,
                                warnings,
                            });
                        }
                        Err(error) => warnings.push(format!(
                            "Schema cache verification failed; downloaded fresh schemas: {error}"
                        )),
                    }
                }
                Ok(_) => warnings.push(
                    "Schema cache metadata does not match the requested schema source; downloaded fresh schemas."
                        .to_owned(),
                ),
                Err(error) if cache_dir.exists() => warnings.push(format!(
                    "Schema cache metadata could not be read; downloaded fresh schemas: {error}"
                )),
                Err(_) => {}
            }
        }

        let cache = match version {
            SchemaVersion::V1 => SchemaCache::V1(V1SchemaCache::download_all(&source).await?),
            SchemaVersion::V2 => SchemaCache::V2(V2SchemaCache::download_all(&source).await?),
        };

        let metadata = match save_cache(&cache, &cache_dir, &source).await {
            Ok(metadata) => metadata,
            Err(error) => {
                warnings.push(format!(
                    "Could not write schema cache at {}: {error}",
                    cache_dir.display()
                ));
                SchemaCacheMetadata::empty(&source)
            }
        };

        Ok(SchemaCacheLoad {
            cache,
            metadata,
            warnings,
        })
    }

    pub async fn inspect_cache(
        version: SchemaVersion,
        revision: Option<&str>,
    ) -> Result<Option<SchemaCacheMetadata>, Box<dyn std::error::Error>> {
        let source = SchemaSource::for_version(version, revision);
        let cache_dir = get_cache_dir(&source)?;
        match load_cache_metadata(&cache_dir).await {
            Ok(metadata) => Ok(Some(metadata)),
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    pub fn cache_directory(
        version: SchemaVersion,
        revision: Option<&str>,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        get_cache_dir(&SchemaSource::for_version(version, revision))
    }

    // Access schemas by field name (works across versions)
    pub fn general(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.general,
            SchemaCache::V2(cache) => &cache.general,
        }
    }

    pub fn customer(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.customer,
            SchemaCache::V2(cache) => &cache.customer,
        }
    }

    pub fn tool(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.tool,
            SchemaCache::V2(cache) => &cache.tool,
        }
    }

    pub fn method(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.method,
            SchemaCache::V2(cache) => &cache.method,
        }
    }

    pub fn data_evaluation(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.data_evaluation,
            SchemaCache::V2(cache) => &cache.data_evaluation,
        }
    }

    pub fn history(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.history,
            SchemaCache::V2(cache) => &cache.history,
        }
    }

    pub fn all_sections(&self) -> Vec<(&'static str, &Value)> {
        self.section_definitions()
            .into_iter()
            .map(|section| (section.name, section.schema))
            .collect()
    }

    pub fn section_definitions(&self) -> Vec<SectionDefinition<'_>> {
        match self {
            SchemaCache::V1(_) => vec![
                SectionDefinition {
                    name: "General Section",
                    schema: self.general(),
                    required: true,
                    extension: false,
                    aliases: &["generalSection"],
                },
                SectionDefinition {
                    name: "Customer Specific",
                    schema: self.customer(),
                    required: false,
                    extension: true,
                    aliases: &["Customer Section", "customerSpecific"],
                },
                SectionDefinition {
                    name: "Tool Specific",
                    schema: self.tool(),
                    required: false,
                    extension: true,
                    aliases: &["Tool Section"],
                },
                SectionDefinition {
                    name: "Method Specific",
                    schema: self.method(),
                    required: true,
                    extension: false,
                    aliases: &["Method Section", "methodSpecific"],
                },
                SectionDefinition {
                    name: "Data Evaluation",
                    schema: self.data_evaluation(),
                    required: false,
                    extension: false,
                    aliases: &["dataEvaluation"],
                },
                SectionDefinition {
                    name: "History",
                    schema: self.history(),
                    required: false,
                    extension: true,
                    aliases: &["history"],
                },
            ],
            SchemaCache::V2(_) => vec![
                SectionDefinition {
                    name: "generalSection",
                    schema: self.general(),
                    required: true,
                    extension: false,
                    aliases: &["General Section", "general_section"],
                },
                SectionDefinition {
                    name: "customerSpecific",
                    schema: self.customer(),
                    required: false,
                    extension: true,
                    aliases: &["Customer Specific", "Customer Section"],
                },
                SectionDefinition {
                    name: "toolSpecific",
                    schema: self.tool(),
                    required: false,
                    extension: true,
                    aliases: &["Tool Specific", "Tool Section"],
                },
                SectionDefinition {
                    name: "methodSpecific",
                    schema: self.method(),
                    required: true,
                    extension: false,
                    aliases: &["Method Specific", "Method Section", "method_section"],
                },
                SectionDefinition {
                    name: "dataEvaluation",
                    schema: self.data_evaluation(),
                    required: false,
                    extension: false,
                    aliases: &["Data Evaluation"],
                },
                SectionDefinition {
                    name: "history",
                    schema: self.history(),
                    required: false,
                    extension: true,
                    aliases: &["History"],
                },
            ],
        }
    }

    pub fn required_sections(&self) -> &'static [&'static str] {
        match self {
            SchemaCache::V1(_) => &["General Section", "Method Specific"],
            SchemaCache::V2(_) => &["generalSection", "methodSpecific"],
        }
    }

    pub fn schema_source(&self) -> String {
        let version = match self {
            SchemaCache::V1(_) => SchemaVersion::V1,
            SchemaCache::V2(_) => SchemaVersion::V2,
        };
        SchemaSource::for_version(version, None).source_url
    }

    pub fn version_label(&self) -> &'static str {
        match self {
            SchemaCache::V1(_) => SchemaVersion::V1.label(),
            SchemaCache::V2(_) => SchemaVersion::V2.label(),
        }
    }
}

async fn download_schema<T: SchemaTypeTrait>(
    schema_type: T,
    source: &SchemaSource,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = schema_type.url_for_source(source);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.map_err(|e| {
        format!(
            "Failed to download {} schema from {}: {}",
            schema_type.label(),
            url,
            e
        )
    })?;
    let response = response.error_for_status().map_err(|e| {
        format!(
            "{} schema endpoint {} returned an error response: {}",
            schema_type.label(),
            url,
            e
        )
    })?;
    response.text().await.map_err(|e| {
        format!(
            "Failed to read {} schema body from {}: {}",
            schema_type.label(),
            url,
            e
        )
        .into()
    })
}

fn parse_schema_from_source<T: SchemaTypeTrait>(
    schema_type: T,
    schema_text: &str,
    source: &SchemaSource,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = schema_type.url_for_source(source);
    serde_json::from_str(schema_text).map_err(|e| {
        format!(
            "Failed to parse {} schema downloaded from {}: {}",
            schema_type.label(),
            url,
            e
        )
        .into()
    })
}

async fn download_and_parse_schema<T: SchemaTypeTrait>(
    schema_type: T,
    source: &SchemaSource,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let schema_text = download_schema(schema_type, source).await?;
    let schema = parse_schema_from_source(schema_type, &schema_text, source)?;
    Ok(schema)
}

fn get_cache_dir(source: &SchemaSource) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cache_dir = dirs::cache_dir()
        .ok_or("Could not determine cache directory")?
        .join("famdo")
        .join("schemas")
        .join(source.cache_directory_name());
    Ok(cache_dir)
}

async fn load_cache_metadata(
    cache_dir: &Path,
) -> Result<SchemaCacheMetadata, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(cache_dir.join(CACHE_METADATA_FILE)).await?;
    SchemaCacheMetadata::from_json(serde_json::from_str(&content)?)
}

async fn verify_cached_files(
    cache_dir: &Path,
    metadata: &SchemaCacheMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    if metadata.files.len() != 6 {
        return Err("Schema cache metadata does not list all schema files".into());
    }

    for file in &metadata.files {
        let content = fs::read(cache_dir.join(&file.name)).await?;
        let actual_hash = sha256_hex(&content);
        if actual_hash != file.sha256 {
            return Err(format!("SHA-256 mismatch for cached schema '{}'", file.name).into());
        }
    }
    Ok(())
}

async fn save_cache(
    cache: &SchemaCache,
    cache_dir: &Path,
    source: &SchemaSource,
) -> Result<SchemaCacheMetadata, Box<dyn std::error::Error>> {
    let hashes = match cache {
        SchemaCache::V1(cache) => cache.save_to_cache(cache_dir).await?,
        SchemaCache::V2(cache) => cache.save_to_cache(cache_dir).await?,
    };
    let metadata = SchemaCacheMetadata::new(source, hashes);
    fs::write(
        cache_dir.join(CACHE_METADATA_FILE),
        serde_json::to_vec_pretty(&metadata.to_json())?,
    )
    .await?;
    Ok(metadata)
}

// Load a single schema from cache file
async fn load_schema_from_file<T: SchemaTypeTrait>(
    cache_dir: &Path,
    schema_type: T,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file_path = cache_dir.join(schema_type.file_name());
    let content = fs::read_to_string(&file_path).await?;
    let schema = serde_json::from_str(&content)?;
    Ok(schema)
}

// Save a single schema to cache file
async fn save_schema_to_file<T: SchemaTypeTrait>(
    cache_dir: &Path,
    schema_type: T,
    schema: &Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let file_path = cache_dir.join(schema_type.file_name());
    let content = serde_json::to_vec(schema)?;
    fs::write(&file_path, &content).await?;
    Ok(sha256_hex(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version_folder() {
        assert_eq!(SchemaVersion::V1.folder(), "v1");
        assert_eq!(SchemaVersion::V2.folder(), "v2");
    }

    #[test]
    fn test_v1_schema_type_file_name() {
        assert_eq!(V1SchemaType::General.file_name(), "General Section.json");
        assert_eq!(V1SchemaType::Customer.file_name(), "Customer Section.json");
        assert_eq!(V1SchemaType::Tool.file_name(), "Tool Specific.json");
        assert_eq!(V1SchemaType::Method.file_name(), "Method Specific.json");
        assert_eq!(
            V1SchemaType::DataEvaluation.file_name(),
            "Data Evaluation.json"
        );
        assert_eq!(V1SchemaType::History.file_name(), "History.json");
    }

    #[test]
    fn test_v2_schema_type_file_name() {
        assert_eq!(V2SchemaType::General.file_name(), "generalSection.json");
        assert_eq!(V2SchemaType::Customer.file_name(), "customerSection.json");
        assert_eq!(V2SchemaType::Tool.file_name(), "toolSpecific.json");
        assert_eq!(V2SchemaType::Method.file_name(), "methodSpecific.json");
        assert_eq!(
            V2SchemaType::DataEvaluation.file_name(),
            "dataEvaluation.json"
        );
        assert_eq!(V2SchemaType::History.file_name(), "historySection.json");
    }

    #[test]
    fn test_v1_schema_type_label() {
        assert_eq!(V1SchemaType::General.label(), "General Section");
        assert_eq!(V1SchemaType::Customer.label(), "Customer Specific");
        assert_eq!(V1SchemaType::Tool.label(), "Tool Specific");
        assert_eq!(V1SchemaType::Method.label(), "Method Specific");
        assert_eq!(V1SchemaType::DataEvaluation.label(), "Data Evaluation");
        assert_eq!(V1SchemaType::History.label(), "History");
    }

    #[test]
    fn test_v2_schema_type_label() {
        assert_eq!(V2SchemaType::General.label(), "general");
        assert_eq!(V2SchemaType::Customer.label(), "customer");
        assert_eq!(V2SchemaType::Tool.label(), "tool");
        assert_eq!(V2SchemaType::Method.label(), "method");
        assert_eq!(V2SchemaType::DataEvaluation.label(), "data evaluation");
        assert_eq!(V2SchemaType::History.label(), "history");
    }

    #[test]
    fn test_schema_type_url_for() {
        let source = SchemaSource::for_version(SchemaVersion::V2, None);
        let url = V2SchemaType::General.url_for_source(&source);
        assert!(url.contains("master"));
        assert!(url.contains("v2"));
        assert!(url.contains("generalSection.json"));
        assert!(url.starts_with(SCHEMA_BASE_URL));
    }

    #[test]
    fn test_schema_source_supports_default_and_pinned_revisions() {
        let default_source = SchemaSource::for_version(SchemaVersion::V1, None);
        assert_eq!(default_source.revision, "master");
        assert!(default_source.source_url.ends_with("/master/schema/v1"));

        let pinned_source = SchemaSource::for_version(SchemaVersion::V2, Some("abc123"));
        assert_eq!(pinned_source.revision, "abc123");
        assert!(pinned_source.source_url.ends_with("/abc123/schema/v2"));
        assert_ne!(
            default_source.cache_directory_name(),
            pinned_source.cache_directory_name()
        );
    }

    #[test]
    fn test_schema_cache_metadata_round_trips_and_hashes() {
        let source = SchemaSource::for_version(SchemaVersion::V2, Some("abc123"));
        let mut files = BTreeMap::new();
        files.insert("generalSection.json".to_owned(), sha256_hex(b"schema"));
        let metadata = SchemaCacheMetadata::new(&source, files);
        let restored = SchemaCacheMetadata::from_json(metadata.to_json()).unwrap();

        assert_eq!(restored, metadata);
        assert_eq!(restored.requested_family, "v2-draft");
        assert_eq!(restored.resolved_revision, "abc123");
        assert_eq!(restored.files[0].sha256.len(), 64);
    }

    #[test]
    fn test_parse_schema_valid() {
        let schema_text = r#"{"$schema": "http://json-schema.org/draft-07/schema#"}"#;
        let source = SchemaSource::for_version(SchemaVersion::V2, None);
        let result = parse_schema_from_source(V2SchemaType::General, schema_text, &source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_schema_invalid() {
        let schema_text = "not valid json";
        let source = SchemaSource::for_version(SchemaVersion::V2, None);
        let result = parse_schema_from_source(V2SchemaType::General, schema_text, &source);
        assert!(result.is_err());
    }

    #[test]
    fn test_v1_schema_cache_get() {
        let cache = V1SchemaCache {
            general: serde_json::json!({"type": "general"}),
            customer: serde_json::json!({"type": "customer"}),
            tool: serde_json::json!({"type": "tool"}),
            method: serde_json::json!({"type": "method"}),
            data_evaluation: serde_json::json!({"type": "data_evaluation"}),
            history: serde_json::json!({"type": "history"}),
        };

        assert_eq!(
            cache.get(V1SchemaType::General),
            &serde_json::json!({"type": "general"})
        );
        assert_eq!(
            cache.get(V1SchemaType::Customer),
            &serde_json::json!({"type": "customer"})
        );
    }

    #[test]
    fn test_unified_schema_cache_accessors() {
        let v1_cache = V1SchemaCache {
            general: serde_json::json!({"type": "general"}),
            customer: serde_json::json!({"type": "customer"}),
            tool: serde_json::json!({"type": "tool"}),
            method: serde_json::json!({"type": "method"}),
            data_evaluation: serde_json::json!({"type": "data_evaluation"}),
            history: serde_json::json!({"type": "history"}),
        };

        let cache = SchemaCache::V1(v1_cache);
        assert_eq!(cache.general(), &serde_json::json!({"type": "general"}));
        assert_eq!(cache.customer(), &serde_json::json!({"type": "customer"}));
    }
}
