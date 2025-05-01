---
title: Understanding roblox-rs Architecture
sidebar_label: Architecture
---

# Understanding roblox-rs Architecture

This document explains the architectural design of roblox-rs, the design decisions behind it, and how the different components work together.

## The Big Picture

roblox-rs is a toolkit that enables Rust code to run on the Roblox platform. At its core, it's a specialized compiler that transforms Rust source code into Luau, the programming language used by Roblox.

The monorepo is organized into multiple crates, each with a specific responsibility:

```
roblox-rs/
├── roblox-rs-core/      # Core compilation engine
├── roblox-rs-ecs/       # Entity Component System framework
├── roblox-rs-cli/       # Command-line interface
├── roblox-rs-gui/       # Reactive GUI framework
└── roblox-rs-rojo/      # (Planned) Rojo integration
```

## Compilation Pipeline

The compilation process in roblox-rs-core follows a multi-stage pipeline:

1. **Parsing**: Rust source code is parsed into an abstract syntax tree (AST) using the `syn` crate.
2. **Analysis**: The AST is analyzed to collect type information, dependencies, and other metadata.
3. **Transformation**: The Rust AST is transformed into an intermediate representation that's closer to Luau's semantics.
4. **Optimization**: The intermediate representation is optimized for performance and size.
5. **Code Generation**: Luau code is generated from the optimized intermediate representation.

This pipeline design allows for separation of concerns and makes it easier to modify or extend individual stages without affecting the others.

### Why Not LLVM?

A common question is why roblox-rs doesn't use LLVM's infrastructure to compile Rust to Luau. There are several reasons:

1. **Semantic Gap**: The gap between Rust (statically typed, compiled) and Luau (dynamically typed, interpreted) is significant and would be difficult to bridge at the LLVM IR level.
2. **Control**: Direct AST manipulation gives us fine-grained control over the transformation process.
3. **Complexity**: LLVM adds significant complexity and dependencies to the project.
4. **Performance**: For our specific use case, a specialized source-to-source compiler is more efficient.

## Entity Component System (ECS)

The Entity Component System in roblox-rs-ecs is inspired by the Bevy game engine but adapted for Roblox's environment. The core principles include:

### Data-Oriented Design

Traditional object-oriented game development organizes code around entities (objects) that contain both data and behavior. ECS instead separates:

- **Entities**: Simple identifiers that represent game objects
- **Components**: Pure data with no behavior attached to entities
- **Systems**: Logic that processes entities with specific components

This separation enables better performance, more modular code, and more flexible game design.

### ECS Architecture

The ECS architecture in roblox-rs follows this design:

```
+-------------+     +----------------+     +-------------+
|  Entities   |---->|   Components   |---->|   Systems   |
| (Identifiers)|     | (Data Storage) |     | (Behavior)  |
+-------------+     +----------------+     +-------------+
                            ^                    |
                            |                    |
                            +--------------------+
                              Query & Execution
```

- **World**: The central container that stores all entities, components, and resources.
- **Resources**: Global singleton data accessible to systems.
- **Queries**: Efficiently filter and retrieve entities with specific component combinations.
- **Schedule**: Determines the order of system execution.

## Reactive GUI System

The GUI system in roblox-rs-gui is built around the concept of reactivity, inspired by modern web frameworks like Svelte. The key concepts include:

### Signals and Reactivity

A signal is a container for a value that can change over time. When a signal's value changes, any UI elements that depend on that signal are automatically updated.

```
+------------+     +----------------+     +---------------+
|  Signals   |---->| Change Events  |---->| UI Components |
| (State)    |     | (Propagation)  |     | (View)        |
+------------+     +----------------+     +---------------+
```

### Virtual DOM

While the term "Virtual DOM" is borrowed from web development, the concept in roblox-rs-gui is adapted for Roblox's UI system:

1. UI component trees are defined in Rust
2. When state changes, a new "virtual" representation is created
3. This is compared with the previous version to find differences
4. Only the minimum necessary changes are applied to the actual Roblox UI instances

This approach minimizes the number of updates to Roblox instances, which are relatively expensive operations.

## CLI and Tooling

The CLI in roblox-rs-cli is designed to be the primary interface for developers. It provides commands for:

- Creating new projects
- Compiling Rust code to Luau
- Building entire projects
- Watching for file changes and automatically recompiling

The CLI follows a design pattern similar to tools like Cargo, with subcommands for different operations and consistent flag naming.

## Cross-Platform Considerations

roblox-rs is designed to work in two environments:

1. **Roblox**: The primary target platform where compiled Luau code runs
2. **Native**: For testing and development on local machines

This dual-target approach is handled through conditional compilation and platform-specific abstractions, particularly in the ECS framework.

## Future Directions

The architecture is designed to be extensible in several directions:

- **Debugging Tools**: Better integration with Roblox's debugging capabilities
- **Hot Reloading**: Faster development cycles through runtime code replacement
- **Advanced Optimization**: More sophisticated optimization passes for better performance
- **Rojo Integration**: Seamless integration with Rojo, the standard Roblox development tool

## Design Principles

Throughout the codebase, several core design principles are followed:

1. **Separation of Concerns**: Each component has a clear, distinct responsibility
2. **Progressive Disclosure**: Simple APIs for common tasks, with deeper capabilities available when needed
3. **Rust Idioms**: Following Rust's best practices and idioms where possible
4. **Performance**: Optimizing for both compile-time and runtime performance
5. **Developer Experience**: Making the developer's life easier through good error messages, documentation, and tooling

These principles guide the evolution of roblox-rs and help maintain consistency across the codebase.

## Conclusion

roblox-rs represents a novel approach to Roblox game development by leveraging Rust's powerful type system, performance characteristics, and ecosystem. Its architecture is designed to provide a seamless experience while bridging the gap between Rust and Luau. 