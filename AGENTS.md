# Repository Guidelines

## Project Structure & Module Organization
`famdo` is a Rust CLI for FA Metadata Header workflows.

- `src/main.rs`: CLI entrypoint and command dispatch.
- `src/cli.rs`: `clap` argument and subcommand definitions.
- `src/commands/`: feature commands (`validate.rs`, `extract.rs`, `edit.rs`, `delete.rs`).
- `src/schema.rs` and `src/utils.rs`: schema download/cache logic and shared helpers.
- `testdata/`: local schema fixtures and sample TIFF data used by tests.

Keep new command logic in `src/commands/` and expose shared code through `src/lib.rs`.

## Build, Test, and Development Commands
- `cargo build --release --locked`: build production binaries (matches CI workflow).
- `cargo test --locked`: run unit tests across all modules.
- `cargo run -- validate <file.json> --version v2 --strict`: validate FAMH JSON.
- `cargo run -- extract <image.tif> -o extracted_metadata.json`: extract TIFF metadata.
- `cargo run -- edit <file.json> generalSection.fileName renamed.tif --version v2 -o metadata_edited.json`: edit a FAMH field.
- `cargo run -- delete <file.json> generalSection.fileName --version v2 -o metadata_deleted.json`: remove a FAMH field.
- `cargo fmt --all`: format code before committing.

## Coding Style & Naming Conventions
Follow standard Rust style (`rustfmt` defaults, 4-space indentation, trailing commas where idiomatic).

- Use `snake_case` for functions, variables, and module files.
- Use `PascalCase` for types and enums.
- Keep command behavior small and composable; move reusable logic into helper functions/modules.
- Prefer explicit error propagation (`Result<_, Box<dyn std::error::Error>>`) over panics in runtime paths.

## Testing Guidelines
Tests are module-local unit tests (`#[cfg(test)] mod tests`) in the same file as implementation.

- Name tests as `test_*` and keep them behavior-focused (see `src/commands/delete.rs`).
- Reuse `testdata/` fixtures for schema and TIFF-related behavior.
- Run `cargo test --locked` before opening a PR.
- Add/adjust tests for every functional change in command output or schema handling.

## Commit & Pull Request Guidelines
Recent history favors short, imperative commit subjects (`fix schema validation for v1 and v2`) and occasional scoped prefixes (`3-delete-command: ...`).

- Keep commit messages concise and action-oriented.
- In PRs, include: problem statement, implementation summary, and test evidence (`cargo test --locked` output).
- Link related issues when applicable, and include sample CLI invocations/output when command behavior changes.

## Security & Configuration Tips
Schema validation may download schema files on first run; use `--no-cache` to force refresh, otherwise rely on local cache for reproducibility. Avoid committing sensitive input data; keep only sanitized fixtures in `testdata/`.

## Coding guidelines
### 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

### 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

### 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

### 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]