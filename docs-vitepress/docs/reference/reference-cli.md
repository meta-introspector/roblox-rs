---
title: roblox-rs-cli Reference
---

# roblox-rs-cli Reference

The `roblox-rs-cli` is the command-line tool for compiling Rust to Luau and managing roblox-rs projects.

## Commands

- `roblox-rs compile <file>`: Compile a single Rust file to Luau.
- `roblox-rs build`: Build the entire project.
- `roblox-rs watch`: Watch for file changes and recompile automatically.
- `roblox-rs new <project>`: Scaffold a new roblox-rs project.

## Common Flags

- `--optimize [level]`: Set optimization level.
- `--debug`: Include debug info.
- `--output [dir]`: Set output directory.
- `--config [file]`: Use custom config.

## Example Usage

```sh
roblox-rs compile src/main.rs --optimize aggressive --debug
roblox-rs build --output out/
roblox-rs watch
```

See [Advanced Compiler Options](../how-to/advanced-compiler-options) for more. 