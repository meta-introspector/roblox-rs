//! Test to verify that the restructured project still works correctly

use roblox_rs_core::runtime::{
    generate_runtime_lib,
    generate_actor_system,
    generate_instance_lib,
    generate_networking_lib
};

#[test]
fn test_runtime_generation() {
    // Generate the runtime library
    let runtime = generate_runtime_lib();
    
    // Verify that it contains all the expected components
    assert!(runtime.contains("RobloxRS.Pool"), "Missing pooling system");
    assert!(runtime.contains("RobloxRS.Actors"), "Missing actor system");
    assert!(runtime.contains("RobloxRS.Instance"), "Missing instance library");
    assert!(runtime.contains("RobloxRS.Net"), "Missing networking library");
    
    println!("✅ Runtime library generated successfully with all components");
}

#[test]
fn test_actor_system() {
    // Generate just the actor system
    let actors = generate_actor_system();
    
    // Verify it contains expected functionality
    assert!(actors.contains("spawn"), "Missing actor spawn function");
    assert!(actors.contains("supervise"), "Missing actor supervision");
    assert!(actors.contains("createPool"), "Missing actor pool functionality");
    
    println!("✅ Actor system generated successfully");
}

#[test]
fn test_instance_lib() {
    // Generate the instance library
    let instance = generate_instance_lib();
    
    // Verify it contains expected functionality
    assert!(instance.contains("RobloxRS.Instance"), "Missing Instance namespace");
    assert!(instance.contains("new"), "Missing Instance.new function");
    assert!(instance.contains("findFirstChild"), "Missing findFirstChild function");
    assert!(instance.contains("connect"), "Missing event connection function");
    
    println!("✅ Instance library generated successfully");
}

#[test]
fn test_networking_lib() {
    // Generate the networking library
    let networking = generate_networking_lib();
    
    // Verify it contains all expected components
    assert!(networking.contains("RobloxRS.Net"), "Missing Net namespace");
    assert!(networking.contains("defineEvent"), "Missing event definition function");
    assert!(networking.contains("RobloxRS.Net.RPC"), "Missing RPC system");
    assert!(networking.contains("RobloxRS.Net.Polling"), "Missing polling system");
    assert!(networking.contains("RobloxRS.Net.Middleware"), "Missing middleware system");
    
    // Check for buffer system
    assert!(networking.contains("Buffer"), "Missing buffer system");
    assert!(networking.contains("writeInt"), "Missing buffer write functions");
    assert!(networking.contains("readInt"), "Missing buffer read functions");
    
    println!("✅ Networking library generated successfully");
}

// Run all tests to verify the reorganized structure works
fn main() {
    println!("Running structure tests to verify reorganized project...");
    
    test_runtime_generation();
    test_actor_system();
    test_instance_lib();
    test_networking_lib();
    
    println!("All tests passed! The reorganized structure is working correctly.");
}
