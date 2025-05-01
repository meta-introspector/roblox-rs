---
title: roblox-rs-ecs Reference
---

# roblox-rs-ecs Reference

The `roblox-rs-ecs` crate provides an Entity Component System (ECS) framework for Roblox games.

## Core Concepts

- **Entity**: Unique identifier for game objects.
- **Component**: Data attached to entities (e.g., Position, Health).
- **System**: Logic that processes entities with specific components.
- **World**: Container for all entities, components, and resources.
- **Resource**: Global singleton data accessible to systems.

## Example: Defining a Component

```rust
struct Health(u32);

fn damage_system(mut query: Query<&mut Health>) {
    for mut health in query.iter_mut() {
        health.0 = health.0.saturating_sub(10);
    }
}
```

## Scheduling Systems

```rust
let mut world = World::new();
let mut schedule = Schedule::new();
schedule.add_system(damage_system);
schedule.run(&mut world);
```

## Best Practices
- Keep components small and focused.
- Use queries to efficiently filter entities.
- Use resources for shared/global state.

See [Integrate GUI with ECS](../how-to/integrate-gui-with-ecs) for more. 