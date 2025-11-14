# famdo - FA Metadata Organizer

`famdo` is a small CLI tool for managing the Failure Analysis (FA) Metadata Header format.
Currently, the tool can validate any FA Metadata Header against the public schema. It can validate both v1 and v2
schemas by downloading the canonical schema fragments directly from the
`fa-metadata-schema` repository and caching them locally for faster re-use.

## Download

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

```bash
famdo validate <path-to-json> [--version <v1|v2>] [--no-cache]
```

If a section fails validation, the command prints the first failing rule along
with the schema section name and exits with a non-zero status. Use `--no-cache`
whenever you need to bypass the on-disk schema cache and force a fresh download.
