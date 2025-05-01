//! Integration test for Roblox-RS 1.0
//! Tests all major components working together: transpiler, runtime, packaging

use std::path::Path;
use std::fs;
use tempfile;
use roblox_rs_core::transpiler;
use roblox_rs_core::runtime;
use roblox_rs_core::packaging;

const TEST_RUST_CODE: &str = r#"
// Sample Roblox-RS game component
struct Player {
    name: String,
    health: i32,
    speed: f32,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            health: 100,
            speed: 16.0,
        }
    }
    
    fn damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
    
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn create_ui() {
    // Create a basic UI for the player
    let screen_gui = Instance::new("ScreenGui");
    let frame = Instance::new("Frame");
    frame.Size = UDim2::new(0, 200, 0, 100);
    frame.Position = UDim2::new(0.5, -100, 0.1, 0);
    frame.BackgroundColor3 = Color3::new(0.2, 0.2, 0.2);
    frame.Parent = screen_gui;
    
    let title = Instance::new("TextLabel");
    title.Text = "Roblox-RS Demo";
    title.Size = UDim2::new(1, 0, 0, 30);
    title.TextColor3 = Color3::new(1, 1, 1);
    title.Parent = frame;
    
    let health_label = Instance::new("TextLabel");
    health_label.Text = "Health: 100";
    health_label.Size = UDim2::new(1, 0, 0, 30);
    health_label.Position = UDim2::new(0, 0, 0, 30);
    health_label.TextColor3 = Color3::new(0, 1, 0);
    health_label.Parent = frame;
    
    screen_gui.Parent = Players.LocalPlayer.PlayerGui;
}

fn main() {
    // Test actor system
    let player_actor = Actors::spawn(|mailbox| {
        let mut player = Player::new("TestPlayer");
        
        while let Some(msg) = mailbox.receive() {
            match msg.as_str() {
                "damage" => player.damage(10),
                "heal" => player.health += 20,
                "status" => println!("Player status: HP={}", player.health),
                _ => {}
            }
        }
    });
    
    // Test networking
    let damage_event = Net::defineEvent("PlayerDamaged", {
        player_id: "string",
        damage: "int",
        position: "Vector3"
    });
    
    damage_event.listen(|data| {
        println!("Player {} took {} damage at position {}", 
                data.player_id, data.damage, data.position);
    });
    
    // Test instance API
    create_ui();
}
"#;

#[test]
fn test_transpilation_with_runtime() {
    // Create temporary directory for test
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test_game.rs");
    
    // Write test code to file
    fs::write(&test_file, TEST_RUST_CODE).expect("Failed to write test file");
    
    // Transpile the code
    let output_file = temp_dir.path().join("test_game.lua");
    transpiler::transpile_file(&test_file, &output_file).expect("Failed to transpile file");
    
    // Verify the output file exists
    assert!(output_file.exists(), "Output file was not created");
    
    // Read the transpiled code
    let lua_code = fs::read_to_string(&output_file).expect("Failed to read output file");
    
    // Check for expected patterns in the output
    assert!(lua_code.contains("local Player = {}"), "Missing Player class definition");
    assert!(lua_code.contains("function Player.new"), "Missing constructor");
    assert!(lua_code.contains("function Player:damage"), "Missing damage method");
    assert!(lua_code.contains("function Player:is_alive"), "Missing is_alive method");
    
    // Check actor system usage
    assert!(lua_code.contains("Actors:spawn"), "Missing actor system usage");
    assert!(lua_code.contains("mailbox.receive"), "Missing actor mailbox usage");
    
    // Check networking usage
    assert!(lua_code.contains("Net:defineEvent"), "Missing networking usage");
    assert!(lua_code.contains("PlayerDamaged"), "Missing event definition");
    
    // Check instance API usage
    assert!(lua_code.contains("Instance.new"), "Missing instance creation");
    assert!(lua_code.contains("ScreenGui"), "Missing UI elements");
    
    println!("✅ Transpilation with runtime test passed!");
}

#[test]
fn test_runtime_generation() {
    // Generate runtime library
    let runtime_lib = runtime::generate_runtime_lib();
    
    // Check runtime components
    assert!(runtime_lib.contains("RobloxRS.Actors"), "Missing actor system");
    assert!(runtime_lib.contains("RobloxRS.Instance"), "Missing instance library");
    assert!(runtime_lib.contains("RobloxRS.Net"), "Missing networking library");
    
    // Check expected actor system functionality
    assert!(runtime_lib.contains("ActorSystem.spawn"), "Missing actor spawn function");
    assert!(runtime_lib.contains("ActorHandle"), "Missing actor handle");
    
    // Check expected instance functionality
    assert!(runtime_lib.contains("RobloxRS.Instance.new"), "Missing instance creation");
    assert!(runtime_lib.contains("RobloxRS.Instance.findFirstChild"), "Missing findFirstChild");
    
    // Check expected networking functionality
    assert!(runtime_lib.contains("RobloxRS.Net.Buffer"), "Missing buffer system");
    assert!(runtime_lib.contains("RobloxRS.Net.defineEvent"), "Missing event definition");
    
    println!("✅ Runtime generation test passed!");
}

#[test]
fn test_full_project_generation() {
    // Create a temporary project structure
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let project_dir = temp_dir.path().join("test_project");
    
    // Create project directories
    let client_dir = project_dir.join("client");
    let server_dir = project_dir.join("server");
    let shared_dir = project_dir.join("shared");
    let output_dir = temp_dir.path().join("output");
    
    fs::create_dir_all(&client_dir).expect("Failed to create client directory");
    fs::create_dir_all(&server_dir).expect("Failed to create server directory");
    fs::create_dir_all(&shared_dir).expect("Failed to create shared directory");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    
    // Create sample files
    fs::write(client_dir.join("ui.rs"), "fn setup_ui() { println!(\"Setting up UI\"); }")
        .expect("Failed to write client file");
    fs::write(server_dir.join("game.rs"), "fn start_game() { println!(\"Starting game\"); }")
        .expect("Failed to write server file");
    fs::write(shared_dir.join("types.rs"), "struct GameState { score: i32 }")
        .expect("Failed to write shared file");
    
    // Create place file
    let place_file = packaging::place_gen::PlaceFile::new(
        &project_dir,
        &output_dir,
        "TestGame"
    );
    
    // Generate place file
    let result = place_file.generate();
    assert!(result.is_ok(), "Place file generation failed: {:?}", result.err());
    
    let place_path = result.unwrap();
    assert!(place_path.exists(), "Place file does not exist");
    
    println!("✅ Full project generation test passed!");
}

fn main() {
    println!("Running Roblox-RS 1.0 integration tests...");
    
    test_transpilation_with_runtime();
    test_runtime_generation();
    test_full_project_generation();
    
    println!("\n🎉 All integration tests passed! Roblox-RS 1.0 is working as expected!");
}
