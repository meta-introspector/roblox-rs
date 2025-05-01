# roblox-rs-core

Core library for the roblox-rs compiler.

## Overview

`roblox-rs-core` is the engine behind the `roblox-rs` compiler, responsible for parsing Rust source code, transforming it to Luau AST, and generating optimized Luau code for the Roblox platform.

## Features

- Rust AST parsing via `syn`
- Type system mapping from Rust to Luau
- AST transformation pipeline
- Optimized Luau code generation
- Support for Roblox APIs and patterns

## Usage

While this library is primarily used by the `roblox-rs-cli` package, you can also use it directly in your Rust code:

```rust
use roblox_rs_core::{compile, CompileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rust_source = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    "#;
    
    let options = CompileOptions {
        optimize: true,
        target_dir: "out".into(),
        ..Default::default()
    };
    
    let luau_output = compile(rust_source, options)?;
    println!("Generated Luau: {}", luau_output);
    
    Ok(())
}
```

## Architecture

The compilation process occurs in several stages:

1. Parse Rust source into AST using `syn`
2. Analyze types and build a type map
3. Transform Rust AST into an intermediate representation
4. Generate Luau AST from the intermediate representation
5. Emit optimized Luau code

## API Documentation

Detailed API documentation is coming soon. For now, see the examples directory for sample usage.

## Related

- `roblox-rs-cli`: Command-line interface using this library
- `roblox-rs`: The main project repository

## License

This project is licensed under the MIT License - see the LICENSE file for details. 