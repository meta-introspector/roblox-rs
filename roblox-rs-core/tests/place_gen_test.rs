//! Test for the Roblox-RS place generator
//! This test validates our place file generation capability for the 1.0 speed run

use std::path::Path;
use std::fs;
use tempfile;
use roblox_rs_core::packaging::place_gen::PlaceFile;

#[test]
fn test_place_generator() {
    // Create a temporary project structure
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let project_dir = temp_dir.path().join("test_project");
    
    // Create project directories
    let client_dir = project_dir.join("client");
    let server_dir = project_dir.join("server");
    let shared_dir = project_dir.join("shared");
    let assets_dir = project_dir.join("assets");
    let output_dir = temp_dir.path().join("output");
    
    fs::create_dir_all(&client_dir).expect("Failed to create client directory");
    fs::create_dir_all(&server_dir).expect("Failed to create server directory");
    fs::create_dir_all(&shared_dir).expect("Failed to create shared directory");
    fs::create_dir_all(&assets_dir).expect("Failed to create assets directory");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Create sample Rust files
    let client_file = client_dir.join("ui_controller.rs");
    let server_file = server_dir.join("game_controller.rs");
    let shared_file = shared_dir.join("game_types.rs");
    
    // Simple client UI controller
    fs::write(&client_file, r#"
// Client UI controller
fn setup_ui() {
    println!("Setting up UI");
    
    // Create UI elements
    let screen_gui = Instance::new("ScreenGui");
    let frame = Instance::new("Frame");
    frame.Size = UDim2::new(0, 200, 0, 200);
    frame.Position = UDim2::new(0.5, -100, 0.5, -100);
    frame.BackgroundColor3 = Color3::new(0.2, 0.2, 0.2);
    frame.Parent = screen_gui;
    
    // Add a title
    let title = Instance::new("TextLabel");
    title.Text = "Roblox-RS Demo";
    title.Size = UDim2::new(1, 0, 0, 30);
    title.TextColor3 = Color3::new(1, 1, 1);
    title.Parent = frame;
    
    // Add to player
    screen_gui.Parent = Players.LocalPlayer.PlayerGui;
}

fn main() {
    setup_ui();
}
"#).expect("Failed to write client file");
    
    // Simple server game controller
    fs::write(&server_file, r#"
// Server game controller
struct GameState {
    active_players: i32,
    game_running: bool,
    score: i32,
}

impl GameState {
    fn new() -> Self {
        GameState {
            active_players: 0,
            game_running: false,
            score: 0,
        }
    }
    
    fn start_game(&mut self) {
        self.game_running = true;
        println!("Game started with {} players", self.active_players);
    }
    
    fn add_player(&mut self) {
        self.active_players += 1;
    }
    
    fn remove_player(&mut self) {
        self.active_players -= 1;
        if self.active_players < 0 {
            self.active_players = 0;
        }
    }
}

fn main() {
    let mut game = GameState::new();
    
    // Set up player joined event
    Players.PlayerAdded.Connect(|player| {
        game.add_player();
        println!("Player joined: {}", player.Name);
        
        if game.active_players >= 2 && !game.game_running {
            game.start_game();
        }
    });
    
    // Set up player left event
    Players.PlayerRemoving.Connect(|player| {
        game.remove_player();
        println!("Player left: {}", player.Name);
    });
}
"#).expect("Failed to write server file");
    
    // Shared game types
    fs::write(&shared_file, r#"
// Shared game types
enum GameMode {
    Casual,
    Competitive,
    Creative,
}

struct PlayerStats {
    points: i32,
    wins: i32,
    losses: i32,
}

impl PlayerStats {
    fn new() -> Self {
        PlayerStats {
            points: 0,
            wins: 0,
            losses: 0,
        }
    }
    
    fn add_win(&mut self, points: i32) {
        self.wins += 1;
        self.points += points;
    }
    
    fn add_loss(&mut self) {
        self.losses += 1;
    }
    
    fn get_win_rate(&self) -> f32 {
        if self.wins + self.losses == 0 {
            return 0.0;
        }
        
        self.wins as f32 / (self.wins + self.losses) as f32
    }
}
"#).expect("Failed to write shared file");
    
    // Create place file generator
    let place_file = PlaceFile::new(
        &project_dir,
        &output_dir,
        "RobloxRSDemo"
    );
    
    // Generate place file
    let result = place_file.generate();
    assert!(result.is_ok(), "Place file generation failed: {:?}", result.err());
    
    let rbxlx_path = result.unwrap();
    assert!(rbxlx_path.exists(), "Place file was not created");
    
    // Check if place file content is valid XML
    let content = fs::read_to_string(&rbxlx_path).expect("Failed to read place file");
    assert!(content.starts_with("<?xml"), "Place file is not valid XML");
    assert!(content.contains("<roblox"), "Place file doesn't contain Roblox XML tag");
    
    // Check for expected services and scripts
    assert!(content.contains("StarterPlayer"), "Place file doesn't contain StarterPlayer service");
    assert!(content.contains("ServerScriptService"), "Place file doesn't contain ServerScriptService");
    assert!(content.contains("ReplicatedStorage"), "Place file doesn't contain ReplicatedStorage");
    
    println!("✅ Place file generation test passed!");
}

fn main() {
    println!("Running place generator test for Roblox-RS 1.0 speed run...");
    test_place_generator();
    println!("All tests passed!");
}
