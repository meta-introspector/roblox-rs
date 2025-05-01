/// Simple tests for the runtime module without external dependencies
use roblox_rs_core::runtime::{
    generate_object_pool,
    generate_parallel_helpers,
    generate_memory_tracker,
    generate_debug_helpers,
    generate_type_utilities
};

#[test]
fn test_object_pool_generation() {
    let object_pool_code = generate_object_pool();
    
    // Verify essential parts exist
    assert!(object_pool_code.contains("RobloxRS.ObjectPool"));
    assert!(object_pool_code.contains("allocate"));
    assert!(object_pool_code.contains("release"));
}

#[test]
fn test_memory_tracker_generation() {
    let memory_code = generate_memory_tracker();
    
    // Verify memory tracking and stats
    assert!(memory_code.contains("RobloxRS.Memory"));
    assert!(memory_code.contains("trackAllocation"));
    assert!(memory_code.contains("releaseAllocation"));
    assert!(memory_code.contains("totalAllocated"));
    assert!(memory_code.contains("currentUsage"));
}

#[test]
fn test_parallel_helpers_generation() {
    let parallel_code = generate_parallel_helpers();
    
    // Verify parallel features
    assert!(parallel_code.contains("RobloxRS.Parallel"));
    assert!(parallel_code.contains("forEach"));
    assert!(parallel_code.contains("map"));
    assert!(parallel_code.contains("reduce"));
}

#[test]
fn test_debug_helpers_generation() {
    let debug_code = generate_debug_helpers();
    
    // Verify debug features
    assert!(debug_code.contains("RobloxRS.Debug"));
    assert!(debug_code.contains("track"));
    assert!(debug_code.contains("trace"));
    assert!(debug_code.contains("getStackTrace"));
}

#[test]
fn test_type_utilities_generation() {
    let type_utils_code = generate_type_utilities();
    
    // Verify type utilities
    assert!(type_utils_code.contains("RobloxRS.Type"));
    assert!(type_utils_code.contains("isType"));
    assert!(type_utils_code.contains("getType"));
}

#[test]
fn test_combined_runtime_generation() {
    // Generate all parts of the runtime
    let object_pool = generate_object_pool();
    let memory = generate_memory_tracker();
    let parallel = generate_parallel_helpers();
    let debug = generate_debug_helpers();
    let type_utils = generate_type_utilities();
    
    // Combine into a single runtime library
    let runtime = format!(
        "-- RobloxRS Runtime Library\nlocal RobloxRS = {{}}\n\n{}\n{}\n{}\n{}\n{}\n\nreturn RobloxRS",
        object_pool, memory, parallel, debug, type_utils
    );
    
    // Verify everything is included and properly namespaced
    assert!(runtime.contains("RobloxRS.ObjectPool"));
    assert!(runtime.contains("RobloxRS.Memory"));
    assert!(runtime.contains("RobloxRS.Parallel"));
    assert!(runtime.contains("RobloxRS.Debug"));
    assert!(runtime.contains("RobloxRS.Type"));
    
    // Make sure it's properly structured as a Lua module
    assert!(runtime.starts_with("-- RobloxRS Runtime Library"));
    assert!(runtime.contains("local RobloxRS = {}"));
    assert!(runtime.ends_with("return RobloxRS"));
}
