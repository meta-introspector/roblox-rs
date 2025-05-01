---
title: Advanced Compiler Options
---

# Advanced Compiler Options

This guide covers all advanced options available in the roblox-rs compiler and CLI. Use these to customize your build, optimize output, and debug issues.

## CLI Flags

- `--optimize [level]`: Set optimization level (`minimal`, `default`, `aggressive`).
- `--debug`: Include debug information in the Luau output.
- `--output [dir]`: Specify a custom output directory for generated files.
- `--config [file]`: Use a custom configuration file.
- `--watch`: Enable watch mode for automatic recompilation.
- `--target [platform]`: Set the compilation target (`roblox`, `native`).

## Example Usage

```sh
roblox-rs compile src/main.rs --optimize aggressive --debug --output out/
roblox-rs build --config roblox-rs.toml --target roblox
```

## Configuration File (`roblox-rs.toml`)

```toml
[build]
output_dir = "game/scripts"
include_runtime = true
optimize = true
debug_mode = false

[compiler]
target = "roblox"
```

## Tips
- Use `--debug` during development for better error messages.
- Set `--optimize aggressive` for production builds.
- Use a custom config file for complex projects.

See the [CLI Reference](../reference/reference-cli) for all commands. 