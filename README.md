# famdo - FA Metadata Organizer

`famdo` is a small CLI tool for managing the Failure Analysis (FA) Metadata Header format.
Currently supported functionality:
- validate JSON file against FAMH Schema v1 or v2
- extract metadata from TIFF file and save as JSON
- map extracted metadata to FAMH v2 format using connector configurations

The FAMH schema is downloaded directly from the
`fa-metadata-schema` repository and cached locally for faster re-use.

## Installation

Grab the latest binary for your platform from the
[GitHub Releases](https://github.com/Failure-Analysis-Metadata-Header/famdo/releases)
page and place it somewhere on your `PATH` (or keep it in your project folder).
On Linux/macOS remember to make it executable:

```bash
chmod +x famdo
```

The first run of a new schema version requires internet access so that the CLI
can download and cache the respective JSON schema fragments. Subsequent runs
reuse the cached copy unless `--no-cache` is supplied.

## Usage

### Schema Validation

```bash
famdo validate <path-to-json> [--version <v1|v2>] [--no-cache]
```

If a section fails validation, the command prints the first failing rule along
with the schema section name and exits with a non-zero status. Use `--no-cache`
whenever you need to bypass the on-disk schema cache and force a fresh download.

### Metadata Extraction
Utility function to extract metadata from a TIFF file:

```bash
famdo extract <path-to-tiff> [--out <out-path>]
```

### Metadata Mapping
Map extracted TIFF metadata to FAMH v2 format using a connector configuration:

```bash
famdo map <path-to-tiff> --connector <path-to-connector-json>
```

The connector JSON defines how to transform raw TIFF metadata into valid FAMH v2
structure. A default TIFF connector is included at `connectors/tiff_to_fam_v2_connector.json`.

**Example:**
```bash
# Map TIFF metadata to FAMH v2 and save to file
famdo map image.tiff -c connectors/tiff_to_fam_v2_connector.json -o output.json

# Validate the mapped output
famdo validate output.json
```
