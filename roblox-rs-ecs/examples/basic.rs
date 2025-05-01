//! A basic example of the roblox-rs-ecs framework

use roblox_rs_ecs::prelude::*;

// Define components
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
struct Player {
    name: String,
    health: f32,
}

// Define resources
#[derive(Debug, Default)]
struct GameState {
    score: i32,
    time_elapsed: f32,
}

// Define events
struct CollisionEvent {
    entity1: Entity,
    entity2: Entity,
}

// Define systems
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter() {
        position.x += velocity.x;
        position.y += velocity.y;
        position.z += velocity.z;
        
        println!("Moved entity to position: {:?}", position);
    }
}

fn player_system(query: Query<(&Player, &Position)>) {
    for (player, position) in query.iter() {
        println!(
            "Player {} at position [{:.1}, {:.1}, {:.1}] with health {:.1}",
            player.name, position.x, position.y, position.z, player.health
        );
    }
}

fn game_state_system(mut game_state: ResMut<GameState>) {
    game_state.score += 1;
    game_state.time_elapsed += 0.1;
    
    println!(
        "Game state: Score={}, Time={:.1}s",
        game_state.score, game_state.time_elapsed
    );
}

fn collision_detection_system(
    query: Query<(&Position, &Player)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // Simulate a collision for demonstration purposes
    let entities: Vec<_> = query.iter().map(|(_, _)| 1).collect();
    
    if entities.len() >= 2 {
        collision_events.send(CollisionEvent {
            entity1: Entity::new(1),
            entity2: Entity::new(2),
        });
        
        println!("Collision detected between entities 1 and 2");
    }
}

fn collision_response_system(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        println!(
            "Handling collision between entities {:?} and {:?}",
            event.entity1, event.entity2
        );
    }
}

fn setup(mut commands: Commands, workspace: Res<Workspace>) {
    // Create a player entity
    commands.spawn((
        Player {
            name: "Player1".to_string(),
            health: 100.0,
        },
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Velocity {
            x: 1.0,
            y: 0.5,
            z: 0.0,
        },
    ));
    
    // Create another entity
    commands.spawn((
        Player {
            name: "Player2".to_string(),
            health: 85.0,
        },
        Position {
            x: 10.0,
            y: 5.0,
            z: 0.0,
        },
        Velocity {
            x: -0.5,
            y: 0.0,
            z: 0.2,
        },
    ));
    
    println!("Created entities in {}", workspace.instance().name());
}

fn main() {
    println!("Starting roblox-rs-ecs example");
    
    // Create and run the app
    let mut app = App::new();
    
    app.add_plugin(RobloxPlugin)
        .init_resource::<GameState>()
        .add_system(setup.into_system())
        .add_system(movement_system.into_system())
        .add_system(player_system.into_system())
        .add_system(game_state_system.into_system())
        .add_system(collision_detection_system.into_system())
        .add_system(collision_response_system.into_system());
    
    println!("Running app...");
    app.run();
    
    println!("App finished running");
} 