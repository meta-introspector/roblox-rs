---
title: How to Compile Rust Code to Luau
sidebar_label: Compile Rust to Luau
---

# How to Compile Rust Code to Luau

This guide shows you how to compile Rust code to Luau code for use in Roblox games.

## Prerequisites

- The roblox-rs-cli installed (see [Getting Started](../tutorials/getting-started.md))
- A Rust project with code you want to compile

## Compiling a Single File

To compile a single Rust file to Luau:

```bash
roblox-rs compile path/to/your/file.rs
```

By default, this will generate a `.lua` file with the same name in the same directory.

### Specifying an Output File

You can specify a different output file:

```bash
roblox-rs compile input.rs --output output.lua
```

### Setting Optimization Levels

Control the optimization level with the `--optimization` flag:

```bash
roblox-rs compile input.rs --optimization aggressive
```

Available optimization levels:
- `minimal`: Basic optimizations only
- `default`: Standard optimizations (the default)
- `aggressive`: Maximum optimizations (may increase compile time)

### Including Debug Information

Include debug information in the compiled output:

```bash
roblox-rs compile input.rs --debug
```

## Compiling an Entire Project

For a full project, use the build command:

```bash
roblox-rs build
```

This command:
1. Reads your project configuration
2. Compiles all Rust source files
3. Generates Luau output files in the configured output directory (default: `out/`)

### Using a Custom Configuration

You can specify a custom configuration file:

```bash
roblox-rs build --config my-config.toml
```

## Using the Watch Mode

For rapid development, use watch mode to automatically recompile when files change:

```bash
roblox-rs watch
```

This will start a file watcher and recompile your project whenever source files are modified.

## Customizing the Build Process

Create a `roblox-rs.toml` file in your project root to customize the build process:

```toml
[build]
output_dir = "game/scripts"  # Output directory for Luau files
include_runtime = true       # Include roblox-rs runtime in output
optimize = true              # Enable optimizations
debug_mode = false           # Include debug information

[compiler]
target = "roblox"            # Target platform (roblox or native)
```

## Troubleshooting Common Issues

### Syntax Errors

If you get syntax errors during compilation:

1. Check that your Rust code is valid
2. Ensure you're using features supported by roblox-rs
3. Check the error message for specific line numbers and issues

### Missing Dependencies

If you get "missing dependency" errors:

1. Make sure all dependencies are listed in your `Cargo.toml`
2. Check that dependencies are compatible with roblox-rs
3. Run `cargo check` to verify your dependencies can be resolved

### Runtime Errors

If your compiled code runs into errors in Roblox:

1. Try enabling debug mode (`--debug` flag) for more detailed error information
2. Check for Rust features that may not translate well to Luau
3. Verify that any platform-specific code has proper conditionals

## Examples

### Example 1: Simple Function

Rust code (`math.rs`):
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

Compile with:
```bash
roblox-rs compile math.rs
```

### Example 2: Project with Dependencies

`Cargo.toml`:
```toml
[package]
name = "my-game"
version = "0.1.0"

[dependencies]
roblox-rs-ecs = "0.1.0"
```

`src/main.rs`:
```rust
use roblox_rs_ecs::prelude::*;

// Your game code...
```

Compile with:
```bash
roblox-rs build
```

## Next Steps

- Learn how to [Debug Compiled Code](./debugging-compiled-code.md)
- Explore [Advanced Compiler Options](./advanced-compiler-options.md)
- See how to [Optimize Performance](./optimize-performance.md) of your compiled code 