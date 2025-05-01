//! Test for the simple game example
//! Demonstrates transpiling and running the Roblox-RS 1.0 simple game

use std::path::Path;
use std::fs;
use tempfile;
use roblox_rs_core::transpiler;
use roblox_rs_core::runtime;

#[test]
fn test_simple_game_transpile() {
    // Read the main.rs file from examples directory
    let main_rs_path = Path::new("examples/simple_game/main.rs");
    println!("Looking for file at: {:?}", main_rs_path.canonicalize().unwrap_or_default());
    
    let main_rs = fs::read_to_string(main_rs_path)
        .expect("Failed to read simple_game/main.rs");
    
    // Create a temporary directory for output
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let output_file = temp_dir.path().join("game.lua");
    
    // Transpile the Rust code to Luau
    let result = transpiler::transpile(&main_rs);
    assert!(result.is_ok(), "Failed to transpile simple game: {:?}", result.err());
    
    let lua_code = result.unwrap();
    fs::write(&output_file, &lua_code).expect("Failed to write output file");
    
    // Verify the transpiled code
    assert!(output_file.exists(), "Output file does not exist");
    println!("Transpiled code written to: {:?}", output_file);
    
    // Check for expected patterns in the output
    assert!(lua_code.contains("Player = {}") || lua_code.contains("local Player"), 
            "Missing Player class definition");
    assert!(lua_code.contains("function Player.new") || lua_code.contains("Player.new = function"), 
            "Missing constructor");
    
    // Test generating the runtime library
    let runtime_lib = runtime::generate_runtime_lib();
    assert!(!runtime_lib.is_empty(), "Runtime library is empty");
    
    // Check key components
    assert!(runtime_lib.contains("RobloxRS.Actors"), "Missing actor system");
    assert!(runtime_lib.contains("RobloxRS.Instance"), "Missing instance library");
    assert!(runtime_lib.contains("RobloxRS.Net"), "Missing networking library");
    
    println!("✅ Runtime library generation test passed!");
    println!("✅ Simple game transpilation test passed!");
}
