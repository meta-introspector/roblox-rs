//! Test for the Roblox-RS transpiler
//! This test validates our core transpiler functionality for the 1.0 speed run

use std::path::Path;
use std::fs;
use roblox_rs_core::transpiler;

const TEST_RUST_CODE: &str = r#"
// Simple Rust test code
use std::collections::HashMap;

struct Player {
    name: String,
    health: i32,
    inventory: HashMap<String, i32>,
}

impl Player {
    // Constructor
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            health: 100,
            inventory: HashMap::new(),
        }
    }
    
    // Method to take damage
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }
    
    // Add item to inventory
    fn add_item(&mut self, item: &str, count: i32) {
        *self.inventory.entry(item.to_string()).or_insert(0) += count;
    }
    
    // Check if player is alive
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    // Create a new player
    let mut player = Player::new("RobloxGamer");
    
    // Test player functionality
    player.add_item("Gold", 50);
    player.add_item("Potion", 3);
    
    println!("Player {} has {} health", player.name, player.health);
    
    // Player takes damage
    player.take_damage(30);
    println!("After taking damage: {} health", player.health);
    
    if player.is_alive() {
        println!("Player is still alive!");
    }
}
"#;

#[test]
fn test_basic_transpilation() {
    // Transpile the test code
    let result = transpiler::transpile(TEST_RUST_CODE);
    assert!(result.is_ok(), "Transpilation failed: {:?}", result.err());
    
    let luau_code = result.unwrap();
    
    // Basic verification - check for expected Luau patterns
    assert!(luau_code.contains("local Player = {}"), "Missing Player class definition");
    assert!(luau_code.contains("function Player.new"), "Missing constructor");
    assert!(luau_code.contains("function Player.take_damage"), "Missing take_damage method");
    assert!(luau_code.contains("function Player.add_item"), "Missing add_item method");
    assert!(luau_code.contains("function Player.is_alive"), "Missing is_alive method");
    
    println!("Generated Luau code:\n{}", luau_code);
    println!("✅ Basic transpilation test passed!");
}

#[test]
fn test_file_transpilation() {
    // Create a temporary directory for the test
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    
    // Input file path
    let input_path = temp_dir.path().join("player.rs");
    
    // Write test code to the input file
    fs::write(&input_path, TEST_RUST_CODE).expect("Failed to write test file");
    
    // Output file path
    let output_path = temp_dir.path().join("player.lua");
    
    // Transpile the file
    transpiler::transpile_file(&input_path, &output_path).expect("Failed to transpile file");
    
    // Check that the output file exists
    assert!(output_path.exists(), "Output file was not created");
    
    // Read the output file
    let luau_code = fs::read_to_string(output_path).expect("Failed to read output file");
    
    // Basic verification - check for expected Luau patterns
    assert!(luau_code.contains("local Player = {}"), "Missing Player class definition");
    assert!(luau_code.contains("function Player.new"), "Missing constructor");
    
    println!("✅ File transpilation test passed!");
}

#[test]
fn test_directory_transpilation() {
    // Create a temporary directory structure for the test
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let src_dir = temp_dir.path().join("src");
    let client_dir = src_dir.join("client");
    let server_dir = src_dir.join("server");
    let shared_dir = src_dir.join("shared");
    
    // Create the directory structure
    fs::create_dir_all(&client_dir).expect("Failed to create client directory");
    fs::create_dir_all(&server_dir).expect("Failed to create server directory");
    fs::create_dir_all(&shared_dir).expect("Failed to create shared directory");
    
    // Create test files
    fs::write(shared_dir.join("player.rs"), TEST_RUST_CODE).expect("Failed to write player.rs");
    fs::write(client_dir.join("ui.rs"), "fn setup_ui() { println!(\"Setting up UI\"); }")
        .expect("Failed to write ui.rs");
    fs::write(server_dir.join("game.rs"), "fn start_game() { println!(\"Starting game\"); }")
        .expect("Failed to write game.rs");
    
    // Output directory
    let out_dir = temp_dir.path().join("out");
    
    // Transpile the directory
    transpiler::transpile_directory(&src_dir, &out_dir, "rs").expect("Failed to transpile directory");
    
    // Check that the output directories and files exist
    assert!(out_dir.exists(), "Output directory was not created");
    assert!(out_dir.join("client").exists(), "Client output directory was not created");
    assert!(out_dir.join("server").exists(), "Server output directory was not created");
    assert!(out_dir.join("shared").exists(), "Shared output directory was not created");
    
    assert!(out_dir.join("shared/player.lua").exists(), "player.lua was not created");
    assert!(out_dir.join("client/ui.lua").exists(), "ui.lua was not created");
    assert!(out_dir.join("server/game.lua").exists(), "game.lua was not created");
    
    println!("✅ Directory transpilation test passed!");
}

fn main() {
    // Run all tests
    println!("Running transpiler tests for Roblox-RS 1.0 speed run...\n");
    
    test_basic_transpilation();
    test_file_transpilation();
    test_directory_transpilation();
    
    println!("\n✅ All transpiler tests passed!");
}
