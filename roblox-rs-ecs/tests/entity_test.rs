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

#[derive(Debug, PartialEq)]
struct Health {
    current: f32,
    max: f32,
}

impl Component for Health {}

// Simple resource for testing
#[derive(Debug, PartialEq)]
struct GameTime {
    elapsed: f32,
}

impl Resource for GameTime {}

#[test]
fn test_entity_creation() {
    let mut world = World::new();
    
    // Create an entity with components
    let entity = world.spawn((
        Position { x: 0.0, y: 1.0, z: 2.0 },
        Velocity { x: 1.0, y: 0.0, z: 0.0 },
    ));
    
    // Verify the entity exists
    assert!(world.entity(entity).is_ok());
}

#[test]
fn test_resource_management() {
    let mut world = World::new();
    
    // Insert a resource
    world.insert_resource(GameTime { elapsed: 0.0 });
    
    // Check if resource exists
    assert!(world.has_resource::<GameTime>());
    
    // Get and modify resource
    if let Some(time) = world.get_resource_mut::<GameTime>() {
        time.elapsed += 0.1;
    }
    
    // Verify modification
    let time = world.get_resource::<GameTime>().unwrap();
    assert_eq!(time.elapsed, 0.1);
    
    // Remove resource
    let removed = world.remove_resource::<GameTime>();
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().elapsed, 0.1);
    
    // Verify resource was removed
    assert!(!world.has_resource::<GameTime>());
}

#[test]
fn test_entity_builder() {
    let mut world = World::new();
    
    // Use the entity builder
    let entity = world.spawn_builder()
        .with(Position { x: 1.0, y: 2.0, z: 3.0 })
        .with(Health { current: 100.0, max: 100.0 })
        .build(&mut world);
    
    // Verify entity and components
    let entity_ref = world.entity(entity).unwrap();
    assert!(entity_ref.has::<Position>());
    assert!(entity_ref.has::<Health>());
    assert!(!entity_ref.has::<Velocity>());
} 