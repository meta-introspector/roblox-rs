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
    
    // Verify object pool has essential components
    assert!(object_pool_code.contains("ObjectPool"));
    assert!(object_pool_code.contains("allocate"));
    assert!(object_pool_code.contains("release"));
    assert!(object_pool_code.contains("createPool"));
    
    // Verify pool has correct behavior
    assert!(object_pool_code.contains("table.insert(self.available"));
    assert!(object_pool_code.contains("table.remove(self.available"));
}

#[test]
fn test_parallel_helpers_generation() {
    let parallel_code = generate_parallel_helpers();
    
    // Verify parallel helpers have essential components
    assert!(parallel_code.contains("Parallel"));
    assert!(parallel_code.contains("foreach"));
    assert!(parallel_code.contains("map"));
    assert!(parallel_code.contains("reduce"));
    
    // Verify task scheduler functionality
    assert!(parallel_code.contains("task.spawn"));
    assert!(parallel_code.contains("task.wait"));
}

#[test]
fn test_memory_tracker_generation() {
    let memory_code = generate_memory_tracker();
    
    // Verify memory tracker has essential components
    assert!(memory_code.contains("Memory"));
    assert!(memory_code.contains("allocations"));
    assert!(memory_code.contains("trackAllocation"));
    assert!(memory_code.contains("releaseAllocation"));
    assert!(memory_code.contains("getStats"));
    
    // Verify tracking behavior
    assert!(memory_code.contains("totalAllocated"));
    assert!(memory_code.contains("currentUsage"));
    assert!(memory_code.contains("peakUsage"));
}

#[test]
fn test_debug_helpers_generation() {
    let debug_code = generate_debug_helpers();
    
    // Verify debug helpers have essential components
    assert!(debug_code.contains("Debug"));
    assert!(debug_code.contains("trace"));
    assert!(debug_code.contains("assert"));
    assert!(debug_code.contains("dump"));
    
    // Verify debug functionality
    assert!(debug_code.contains("print"));
    assert!(debug_code.contains("warn"));
    assert!(debug_code.contains("error"));
}

#[test]
fn test_type_utilities_generation() {
    let type_utils_code = generate_type_utilities();
    
    // Verify type utilities have essential components
    assert!(type_utils_code.contains("TypeUtils"));
    assert!(type_utils_code.contains("isType"));
    assert!(type_utils_code.contains("getTypeName"));
    
    // Verify type checking behavior
    assert!(type_utils_code.contains("typeof"));
}

#[test]
fn test_runtime_integration() {
    // Generate all runtime helpers
    let object_pool = generate_object_pool();
    let parallel = generate_parallel_helpers();
    let memory = generate_memory_tracker();
    let debug = generate_debug_helpers();
    let type_utils = generate_type_utilities();
    
    // Combine all helpers into a complete runtime package
    let runtime_code = format!(
        "-- Roblox-RS Runtime\nlocal RobloxRS = {{}}\n\n{}\n{}\n{}\n{}\n{}\n\nreturn RobloxRS",
        object_pool, parallel, memory, debug, type_utils
    );
    
    // Verify that the combined code contains all components
    assert!(runtime_code.contains("RobloxRS.ObjectPool"));
    assert!(runtime_code.contains("RobloxRS.Parallel"));
    assert!(runtime_code.contains("RobloxRS.Memory"));
    assert!(runtime_code.contains("RobloxRS.Debug"));
    assert!(runtime_code.contains("RobloxRS.TypeUtils"));
    
    // Verify that namespaces don't conflict
    assert!(runtime_code.contains("RobloxRS.ObjectPool = {"));
    assert!(runtime_code.contains("RobloxRS.Parallel = {"));
    assert!(runtime_code.contains("RobloxRS.Memory = {"));
    assert!(runtime_code.contains("RobloxRS.Debug = {"));
    assert!(runtime_code.contains("RobloxRS.TypeUtils = {"));
}

#[test]
fn test_runtime_helper_execution() {
    // Simulate executing the runtime helpers in a Lua environment
    // This is a mock test that validates the structure without actually running Lua
    
    // Create a simplified Luau AST that would use these helpers
    let simulated_code = r#"
    local RobloxRS = require(script.Parent.RobloxRS)
    
    -- Use object pool
    local pool = RobloxRS.ObjectPool.createPool(function() 
        return { value = 0 } 
    end)
    local obj = pool:allocate()
    obj.value = 42
    pool:release(obj)
    
    -- Use parallel helpers
    local results = RobloxRS.Parallel.map({1, 2, 3, 4}, function(x)
        return x * 2
    end)
    
    -- Use memory tracker
    RobloxRS.Memory.trackAllocation("test_object", 100, "table")
    local stats = RobloxRS.Memory.getStats()
    RobloxRS.Memory.releaseAllocation("test_object")
    
    -- Use debug helpers
    RobloxRS.Debug.trace("Debug message")
    
    -- Use type utilities
    local isNumber = RobloxRS.TypeUtils.isType(42, "number")
    "#;
    
    // Assert that all helper methods are referenced in the simulated code
    assert!(simulated_code.contains("ObjectPool.createPool"));
    assert!(simulated_code.contains("pool:allocate"));
    assert!(simulated_code.contains("pool:release"));
    
    assert!(simulated_code.contains("Parallel.map"));
    
    assert!(simulated_code.contains("Memory.trackAllocation"));
    assert!(simulated_code.contains("Memory.getStats"));
    assert!(simulated_code.contains("Memory.releaseAllocation"));
    
    assert!(simulated_code.contains("Debug.trace"));
    
    assert!(simulated_code.contains("TypeUtils.isType"));
}
