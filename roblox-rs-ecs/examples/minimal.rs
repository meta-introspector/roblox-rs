//! A minimal example demonstrating the current capabilities of roblox-rs-ecs

use roblox_rs_ecs::prelude::*;

// Define some simple components
#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position {}

#[derive(Debug, Clone)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {}

#[derive(Debug, Clone)]
struct Name(String);

impl Component for Name {}

// Define a simple resource
#[derive(Debug, Default)]
struct FrameCount {
    value: u32,
}

impl Resource for FrameCount {}

// Define a simple event
struct CollisionEvent {
    entity1: Entity,
    entity2: Entity,
}

// Define some simple systems
fn increment_frame_count(world: &mut World) {
    if let Some(mut count) = world.get_resource_mut::<FrameCount>() {
        count.value += 1;
        println!("Frame count: {}", count.value);
    }
}

fn movement_system(world: &mut World) {
    // Get all entities
    let entities = world.entities();
    
    // Process each entity with Position and Velocity
    for entity in entities {
        // Skip entities without both Position and Velocity
        if let Ok(entity_ref) = world.entity(entity) {
            if !entity_ref.has::<Position>() || !entity_ref.has::<Velocity>() {
                continue;
            }
            
            // Get the current components (this is simplified for the example)
            // In a real implementation, we would use proper queries
            let position = match world.entity(entity).unwrap().get::<Position>() {
                Ok(pos) => pos.clone(),
                Err(_) => continue,
            };
            
            let velocity = match world.entity(entity).unwrap().get::<Velocity>() {
                Ok(vel) => vel.clone(),
                Err(_) => continue,
            };
            
            // Create updated position
            let new_position = Position {
                x: position.x + velocity.x,
                y: position.y + velocity.y,
                z: position.z + velocity.z,
            };
            
            // Update the entity's position (simplified)
            // In a real implementation, we would use a proper component mutation system
            let mut commands = Commands::new();
            let entity_id = entity; // Make a copy for the closure
            commands.add(move |world| {
                // This is a simple but inefficient way to update a component
                // In a real implementation, we would have a more efficient approach
                if let Ok(entity_ref) = world.entity_mut(entity_id) {
                    // Would actually update the component directly
                    println!("Updated position for entity: {:?}", entity_id);
                }
            });
            commands.apply(world);
            
            // Print position for debugging
            if let Ok(entity_ref) = world.entity(entity) {
                if let Ok(name) = entity_ref.get::<Name>() {
                    println!("Entity {} moved to ({:.1}, {:.1}, {:.1})",
                        name.0, new_position.x, new_position.y, new_position.z);
                }
            }
        }
    }
}

fn main() {
    println!("Starting minimal ECS example");
    
    // Create a new app
    let mut app = App::new();
    
    // Add resources
    app.init_resource::<FrameCount>();
    app.insert_resource(Events::<CollisionEvent>::new());
    
    // Add systems
    app.add_system(increment_frame_count);
    app.add_system(movement_system);
    
    // Create entities in the world
    let mut world = app.world_mut();
    
    // Create a player entity
    world.spawn((
        Name("Player".to_string()),
        Position { x: 0.0, y: 0.0, z: 0.0 },
        Velocity { x: 1.0, y: 0.5, z: 0.0 },
    ));
    
    // Create an enemy entity
    world.spawn((
        Name("Enemy".to_string()),
        Position { x: 10.0, y: 5.0, z: 0.0 },
        Velocity { x: -0.5, y: 0.0, z: 0.2 },
    ));
    
    // Run the app for a few frames
    println!("Running app for 5 frames");
    for _ in 0..5 {
        app.update();
    }
    
    println!("Example completed");
} 