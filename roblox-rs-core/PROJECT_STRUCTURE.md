# Roblox-RS Project Structure

This document outlines the reorganized project structure for better maintainability and separation of concerns.

## Directory Structure

```
roblox-rs-core/
├── src/
│   ├── core/              # Core functionality and shared utilities
│   │   ├── error.rs       # Error handling
│   │   ├── utils.rs       # Shared utilities
│   │   └── mod.rs         # Core module exports
│   │
│   ├── transpiler/        # Rust-to-Luau transpilation
│   │   ├── ast/           # Abstract Syntax Tree 
│   │   ├── parser/        # Rust parsing
│   │   ├── transforms/    # AST transformations
│   │   ├── codegen/       # Luau code generation
│   │   └── mod.rs         # Transpiler module exports
│   │
│   ├── runtime/           # Runtime libraries for Luau
│   │   ├── actor/         # Actor system
│   │   ├── instance/      # Roblox instance interaction
│   │   ├── networking/    # High-performance networking
│   │   ├── helpers/       # Runtime helpers
│   │   └── mod.rs         # Runtime module exports
│   │
│   ├── packaging/         # Packaging code and assets
│   │   ├── place/         # Place file generation
│   │   ├── assets/        # Asset handling
│   │   └── mod.rs         # Packaging module exports
│   │
│   ├── bundling/          # Final project bundling
│   │   ├── workspaces/    # Client/Shared/Server workspaces
│   │   ├── rbxl/          # RBXL file generation
│   │   └── mod.rs         # Bundling module exports
│   │
│   ├── tools/             # Developer tools
│   │   ├── cli/           # Command-line interface
│   │   ├── testing/       # Testing utilities
│   │   └── mod.rs         # Tools module exports
│   │
│   └── lib.rs             # Main library interface
│
├── examples/              # Example projects
│   ├── basic/             # Basic example
│   ├── actors/            # Actor system example
│   ├── networking/        # Networking example
│   └── full_game/         # Complete game example
│
├── tests/                 # Integration tests
│   ├── transpiler_tests/  # Transpiler tests
│   ├── runtime_tests/     # Runtime tests
│   └── e2e_tests/         # End-to-end tests
│
└── docs/                  # Documentation
    ├── user_guide/        # User guide
    ├── api/               # API documentation
    └── examples/          # Example documentation
```

## Component Overview

### Core
Contains foundational functionality used across the project, including error handling, logging, and shared utilities.

### Transpiler
The Rust-to-Luau transpilation engine, responsible for parsing Rust code, transforming the AST, and generating Luau code.

### Runtime
Runtime libraries that are included in the generated Luau code, providing functionality like actor systems, networking, and Roblox instance interaction.

### Packaging
Tools for packaging code and assets into a format usable by Roblox, including place file generation.

### Bundling
Handles the final bundling of transpiled code and assets into client, shared, and server workspaces.

### Tools
Developer tools for working with Roblox-RS, including the CLI and testing utilities.

## Module Dependencies

- `core` has no dependencies on other modules
- `transpiler` depends on `core`
- `runtime` depends on `core`
- `packaging` depends on `core`, `runtime`
- `bundling` depends on `core`, `runtime`, `packaging`
- `tools` depends on all other modules
