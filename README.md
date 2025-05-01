# roblox-rs

A compiler that translates Rust code to optimized Luau for the Roblox platform.

## Overview

`roblox-rs` enables developers to write Roblox games and plugins using Rust, compiling to highly optimized Luau code. This project is inspired by the success of [roblox-ts](https://roblox-ts.com/), but for the Rust programming language.

## Features

- Direct compilation from Rust source to optimized Luau
- Preservation of Rust's rich type system where possible
- Support for Roblox's Actor model for parallelism
- Async/await implementation compatible with Roblox's task library
- Optimized output with `--!optimize 2` and `--!native` annotations
- Seamless integration with Roblox APIs

## Project Structure

- `roblox-rs-cli`: Command-line interface for the compiler
- `roblox-rs-core`: Core compilation and transformation logic
- `roblox-rs-ecs`: Bevy-inspired Entity Component System for game development
- `roblox-rs-rojo`: Integration with Rojo project management
- (Future) Standard library for Roblox API bindings

## Installation

```bash
# Install from crates.io (not yet available)
cargo install roblox-rs-cli

# Or build from source
git clone https://github.com/yourusername/roblox-rs.git
cd roblox-rs
cargo install --path roblox-rs-cli
```

## Quick Start

```bash
# Compile a Rust file to Luau
roblox-rs compile my_file.rs

# Create a new roblox-rs project
roblox-rs new my-game
```

## Documentation

Detailed documentation is coming soon. For now, see the examples directory for sample Rust code and its compiled Luau output.

## Roadmap to 1.0

See [PROJECT_PLAN.md](PROJECT_PLAN.md) for the current development status and roadmap.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 