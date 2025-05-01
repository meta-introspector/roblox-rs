---
title: Integrate GUI with ECS
---

# Integrate GUI with ECS

Learn how to connect roblox-rs-gui with roblox-rs-ecs to build interactive, data-driven Roblox games.

## Why Integrate?
- Use ECS for game logic and state management.
- Use GUI for player interaction and feedback.

## Example: Button Updates ECS State

```rust
// ECS component for score
struct Score(u32);

// System to update score
fn increment_score_system(mut query: Query<&mut Score>) {
    for mut score in query.iter_mut() {
        score.0 += 1;
    }
}

// GUI button that triggers ECS system
let button = Button::new()
    .text("Add Point")
    .on_click(move || {
        // Dispatch ECS event or directly call system
        world.run_system(increment_score_system);
    });
```

## Best Practices
- Use signals to reflect ECS state in the GUI.
- Keep UI logic and ECS logic modular.
- Use events or resources for communication between GUI and ECS.

## Next Steps
- [Advanced State Management](./advanced-state-management)
- [Reference: ECS](../reference/reference-ecs) 