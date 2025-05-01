use roblox_rs_ecs::prelude::*;

// Simple test components
#[derive(Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32, 
    z: f32,
}

impl Component for Position {}

#[derive(Debug, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {}

// Simple resource for testing
#[derive(Debug, PartialEq, Default)]
struct FrameCounter {
    count: u32,
}

impl Resource for FrameCounter {}

// Test system that increments a counter
fn increment_counter_system(mut counter: ResMut<FrameCounter>) {
    counter.count += 1;
}

// Test system that moves entities based on velocity
// (Using world access directly to avoid query lifetime issues)
fn movement_system(world: &mut World) {
    // Get all entities with both Position and Velocity
    let entities = world.entities();
    
    for entity in entities {
        if let Ok(entity_ref) = world.entity(entity) {
            if entity_ref.has::<Position>() && entity_ref.has::<Velocity>() {
                // In a real implementation, we'd use queries for this
                // Just testing the concept here
                let position = world.get_resource::<Position>().unwrap();
                let velocity = world.get_resource::<Velocity>().unwrap();
                
                // Apply movement (this would normally be done with mutable component access)
                world.insert_resource(Position {
                    x: position.x + velocity.x,
                    y: position.y + velocity.y,
                    z: position.z + velocity.z,
                });
            }
        }
    }
}

#[test]
fn test_basic_system_execution() {
    // Set up app and resources
    let mut app = App::new();
    app.init_resource::<FrameCounter>();
    
    // Add a simple system
    app.add_system(increment_counter_system);
    
    // Run the app once
    app.update();
    
    // Verify the system ran and updated the counter
    let counter = app.world().get_resource::<FrameCounter>().unwrap();
    assert_eq!(counter.count, 1);
    
    // Run again
    app.update();
    
    // Verify counter was incremented again
    let counter = app.world().get_resource::<FrameCounter>().unwrap();
    assert_eq!(counter.count, 2);
}

#[test]
fn test_system_execution_order() {
    // Set up app with two systems that increment the counter
    let mut app = App::new();
    app.init_resource::<FrameCounter>();
    
    // Add systems
    app.add_system(increment_counter_system)
       .add_system(increment_counter_system);
    
    // Run the app once
    app.update();
    
    // The counter should be incremented twice since we have two systems
    let counter = app.world().get_resource::<FrameCounter>().unwrap();
    assert_eq!(counter.count, 2);
} 