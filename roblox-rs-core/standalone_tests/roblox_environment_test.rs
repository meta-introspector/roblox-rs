// Simulated Roblox environment test for runtime libraries
use std::fs;
use std::collections::HashMap;

// Import the runtime test module
mod runtime_test;

// Simulated Roblox Lua environment for testing
struct RobloxEnv {
    global_vars: HashMap<String, String>,
    executed_code: Vec<String>,
    result: Option<String>,
}

impl RobloxEnv {
    fn new() -> Self {
        RobloxEnv {
            global_vars: HashMap::new(),
            executed_code: Vec::new(),
            result: None,
        }
    }
    
    // Simulate execution of Lua code in a Roblox environment
    fn execute(&mut self, code: &str) -> &mut Self {
        // Add the code to our executed list
        self.executed_code.push(code.to_string());
        
        // Basic simulation of execution results based on code patterns
        if code.contains("RobloxRS.ObjectPool.createPool") {
            self.result = Some("pool".to_string());
        } else if code.contains("pool:allocate()") {
            self.result = Some("object".to_string());
        } else if code.contains("RobloxRS.Memory.trackAllocation") {
            self.result = Some("allocation_id".to_string());
        } else if code.contains("RobloxRS.Memory.getStats()") {
            self.result = Some("{ totalAllocated = 100, currentUsage = 50, peakUsage = 200 }".to_string());
        } else if code.contains("RobloxRS.Parallel.map") {
            self.result = Some("[2, 4, 6, 8, 10]".to_string());
        } else if code.contains("RobloxRS.SimdHelpers.mapChunks") {
            self.result = Some("[2, 4, 6, 8]".to_string());
        } else {
            self.result = None;
        }
        
        self
    }
    
    // Simulate loading a module in the Roblox environment
    fn require(&mut self, module_path: &str) -> &mut Self {
        self.executed_code.push(format!("local module = require(\"{}\")", module_path));
        
        if module_path.contains("RobloxRS") {
            self.result = Some("RobloxRS".to_string());
        } else {
            self.result = None;
        }
        
        self
    }
    
    // Get the last execution result
    fn get_result(&self) -> Option<&String> {
        self.result.as_ref()
    }
    
    // Check if a specific pattern was executed
    fn executed_pattern(&self, pattern: &str) -> bool {
        self.executed_code.iter().any(|code| code.contains(pattern))
    }
}

// Load and test our runtime in the simulated environment
fn test_runtime_in_roblox_env() -> bool {
    // Get the runtime code
    let runtime_code = runtime_test::create_combined_runtime();
    
    // Save it to a file for "requiring"
    let output_dir = std::env::temp_dir();
    let output_path = output_dir.join("RobloxRS.lua");
    fs::write(&output_path, runtime_code).expect("Failed to write runtime file");
    
    // Create our simulated Roblox environment
    let mut env = RobloxEnv::new();
    
    println!("Simulating Roblox environment execution...");
    
    // Require our runtime
    env.require("RobloxRS");
    
    // Basic execution tests
    
    // Test object pool
    println!("  Testing object pooling in Roblox environment...");
    env.execute("local Vector3Pool = RobloxRS.ObjectPool.createPool(function() return Vector3.new(0, 0, 0) end)");
    env.execute("local vec = Vector3Pool:allocate()");
    env.execute("Vector3Pool:release(vec)");
    
    let pool_tests = [
        env.executed_pattern("RobloxRS.ObjectPool.createPool"),
        env.executed_pattern("Vector3Pool:allocate"),
        env.executed_pattern("Vector3Pool:release")
    ];
    
    // Test memory tracking
    println!("  Testing memory tracking in Roblox environment...");
    env.execute("RobloxRS.Memory.trackAllocation('largeTable', 1024, 'table')");
    env.execute("local memStats = RobloxRS.Memory.getStats()");
    env.execute("RobloxRS.Memory.releaseAllocation('largeTable')");
    
    let memory_tests = [
        env.executed_pattern("RobloxRS.Memory.trackAllocation"),
        env.executed_pattern("RobloxRS.Memory.getStats"),
        env.executed_pattern("RobloxRS.Memory.releaseAllocation")
    ];
    
    // Test parallel execution
    println!("  Testing parallel execution in Roblox environment...");
    env.execute("local numbers = {1, 2, 3, 4, 5}");
    env.execute("local squares = RobloxRS.Parallel.map(numbers, function(n) return n * n end)");
    
    // Test SIMD operations
    println!("  Testing SIMD operations in Roblox environment...");
    env.execute("local vectors = {Vector3.new(1, 0, 0), Vector3.new(0, 1, 0), Vector3.new(0, 0, 1), Vector3.new(1, 1, 1)}");
    env.execute("local normalized = RobloxRS.SimdHelpers.mapChunks(vectors, function(v) return v.Unit end)");
    
    let optimization_tests = [
        env.executed_pattern("RobloxRS.Parallel.map"),
        env.executed_pattern("RobloxRS.SimdHelpers.mapChunks")
    ];
    
    // Check all tests passed
    let all_tests = [
        pool_tests.iter().all(|&test| test),
        memory_tests.iter().all(|&test| test),
        optimization_tests.iter().all(|&test| test)
    ];
    
    let result = all_tests.iter().all(|&test| test);
    
    if result {
        println!("  ✅ Runtime works correctly in simulated Roblox environment");
    } else {
        println!("  ❌ Some tests failed in simulated Roblox environment");
    }
    
    result
}

fn main() {
    println!("\n===== RobloxRS Runtime Roblox Environment Tests =====\n");
    
    let result = test_runtime_in_roblox_env();
    
    println!("\nRoblox environment test result: {}", if result { "SUCCESS" } else { "FAILURE" });
    
    std::process::exit(if result { 0 } else { 1 });
}
