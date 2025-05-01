---
title: Creating Your First ECS Game
sidebar_label: Hello World ECS
---

# Creating Your First ECS Game

In this tutorial, you'll learn how to create a simple game using the Entity Component System (ECS) in roblox-rs. You'll build a basic game where cubes move around in a pattern.

## Prerequisites

- Completed the [Getting Started](getting-started.md) tutorial
- Basic familiarity with Rust concepts (structs, traits, functions)
- Basic understanding of Roblox Studio

## What is ECS?

The Entity Component System (ECS) is a pattern for game development that separates:

- **Entities**: Game objects (like players, items, enemies)
- **Components**: Data attached to entities (like position, health, speed)
- **Systems**: Logic that operates on entities with specific components

This separation makes code more modular, reusable, and easier to reason about.

## Step 1: Set Up Your Project

First, create a new project using the roblox-rs CLI:

```bash
roblox-rs new ecs-tutorial --ecs
cd ecs-tutorial
```

The `--ecs` flag includes the roblox-rs-ecs crate in your project.

## Step 2: Define Your Components

Open `src/main.rs` and define some simple components for our game:

```rust
use roblox_rs_ecs::prelude::*;

// Position component
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

// Velocity component
#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

// Visual component (for Roblox representation)
#[derive(Debug)]
struct Visual {
    color: Color,
}

// A simple color struct
#[derive(Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}
```

These components will store data about our entities.

## Step 3: Create Your Systems

Now let's define the systems that will operate on these components:

```rust
// Movement system: updates positions based on velocities
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}

// Boundary system: keeps entities within bounds
fn boundary_system(mut query: Query<(&mut Position, &mut Velocity)>) {
    const BOUNDARY: f32 = 10.0;
    
    for (mut position, mut velocity) in query.iter() {
        if position.x.abs() > BOUNDARY {
            velocity.x = -velocity.x;
            position.x = position.x.signum() * BOUNDARY;
        }
        
        if position.z.abs() > BOUNDARY {
            velocity.z = -velocity.z;
            position.z = position.z.signum() * BOUNDARY;
        }
    }
}

// Visualization system: creates/updates Roblox parts
fn visualization_system(query: Query<(Entity, &Position, &Visual)>, world: &World) {
    // In a real implementation, this would interact with Roblox instances
    // For this tutorial, we'll just print the state
    for (entity, position, visual) in query.iter() {
        println!(
            "Entity {:?} at position ({}, {}, {}) with color ({}, {}, {})",
            entity,
            position.x, position.y, position.z,
            visual.color.r, visual.color.g, visual.color.b
        );
    }
}
```

## Step 4: Set Up Your Main Function

Finally, let's tie everything together in the main function:

```rust
fn main() {
    // Create our app with the ECS framework
    let mut app = App::new();
    
    // Add our systems
    app.add_system(movement_system)
       .add_system(boundary_system)
       .add_system(visualization_system);
    
    // Spawn some entities
    app.world_mut().spawn((
        Position { x: 0.0, y: 0.0, z: 0.0 },
        Velocity { x: 0.1, y: 0.0, z: 0.2 },
        Visual { color: Color { r: 1.0, g: 0.0, b: 0.0 } }
    ));
    
    app.world_mut().spawn((
        Position { x: 2.0, y: 0.0, z: 3.0 },
        Velocity { x: -0.1, y: 0.0, z: 0.1 },
        Visual { color: Color { r: 0.0, g: 1.0, b: 0.0 } }
    ));
    
    // Run the app (in a real game, this would be part of the game loop)
    for _ in 0..100 {
        app.update();
    }
}
```

## Step 5: Build and Run

Build your project:

```bash
roblox-rs build
```

Import the generated Luau files into Roblox Studio as described in the previous tutorial.

## What's Happening?

Let's break down what's happening in this example:

1. We define **Components** (Position, Velocity, Visual) that hold data
2. We create **Systems** (movement, boundary, visualization) that act on entities with specific components
3. We **Spawn Entities** with combinations of components
4. The ECS **Update Loop** runs all systems on matching entities

## Next Steps

This example shows the basics of ECS in roblox-rs. To expand on this:

- Try adding more components (like Rotation or Scale)
- Create new systems (like collision detection)
- Integrate with actual Roblox objects using roblox-rs's platform-specific APIs

## Complete Code

Here's the complete code for reference:

```rust
use roblox_rs_ecs::prelude::*;

// Components
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct Visual {
    color: Color,
}

#[derive(Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

// Systems
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
    }
}

fn boundary_system(mut query: Query<(&mut Position, &mut Velocity)>) {
    const BOUNDARY: f32 = 10.0;
    
    for (mut position, mut velocity) in query.iter() {
        if position.x.abs() > BOUNDARY {
            velocity.x = -velocity.x;
            position.x = position.x.signum() * BOUNDARY;
        }
        
        if position.z.abs() > BOUNDARY {
            velocity.z = -velocity.z;
            position.z = position.z.signum() * BOUNDARY;
        }
    }
}

fn visualization_system(query: Query<(Entity, &Position, &Visual)>, world: &World) {
    for (entity, position, visual) in query.iter() {
        println!(
            "Entity {:?} at position ({}, {}, {}) with color ({}, {}, {})",
            entity,
            position.x, position.y, position.z,
            visual.color.r, visual.color.g, visual.color.b
        );
    }
}

fn main() {
    // Create our app
    let mut app = App::new();
    
    // Add our systems
    app.add_system(movement_system)
       .add_system(boundary_system)
       .add_system(visualization_system);
    
    // Spawn some entities
    app.world_mut().spawn((
        Position { x: 0.0, y: 0.0, z: 0.0 },
        Velocity { x: 0.1, y: 0.0, z: 0.2 },
        Visual { color: Color { r: 1.0, g: 0.0, b: 0.0 } }
    ));
    
    app.world_mut().spawn((
        Position { x: 2.0, y: 0.0, z: 3.0 },
        Velocity { x: -0.1, y: 0.0, z: 0.1 },
        Visual { color: Color { r: 0.0, g: 1.0, b: 0.0 } }
    ));
    
    // Run the app
    for _ in 0..100 {
        app.update();
    }
} 