# Repository Guidelines

## Project Structure & Module Organization
`famdo` is a Rust CLI for FA Metadata Header workflows.

- `src/main.rs`: CLI entrypoint and command dispatch.
- `src/cli.rs`: `clap` argument and subcommand definitions.
- `src/commands/`: feature commands (`validate.rs`, `extract.rs`, `map.rs`).
- `src/schema.rs` and `src/utils.rs`: schema download/cache logic and shared helpers.
- `connectors/`: mapping connector JSON files (for example, `connectors/tiff_to_fam_v2_connector.json`).
- `testdata/`: local schema fixtures and sample TIFF data used by tests.

Keep new command logic in `src/commands/` and expose shared code through `src/lib.rs`.

## Build, Test, and Development Commands
- `cargo build --release --locked`: build production binaries (matches CI workflow).
- `cargo test --locked`: run unit tests across all modules.
- `cargo run -- validate <file.json> --version v2 --strict`: validate FAMH JSON.
- `cargo run -- extract <image.tif> -o extracted_metadata.json`: extract TIFF metadata.
- `cargo run -- map <image.tif> -c connectors/tiff_to_fam_v2_connector.json -o mapped_output.json`: map TIFF metadata to FAMH v2.
- `cargo fmt --all`: format code before committing.

## Coding Style & Naming Conventions
Follow standard Rust style (`rustfmt` defaults, 4-space indentation, trailing commas where idiomatic).

- Use `snake_case` for functions, variables, and module files.
- Use `PascalCase` for types and enums.
- Keep command behavior small and composable; move reusable logic into helper functions/modules.
- Prefer explicit error propagation (`Result<_, Box<dyn std::error::Error>>`) over panics in runtime paths.

## Testing Guidelines
Tests are module-local unit tests (`#[cfg(test)] mod tests`) in the same file as implementation.

- Name tests as `test_*` and keep them behavior-focused (see `src/commands/map.rs`).
- Reuse `testdata/` fixtures for schema and TIFF-related behavior.
- Run `cargo test --locked` before opening a PR.
- Add/adjust tests for every functional change in command output or schema handling.

## Commit & Pull Request Guidelines
Recent history favors short, imperative commit subjects (`fix schema validation for v1 and v2`) and occasional scoped prefixes (`3-map-command: ...`).

- Keep commit messages concise and action-oriented.
- In PRs, include: problem statement, implementation summary, and test evidence (`cargo test --locked` output).
- Link related issues when applicable, and include sample CLI invocations/output when command behavior changes.

## Security & Configuration Tips
Schema validation may download schema files on first run; use `--no-cache` to force refresh, otherwise rely on local cache for reproducibility. Avoid committing sensitive input data; keep only sanitized fixtures in `testdata/`.
