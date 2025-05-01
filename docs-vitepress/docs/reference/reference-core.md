---
title: roblox-rs-core Reference
---

# roblox-rs-core Reference

The `roblox-rs-core` crate is the heart of the Rust-to-Luau compiler, handling parsing, analysis, transformation, and code generation.

## Compiler Pipeline

1. **Parsing**: Uses `syn` to parse Rust code into an AST.
2. **Analysis**: Collects type info, dependencies, and metadata.
3. **Transformation**: Converts Rust AST to an intermediate representation and then to Luau AST.
4. **Optimization**: Applies optimizations for performance and size.
5. **Code Generation**: Emits Luau code from the optimized AST.

## Main Modules
- `ast`: Rust and Luau AST definitions.
- `transform`: AST transformation logic.
- `optimize`: Code optimization passes.
- `codegen`: Luau code emitter.
- `types`: Type system and mapping.
- `analysis`: Type and dependency analysis.
- `compiler.rs`: Orchestrates the pipeline.

## Example Usage

```rust
let source = std::fs::read_to_string("main.rs")?;
let luau_code = roblox_rs_core::compile(&source)?;
println!("{}", luau_code);
```

## Best Practices
- Keep Rust code idiomatic for best translation.
- Use supported Rust features (see docs).
- Use the CLI for most workflows.

See [How Rust Code Becomes Luau](../explanation/rust-to-luau) for a deep dive. 