//! Runner script for structure tests
//! This script verifies functionality after project reorganization

use std::env;
use std::process::{Command, exit};

fn main() {
    println!("Running structure verification tests...");
    
    // Step 1: Test the runtime library generation
    println!("\n--- Testing Runtime Library Generation ---");
    let runtime_test_result = Command::new("cargo")
        .args(["test", "--test", "structure_test", "test_runtime_generation", "--", "--nocapture"])
        .status()
        .expect("Failed to run runtime test");
    
    if !runtime_test_result.success() {
        println!("❌ Runtime library test failed");
        exit(1);
    }
    
    // Step 2: Test actor system
    println!("\n--- Testing Actor System ---");
    let actor_test_result = Command::new("cargo")
        .args(["test", "--test", "structure_test", "test_actor_system", "--", "--nocapture"])
        .status()
        .expect("Failed to run actor system test");
    
    if !actor_test_result.success() {
        println!("❌ Actor system test failed");
        exit(1);
    }
    
    // Step 3: Test instance library
    println!("\n--- Testing Instance Library ---");
    let instance_test_result = Command::new("cargo")
        .args(["test", "--test", "structure_test", "test_instance_lib", "--", "--nocapture"])
        .status()
        .expect("Failed to run instance library test");
    
    if !instance_test_result.success() {
        println!("❌ Instance library test failed");
        exit(1);
    }
    
    // Step 4: Test networking library
    println!("\n--- Testing Networking Library ---");
    let networking_test_result = Command::new("cargo")
        .args(["test", "--test", "structure_test", "test_networking_lib", "--", "--nocapture"])
        .status()
        .expect("Failed to run networking library test");
    
    if !networking_test_result.success() {
        println!("❌ Networking library test failed");
        exit(1);
    }
    
    println!("\n✅ All structure tests passed!");
    println!("The project reorganization was successful and all components are working correctly.");
}
