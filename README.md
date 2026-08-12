# famdo - FA Metadata Organizer

`famdo` is a small CLI tool for managing the Failure AnalysisMetadata Header (FAMH) format.

Note: `famdo` is still in early development. Bug are expected - please report them as issue!

Currently, the primarily supported functionality is **FAMH schema validation**.

Other functionality that is already partially implemented or in POC state:
- extract metadata from TIFF file and save as JSON
- edit a field in a FAMH v1 or v2 JSON document
- delete a field from a FAMH v1 or v2 JSON document

The repository also includes a reusable model crate:
- `crates/famh-model`: typed Rust structs for FA metadata (`v1` and `v2`) with serde helpers.

The FAMH schema is downloaded directly from the
`fa-metadata-schema` repository and cached locally for faster re-use.

## Installation

Grab the latest binary for your platform from the
[GitHub Releases](https://github.com/Failure-Analysis-Metadata-Header/famdo/releases)
page and place it somewhere on your `PATH` (or keep it in your project folder).
On Linux/macOS remember to make it executable:

## Usage

### Schema Validation

```bash
famdo validate <path-to-json> [--version <v1|v2|v2-draft>] [--no-cache] [--strict]
					 [--format <text|json>] [--fail-on <error|warning>]
```

The command reports the selected schema family, every schema error, missing
required sections, absent optional sections, and unexpected root-level sections.
`v1` is the current stable schema line. `v2-draft` is an experimental draft and
is available as the `v2` compatibility name or the explicit `v2-draft` alias.
With `--strict`, unexpected root-level sections are errors instead of warnings.
Use `--no-cache` whenever you need to bypass the on-disk schema cache and force a
fresh download.

Validation exits with status `0` when no configured findings fail the threshold,
`1` when metadata has validation errors (or warnings with `--fail-on warning`),
and `2` when an operational failure prevents validation, such as a missing input,
invalid JSON, or unavailable schemas. `--format text` is the default. The JSON
format is a stable report containing the tool version, schema family and source,
overall schema result, severity counts, section summaries, and all findings.

The first run of a new schema version requires internet access so that the CLI
can download and cache the respective JSON schema fragments. Subsequent runs
reuse the cached copy unless `--no-cache` is supplied.

Schema sources default to the `master` revision. Pin a branch, tag, or commit
with `--revision <revision>` when validating or refreshing a cache. Cache
metadata records the requested family, source URL, revision, retrieval time,
and SHA-256 for every cached schema file:

```bash
famdo cache inspect [--version <v1|v2|v2-draft>] [--revision <revision>]
famdo cache refresh [--version <v1|v2|v2-draft>] [--revision <revision>]
```

`cache refresh` forces a download. `cache inspect` displays the metadata for a
matching source, and reports a missing cache with exit status `2`. If a cache
cannot be written during validation, famdo continues the validation and emits a
`WARNING` finding describing the cache problem.

### Metadata Extraction
Utility function to extract metadata from a TIFF file:

```bash
famdo extract <path-to-tiff> [--out <out-path>]
```

The extracted JSON reports each TIFF tag with its numeric TIFF `id`, human-readable
`tag` name, raw `value`, and TIFF value `type`. The numeric `id` is the stable
cross-tool identifier and should be preferred when building connector mappings.

### Metadata Editing
Update a single field in an existing FAMH JSON document:

```bash
famdo edit <path-to-json> <field> <value> [--version <v1|v2>] [--out <out-path>]
```

`<field>` supports dot notation (`generalSection.datasetName`) or JSON Pointer
style (`/generalSection/datasetName`). `<value>` is parsed as JSON when possible
(for example `42`, `true`, or `{"k":"v"}`), otherwise it is written as a string.

### Metadata Deletion
Remove a single field from an existing FAMH JSON document:

```bash
famdo delete <path-to-json> <field> [--version <v1|v2>] [--out <out-path>]
```

`delete` uses the same field syntax as `edit`. Dot notation automatically
escapes JSON Pointer special characters inside field names, while explicit JSON
Pointer input must already use RFC 6901 escaping (for example `~1` for `/`).
By default the updated document is written to `metadata_deleted.json`; use
`--out <same-path>` if you explicitly want an in-place replacement.

## Using the model crate

Other Rust projects can consume typed FA metadata models from `famh-model`:

```toml
[dependencies]
famh-model = { git = "https://github.com/Failure-Analysis-Metadata-Header/famdo.git" }
```

```rust
use famh_model::v2::FaMetadataHeader;

let model = FaMetadataHeader::from_str(r#"{"generalSection":{},"methodSpecific":{}}"#)?;
let normalized = model.to_string_pretty()?;
```
