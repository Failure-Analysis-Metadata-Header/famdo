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
famdo validate <path-to-json> [--version <v1|v2>] [--no-cache] [--strict]
```

If a section fails validation, the command prints the first failing rule along
with the schema section name and exits with a non-zero status. With `--strict`,
validation also fails when required top-level sections are missing or unknown
top-level sections are present. Use `--no-cache` whenever you need to bypass the
on-disk schema cache and force a fresh download.

The first run of a new schema version requires internet access so that the CLI
can download and cache the respective JSON schema fragments. Subsequent runs
reuse the cached copy unless `--no-cache` is supplied.

### Metadata Extraction
Utility function to extract metadata from a TIFF file:

```bash
famdo extract <path-to-tiff> [--out <out-path>]
```

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
