# roblox-rs-rojo

Rojo integration for the roblox-rs compiler.

## Overview

`roblox-rs-rojo` provides seamless integration between roblox-rs projects and [Rojo](https://rojo.space/), the popular Roblox development tool. This integration allows developers to leverage both the power of Rust and the mature project management capabilities of Rojo.

## Features

- **Project Structure Compatibility**: Generate Rojo-compatible project structures from Rust code
- **Automated Synchronization**: Keep Rust source and Rojo project files in sync
- **Build Pipeline Integration**: Integrate roblox-rs compilation into your Rojo workflow
- **Asset Management**: Handle non-Rust assets within your project
- **Development Server**: Live sync compiled Luau to Roblox Studio
- **Project Templates**: Ready-to-use templates for common project types

## Installation

```bash
# Install as a global tool (alongside roblox-rs-cli)
cargo install roblox-rs-rojo

# Or add to an existing project
cd my-roblox-rs-project
roblox-rs add rojo
```

## Usage

### Initialize a new roblox-rs project with Rojo support

```bash
# Create a new project with Rojo support
roblox-rs new my-game --with-rojo

# Or add Rojo to an existing project
cd existing-project
roblox-rs add rojo
```

### Compile and sync with Rojo

```bash
# In your project directory
roblox-rs rojo serve
```

This command:
1. Compiles your Rust code to Luau
2. Generates a proper Rojo project structure 
3. Starts the Rojo server for syncing to Roblox Studio

### Project Structure

A typical project with Rojo integration looks like:

```
my-game/
├── Cargo.toml                  # Rust dependencies
├── roblox-rs.toml              # roblox-rs configuration
├── default.project.json        # Auto-generated Rojo project file
├── src/
│   ├── main.rs                 # Rust entry point
│   └── lib/
│       ├── player.rs           # Rust modules
│       └── ...
├── assets/
│   ├── sounds/                 # Non-code assets
│   └── textures/
└── out/                        # Generated Luau output
    ├── init.lua
    └── lib/
        ├── player.lua
        └── ...
```

### Configuration

In your `roblox-rs.toml` file:

```toml
[rojo]
project_file = "default.project.json"  # Default Rojo project file
sync_dir = "out"                       # Where compiled Luau goes
port = 34872                           # Rojo server port

[rojo.mapping]
# Custom directory mappings
"src/client" = "StarterPlayerScripts"
"src/server" = "ServerScriptService"
"assets" = "ReplicatedStorage/Assets"
```

## Workflow Integration

`roblox-rs-rojo` is designed to fit smoothly into existing Roblox development workflows:

1. Write your game logic in Rust
2. Let `roblox-rs` compile to Luau
3. Use `roblox-rs-rojo` to sync with Rojo
4. Test in Roblox Studio
5. Make changes and repeat - with live updates!

## Related Crates

- `roblox-rs-core`: Core compiler for the Rust to Luau translation
- `roblox-rs-cli`: Command-line interface for the compiler
- `roblox-rs-ecs`: Entity Component System framework for Roblox games

## License

This project is licensed under the MIT License - see the LICENSE file for details. 