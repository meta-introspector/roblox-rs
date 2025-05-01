/// Integration test for dependencies and runtime helpers
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

// Import the runtime test module
mod runtime_test;

// Mock dependency structure
struct Dependency {
    name: String,
    version: String,
    content: String,
}

// Mock dependency system
struct DependencyManager {
    dependencies: Vec<Dependency>,
    output_path: PathBuf,
}

impl DependencyManager {
    fn new(output_path: PathBuf) -> Self {
        DependencyManager {
            dependencies: Vec::new(),
            output_path,
        }
    }
    
    fn add_dependency(&mut self, name: &str, version: &str, content: &str) {
        self.dependencies.push(Dependency {
            name: name.to_string(),
            version: version.to_string(),
            content: content.to_string(),
        });
    }
    
    fn scan_dependencies(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for dep in &self.dependencies {
            result.insert(dep.name.clone(), dep.version.clone());
        }
        result
    }
    
    fn output_dependencies(&self) -> Result<(), String> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_path).map_err(|e| format!("Failed to create output directory: {}", e))?;
        
        // Write each dependency to a file
        for dep in &self.dependencies {
            let file_path = self.output_path.join(format!("{}.lua", dep.name));
            fs::write(&file_path, &dep.content).map_err(|e| format!("Failed to write dependency file: {}", e))?;
        }
        
        // Generate and write the index file
        let mut index_content = String::from("-- Generated index file\nlocal Packages = {}\n\n");
        
        for dep in &self.dependencies {
            index_content.push_str(&format!("Packages[\"{0}\"] = require(\"{0}\")\n", dep.name));
        }
        
        index_content.push_str("\nreturn Packages");
        
        let index_path = self.output_path.join("init.lua");
        fs::write(&index_path, index_content).map_err(|e| format!("Failed to write index file: {}", e))?;
        
        Ok(())
    }
}

// Test integration between dependency system and runtime helpers
fn test_dependency_runtime_integration() -> bool {
    println!("Testing dependency system with runtime helpers integration...");
    
    // Create temporary output directory
    let output_dir = std::env::temp_dir().join("roblox_rs_test");
    let _ = fs::remove_dir_all(&output_dir); // Clean up any previous test run
    fs::create_dir_all(&output_dir).expect("Failed to create test directory");
    
    // Initialize dependency manager
    let mut manager = DependencyManager::new(output_dir.clone());
    
    // Generate runtime code
    let object_pool = runtime_test::generate_object_pool();
    let memory_tracker = runtime_test::generate_memory_tracker();
    let parallel_helpers = runtime_test::generate_parallel_helpers();
    let simd_helpers = runtime_test::generate_simd_helpers();
    
    // Add runtime components as dependencies
    manager.add_dependency("ObjectPool", "1.0.0", &object_pool);
    manager.add_dependency("MemoryTracker", "1.0.0", &memory_tracker);
    manager.add_dependency("ParallelHelpers", "1.0.0", &parallel_helpers);
    manager.add_dependency("SimdHelpers", "1.0.0", &simd_helpers);
    
    // Add a mock user code that uses the runtime helpers
    let user_code = r#"-- User code that uses runtime helpers
local ObjectPool = require("ObjectPool")
local MemoryTracker = require("MemoryTracker")
local ParallelHelpers = require("ParallelHelpers")
local SimdHelpers = require("SimdHelpers")

-- Create a pool for Vector3 objects
local Vector3Pool = ObjectPool.createPool(function()
    return Vector3.new(0, 0, 0)
end)

-- Track memory usage
MemoryTracker.trackAllocation("largeTable", 1024, "table")

-- Process data in parallel
local numbers = {1, 2, 3, 4, 5, 6, 7, 8}
local squares = ParallelHelpers.map(numbers, function(n)
    return n * n
end)

-- Use SIMD operations for faster vector math
local vectors = {
    Vector3.new(1, 0, 0),
    Vector3.new(0, 1, 0),
    Vector3.new(0, 0, 1),
    Vector3.new(1, 1, 1)
}

local normalized = SimdHelpers.mapChunks(vectors, function(v)
    return v.Unit
end)

return {
    vectorPool = Vector3Pool,
    squares = squares,
    vectors = normalized
}
"#;
    
    manager.add_dependency("UserCode", "1.0.0", user_code);
    
    // Output all dependencies
    match manager.output_dependencies() {
        Ok(_) => {
            println!("  ✓ Successfully output dependencies");
        },
        Err(e) => {
            println!("  ❌ Failed to output dependencies: {}", e);
            return false;
        }
    }
    
    // Verify dependencies were output correctly
    let dependencies = [
        "ObjectPool.lua",
        "MemoryTracker.lua",
        "ParallelHelpers.lua",
        "SimdHelpers.lua",
        "UserCode.lua",
        "init.lua"
    ];
    
    let all_files_exist = dependencies.iter().all(|&file| {
        let file_path = output_dir.join(file);
        let exists = file_path.exists();
        if !exists {
            println!("  ❌ Missing expected file: {}", file);
        }
        exists
    });
    
    // Verify the content of files
    let index_content = fs::read_to_string(output_dir.join("init.lua"))
        .expect("Failed to read index file");
    
    let index_has_references = dependencies[..dependencies.len()-1].iter().all(|&file| {
        let dep_name = file.trim_end_matches(".lua");
        let contains_ref = index_content.contains(&format!("Packages[\"{}\"] = require(\"{}\")", dep_name, dep_name));
        if !contains_ref {
            println!("  ❌ Index file missing reference to {}", dep_name);
        }
        contains_ref
    });
    
    // Verify user code contains required imports
    let user_code_content = fs::read_to_string(output_dir.join("UserCode.lua"))
        .expect("Failed to read user code file");
    
    let user_code_has_imports = [
        "require(\"ObjectPool\")",
        "require(\"MemoryTracker\")",
        "require(\"ParallelHelpers\")",
        "require(\"SimdHelpers\")"
    ].iter().all(|&import| {
        let contains_import = user_code_content.contains(import);
        if !contains_import {
            println!("  ❌ User code missing import: {}", import);
        }
        contains_import
    });
    
    // Final result
    let result = all_files_exist && index_has_references && user_code_has_imports;
    
    if result {
        println!("  ✅ Dependencies and runtime helpers integrated successfully");
    } else {
        println!("  ❌ Integration test failed");
    }
    
    // Clean up test files
    let _ = fs::remove_dir_all(&output_dir);
    
    result
}

fn main() {
    println!("\n===== RobloxRS Dependency-Runtime Integration Test =====\n");
    
    let result = test_dependency_runtime_integration();
    
    println!("\nIntegration test result: {}", if result { "SUCCESS" } else { "FAILURE" });
    
    std::process::exit(if result { 0 } else { 1 });
}
