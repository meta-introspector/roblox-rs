# Roblox-rs Project Plan

## Core Goal

Create a compiler that translates Rust code into optimized Luau code suitable for running on the Roblox platform. Inspired by `roblox-ts`.

## Key Features / Ideas

*   **Rust to Luau Compilation:**
    *   Use `syn` crate to parse Rust source into AST
    *   Transform Rust AST directly to Luau AST
    *   Preserve semantic information for better code generation
    *   Handle Rust concepts (types, ownership, traits) mapping to Luau
    *   Utilize Luau features like `buffer` for optimization
*   **Type System:**
    *   Map Rust types to Luau types
    *   Handle generics and traits
    *   Implement smart type inference where possible
*   **Optimization:** Generate highly optimized Luau code (`--!optimize 2`, `--!native`).
*   **Roblox API Integration:** Provide mechanisms for Rust code to interact with Roblox APIs.
*   **Concurrency:** Support Roblox Actors and async patterns (mapping Rust async/await).
*   **Tooling:** Develop a user-friendly CLI (`roblox-rs-cli`).

## Current Status

*   Workspace created (`roblox-rs`).
*   CLI package stubbed (`roblox-rs-cli`).
*   Core library package stubbed (`roblox-rs-core`).
*   Basic link established: `roblox-rs-cli` depends on and calls a placeholder function in `roblox-rs-core`.

## Next Steps

1.  **Add Core Dependencies:**
    - `syn` for Rust parsing
    - `quote` for code generation
    - `proc-macro2` for token handling

2.  **AST Processing:**
    - Implement Rust AST visitor/traversal
    - Define Luau AST structures
    - Create initial AST transformation pipeline

3.  **Basic Type System:**
    - Define type mapping rules
    - Implement basic type checker
    - Handle primitive types and basic structs

4.  **Code Generation:**
    - Implement Luau code emitter
    - Generate basic function and struct definitions
    - Handle simple expressions and control flow

## Roadmap to 1.0

### Alpha Phase (0.1.x)
- [ ] Basic CLI interface
- [ ] Rust parsing with `syn`
- [ ] Simple type mapping (primitives)
- [ ] Basic expression transformation
- [ ] Initial Luau code generation
- [ ] Support for simple Rust functions & structs

### Beta Phase (0.2.x - 0.9.x)
- [ ] Full type system support
  - [ ] Generics
  - [ ] Traits
  - [ ] Enums & pattern matching
- [ ] Control flow (if/else, loops, match)
- [ ] Advanced features
  - [ ] Ownership system mapping
  - [ ] Error handling
  - [ ] Closures
- [ ] Standard library support
  - [ ] Basic collections (Vec, HashMap, etc.)
  - [ ] Option/Result types
- [ ] Roblox-specific features
  - [ ] Instance manipulation
  - [ ] EventHandling
- [ ] Project management
  - [ ] Multi-file projects
  - [ ] Dependencies
  - [ ] Module system
- [ ] Framework and tool integration
  - [ ] Rojo project structure integration (`roblox-rs-rojo`)
  - [ ] ECS game framework (`roblox-rs-ecs`)
  - [ ] Build and deploy workflows

### Release Candidate (0.9.x)
- [ ] Performance optimizations
- [ ] Comprehensive test suite
- [ ] Documentation
  - [ ] User guide
  - [ ] API documentation
  - [ ] Examples
- [ ] Compatibility testing with various Roblox features

### Version 1.0
- [ ] API stability
- [ ] Full compiler feature set
- [ ] Battle-tested on real-world projects
- [ ] Continuous integration & automated testing

## Post-1.0 Ideas
- IDE integration
- Language server protocol support
- Advanced optimizations
- Wasm target support
- Plugin ecosystem