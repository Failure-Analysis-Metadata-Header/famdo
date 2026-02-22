# famh-model Playground Guide

This guide is for quickly exploring the whole `famh-model` crate API.

## Run the full playground example

```bash
cargo run -p famh-model --example famh_model_playground
```

Source file:

- `crates/famh-model/examples/famh_model_playground.rs`

The example demonstrates:

- Parsing v1 and v2 headers from JSON.
- Reading strongly typed nested values.
- Round-trip serialization back to JSON.
- Using `MetadataDocument<T>` with `_meta`.
- Preserving unknown top-level fields via `extra`.

## Script-like workflows for experimentation

Use one of these patterns:

1. `examples/` inside `crates/famh-model` (best for quick API experiments)

- Add `crates/famh-model/examples/my_experiment.rs`
- Run `cargo run -p famh-model --example my_experiment`

2. `src/bin/` in workspace root (best for reusable internal tools)

- Add `src/bin/my_tool.rs`
- Run `cargo run --bin my_tool`

Start in `examples/`, then move stable code into library modules.

## Minimal template for new examples

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse JSON with famh-model APIs
    // print fields
    // serialize and compare round-trip output
    Ok(())
}
```

## Useful commands while iterating

```bash
# Run one example
cargo run -p famh-model --example famh_model_playground

# Check only famh-model
cargo test -p famh-model --locked

# Format after edits
cargo fmt --all
```
