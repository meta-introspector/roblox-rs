//! Test to verify our reorganized project structure
//! This test ensures all components work together correctly after reorganization

use std::path::Path;
use std::fs;

#[test]
fn test_directory_structure() {
    // Verify that our new directory structure exists
    
    // Core components
    assert!(Path::new("src/runtime/actor").is_dir(), "Actor directory missing");
    assert!(Path::new("src/runtime/instance").is_dir(), "Instance directory missing");
    assert!(Path::new("src/runtime/networking").is_dir(), "Networking directory missing");
    
    // Module files
    assert!(Path::new("src/runtime/actor/mod.rs").is_file(), "Actor module file missing");
    assert!(Path::new("src/runtime/instance/mod.rs").is_file(), "Instance module file missing");
    assert!(Path::new("src/runtime/networking/mod.rs").is_file(), "Networking module file missing");
    assert!(Path::new("src/runtime/debug.rs").is_file(), "Debug module file missing");
    assert!(Path::new("src/runtime/debugger.rs").is_file(), "Debugger module file missing");
    assert!(Path::new("src/runtime/future.rs").is_file(), "Future module file missing");
    
    println!("✅ Directory structure verified successfully!");
}

#[test]
fn test_runtime_generation() {
    // Import the runtime generator
    use roblox_rs_core::runtime::generate_runtime_lib;
    
    // Generate the runtime library
    let runtime = generate_runtime_lib();
    
    // Verify that it includes key components
    assert!(runtime.contains("RobloxRS.Actors"), "Missing actor system");
    assert!(runtime.contains("RobloxRS.Instance"), "Missing instance library");
    assert!(runtime.contains("RobloxRS.Net"), "Missing networking library");
    assert!(runtime.contains("RobloxRS.Debug"), "Missing debug module");
    assert!(runtime.contains("RobloxRS.Debugger"), "Missing debugger module");
    assert!(runtime.contains("RobloxRS.Future"), "Missing future module");
    
    println!("✅ Runtime generation verified successfully!");
}

#[test]
fn test_actor_system() {
    // Import the actor system generator
    use roblox_rs_core::runtime::generate_actor_system;
    
    // Generate the actor system
    let actor_system = generate_actor_system();
    
    // Verify basic functionality
    assert!(actor_system.contains("spawn"), "Missing actor spawn function");
    assert!(actor_system.contains("supervise"), "Missing actor supervision");
    assert!(actor_system.contains("createPool"), "Missing actor pool");
    
    println!("✅ Actor system verified successfully!");
}

#[test]
fn test_instance_library() {
    // Import the instance library generator
    use roblox_rs_core::runtime::generate_instance_lib;
    
    // Generate the instance library
    let instance_lib = generate_instance_lib();
    
    // Verify basic functionality
    assert!(instance_lib.contains("RobloxRS.Instance"), "Missing Instance namespace");
    assert!(instance_lib.contains("new"), "Missing Instance.new function");
    assert!(instance_lib.contains("findFirstChild"), "Missing findFirstChild function");
    
    println!("✅ Instance library verified successfully!");
}

#[test]
fn test_networking_library() {
    // Import the networking library generator
    use roblox_rs_core::runtime::generate_networking_lib;
    
    // Generate the networking library
    let networking_lib = generate_networking_lib();
    
    // Verify basic functionality
    assert!(networking_lib.contains("RobloxRS.Net"), "Missing Net namespace");
    assert!(networking_lib.contains("defineEvent"), "Missing event definition function");
    assert!(networking_lib.contains("RobloxRS.Net.RPC"), "Missing RPC system");
    assert!(networking_lib.contains("RobloxRS.Net.Polling"), "Missing polling system");
    
    println!("✅ Networking library verified successfully!");
}

fn main() {
    println!("Running tests to verify the reorganized project structure...");
    
    test_directory_structure();
    test_runtime_generation();
    test_actor_system();
    test_instance_library();
    test_networking_library();
    
    println!("\n✅ All tests passed! Project reorganization was successful!");
}
