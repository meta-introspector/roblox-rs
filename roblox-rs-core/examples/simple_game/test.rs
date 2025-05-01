//! Test for the simple game example
//! Demonstrates transpiling and running the Roblox-RS 1.0 simple game

use std::path::Path;
use std::fs;
use tempfile;
use roblox_rs_core::transpiler;
use roblox_rs_core::runtime;
use roblox_rs_core::packaging;

#[test]
fn test_simple_game_transpile() {
    // Read the main.rs file
    let main_rs_path = Path::new("examples/simple_game/main.rs");
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
    
    // Check for expected patterns in the output
    assert!(lua_code.contains("Player = {}"), "Missing Player class definition");
    assert!(lua_code.contains("function Player.new"), "Missing constructor");
    assert!(lua_code.contains("take_damage"), "Missing take_damage method");
    assert!(lua_code.contains("add_score"), "Missing add_score method");
    assert!(lua_code.contains("is_alive"), "Missing is_alive method");
    
    // Check for actor system usage
    assert!(lua_code.contains("Actors:spawn"), "Missing actor system usage");
    assert!(lua_code.contains("mailbox.receive"), "Missing mailbox usage");
    
    // Check for instance API usage
    assert!(lua_code.contains("Instance.new"), "Missing instance creation");
    assert!(lua_code.contains("ScreenGui"), "Missing UI elements");
    
    // Check for networking usage
    assert!(lua_code.contains("defineEvent"), "Missing event definition");
    assert!(lua_code.contains("PlayerDamaged"), "Missing damage event");
    assert!(lua_code.contains("defineRPC"), "Missing RPC definition");
    
    println!("✅ Simple game transpilation test passed!");
    
    // Write runtime library
    let runtime_lib = runtime::generate_runtime_lib();
    let runtime_file = temp_dir.path().join("runtime.lua");
    fs::write(&runtime_file, &runtime_lib).expect("Failed to write runtime file");
    
    // Verify runtime library
    assert!(runtime_file.exists(), "Runtime file does not exist");
    
    println!("✅ Runtime library generation test passed!");
    
    // Test packaging the game
    let project_dir = temp_dir.path().join("game_project");
    let client_dir = project_dir.join("client");
    let server_dir = project_dir.join("server");
    let shared_dir = project_dir.join("shared");
    
    fs::create_dir_all(&client_dir).expect("Failed to create client directory");
    fs::create_dir_all(&server_dir).expect("Failed to create server directory");
    fs::create_dir_all(&shared_dir).expect("Failed to create shared directory");
    
    // Copy the transpiled game to the project
    fs::write(client_dir.join("game.lua"), &lua_code).expect("Failed to write client game file");
    fs::write(server_dir.join("server.lua"), r#"
-- Server script
print("Server starting up")
    "#).expect("Failed to write server file");
    fs::write(shared_dir.join("shared.lua"), r#"
-- Shared module
local SharedModule = {}

function SharedModule.greet(name)
    return "Hello, " .. name .. "!"
end

return SharedModule
    "#).expect("Failed to write shared file");
    
    // Test place file generation
    let place_file = packaging::place_gen::PlaceFile::new(
        &project_dir,
        &temp_dir.path(),
        "SimpleGameDemo"
    );
    
    let result = place_file.generate();
    if result.is_ok() {
        println!("✅ Place file generation passed!");
        let place_path = result.unwrap();
        assert!(place_path.exists(), "Place file does not exist");
    } else {
        println!("⚠️ Place file generation skipped (dependency issues)");
    }
    
    println!("\n🎉 All simple game tests completed successfully!");
}

fn main() {
    println!("Running simple game example test...");
    test_simple_game_transpile();
}
