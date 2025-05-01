# roblox-rs-ecs

A Bevy-inspired Entity Component System (ECS) framework for Roblox game development using Rust.

## Overview

`roblox-rs-ecs` is a data-oriented framework designed specifically for creating games on the Roblox platform. It provides a powerful ECS architecture inspired by the popular Bevy game engine, adapted for the unique needs of Roblox development.

This crate is part of the larger `roblox-rs` project, which aims to bring the safety and performance of Rust to Roblox game development through compilation to optimized Luau.

## Separation of Powers

The framework follows a strict separation of powers to enable a maintainable and scalable architecture:

### Core Components

1. **Entities**: Unique identifiers that represent game objects
   - Entities are lightweight IDs that serve as containers for components
   - Created and managed through the World or EntityBuilder

2. **Components**: Pure data containers with no behavior
   - Attached to entities to define their properties and state
   - Implemented as simple Rust structs with no methods
   - Can be dynamically added, removed, or modified at runtime

3. **Resources**: Global state shared across systems
   - Singleton data that isn't associated with any specific entity
   - Accessible by systems that need global information
   - Include Roblox services and application-wide state

4. **Systems**: Logic that operates on entities with specific components
   - Defined as functions that query for entities with specific component combinations
   - Cannot directly communicate with each other (only through resources and events)
   - Run in a deterministic order defined by the Schedule

5. **Queries**: Efficient access to entities with specific component types
   - Allow systems to retrieve only the entities they need to process
   - Support filtering, sorting, and batched operations
   - Enable high-performance iteration over large collections of entities

6. **World**: Central container for all entities, components, and resources
   - Maintains the relationship between entities and their components
   - Manages resource storage and retrieval
   - Provides the execution context for systems

7. **Events**: Type-safe communication between systems
   - Allow decoupled systems to communicate through events
   - Support event production (EventWriter) and consumption (EventReader)
   - Enable reactive programming patterns

### Plugin Architecture

1. **App**: Main entry point and container for the ECS
   - Manages the world and schedule
   - Provides an API for adding systems, resources, and plugins

2. **Plugins**: Modular extensions to add functionality
   - Can register systems, resources, and events
   - Enable a composable architecture
   - Include specialized plugins like RobloxPlugin for Roblox service integration

3. **Schedule**: Controls the execution flow of systems
   - Organizes systems into ordered stages
   - Determines when systems run relative to each other
   - Handles command buffer application between system executions

## Roadmap to Version 1.0

### Phase 1: Core Architecture (0.1.x)
- [x] Basic Entity-Component model
- [x] World and resource management
- [x] Simple system execution
- [x] Component registration
- [ ] Fix current compilation errors
- [ ] Refine system parameter fetching
- [ ] Improve query implementation
- [ ] Stabilize core APIs

### Phase 2: Enhanced Features (0.2.x)
- [ ] Robust error handling throughout the framework
- [ ] Improved event system with event filtering
- [ ] Reflection capabilities for components and resources
- [ ] Hierarchical entity relationships
- [ ] Asset handling integrated with Roblox assets
- [ ] Type-safe state transitions
- [ ] Commands system refinement

### Phase 3: Roblox Integration (0.3.x)
- [ ] Complete Roblox service wrappers
- [ ] Seamless instance-to-entity mapping
- [ ] Replication support for client-server architecture
- [ ] Remote event handling
- [ ] DataStore integration
- [ ] UI component integration

### Phase 4: Performance and Optimization (0.4.x)
- [ ] Parallel system execution
- [ ] Archetype-based storage for optimal component access
- [ ] System dependency graph optimization
- [ ] Memory usage optimization
- [ ] Benchmarking tools
- [ ] Change detection for efficient updates

### Phase 5: Developer Experience (0.5.x)
- [ ] Comprehensive documentation with examples
- [ ] Runtime debugging tools
- [ ] Component inspector
- [ ] Hot reloading support
- [ ] Testing utilities
- [ ] More example projects

### Phase 6: Advanced Features (0.6.x - 0.9.x)
- [ ] Time and timer abstraction
- [ ] Physics integration
- [ ] Animation system
- [ ] Spatial partitioning for efficient queries
- [ ] Audio system integration
- [ ] Asset bundling and loading
- [ ] Scene serialization

### Phase 7: Stabilization (1.0.0)
- [ ] API finalization
- [ ] Performance optimizations
- [ ] Full documentation coverage
- [ ] Compatibility guarantees
- [ ] Migration guides from previous versions

## Current Status

This framework is in early development (0.1.0) and not yet ready for production use. We're focusing on building a solid foundation before adding more features.

## Usage Example

```rust
use roblox_rs_ecs::prelude::*;

// Define components
#[derive(Debug)]
struct Position { x: f32, y: f32, z: f32 }

#[derive(Debug)]
struct Velocity { x: f32, y: f32, z: f32 }

// Define systems
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}

fn main() {
    // Create and run the app
    let mut app = App::new();
    
    app.add_plugin(RobloxPlugin)
       .add_system(movement_system.into_system());
    
    app.run();
}
```

## Inspiration

This framework is heavily inspired by the [Bevy](https://bevyengine.org/) game engine, adapted for Roblox's unique environment. We've borrowed many architectural concepts while tailoring the implementation for Roblox's constraints and opportunities.

## Contributing

We welcome contributions! Please see our [Contributing Guide](../CONTRIBUTING.md) for details on how to get involved.

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details. 