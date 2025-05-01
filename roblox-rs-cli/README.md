# roblox-rs-cli

Command-line interface for the roblox-rs compiler.

## Overview

`roblox-rs-cli` provides a user-friendly command-line interface for the `roblox-rs` compiler, allowing developers to compile Rust code to Luau for Roblox platform development.

## Installation

```bash
# From crates.io (not yet available)
cargo install roblox-rs-cli

# From source
git clone https://github.com/yourusername/roblox-rs.git
cd roblox-rs
cargo install --path roblox-rs-cli
```

## Usage

### Compile a Rust file

```bash
roblox-rs compile my_file.rs
```

### Create a new project

```bash
roblox-rs new my-game
```

### Compile an entire project

```bash
cd my-game
roblox-rs build
```

### Watch for changes and compile automatically

```bash
roblox-rs watch
```

## Configuration

`roblox-rs-cli` supports configuration via a `roblox-rs.toml` file at the root of your project:

```toml
# Example configuration
[build]
output_dir = "out"
optimize = true
native = true

[types]
strict = true
```

## Related

- `roblox-rs-core`: The core compiler library used by this CLI
- `roblox-rs`: The main project repository

## License

This project is licensed under the MIT License - see the LICENSE file for details. 