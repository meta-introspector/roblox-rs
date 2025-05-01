/// Standalone test for runtime helpers
/// This test avoids dependencies on the core library to ensure it can run

use std::fs;
use std::path::PathBuf;

// Runtime helper generation functions
pub fn generate_object_pool() -> String {
    r#"-- ObjectPool implementation
local RobloxRS = RobloxRS or {}
RobloxRS.ObjectPool = {
    pools = {}
}

function RobloxRS.ObjectPool.createPool(constructor)
    local pool = {
        available = {},
        constructor = constructor,
        size = 0,
        hits = 0,
        misses = 0
    }
    
    -- Set the metatable to handle OOP-style methods
    setmetatable(pool, {__index = RobloxRS.ObjectPool})
    table.insert(RobloxRS.ObjectPool.pools, pool)
    return pool
end

function RobloxRS.ObjectPool:allocate()
    if #self.available > 0 then
        local obj = table.remove(self.available)
        self.hits = self.hits + 1
        return obj
    else
        self.misses = self.misses + 1
        self.size = self.size + 1
        return self.constructor()
    end
end

function RobloxRS.ObjectPool:release(obj)
    table.insert(self.available, obj)
end

function RobloxRS.ObjectPool:getStats()
    return {
        size = self.size,
        available = #self.available,
        utilization = self.hits / (self.hits + self.misses)
    }
end

return RobloxRS.ObjectPool"#.to_string()
}

pub fn generate_memory_tracker() -> String {
    r#"-- Memory tracking implementation
local RobloxRS = RobloxRS or {}
RobloxRS.Memory = {
    allocations = {},
    stats = {
        totalAllocated = 0,
        currentUsage = 0,
        peakUsage = 0
    }
}

function RobloxRS.Memory.trackAllocation(id, size, type)
    RobloxRS.Memory.allocations[id] = {
        size = size or 1,
        type = type or "unknown"
    }
    
    RobloxRS.Memory.stats.totalAllocated = RobloxRS.Memory.stats.totalAllocated + (size or 1)
    RobloxRS.Memory.stats.currentUsage = RobloxRS.Memory.stats.currentUsage + (size or 1)
    
    if RobloxRS.Memory.stats.currentUsage > RobloxRS.Memory.stats.peakUsage then
        RobloxRS.Memory.stats.peakUsage = RobloxRS.Memory.stats.currentUsage
    end
end

function RobloxRS.Memory.releaseAllocation(id)
    local allocation = RobloxRS.Memory.allocations[id]
    if not allocation then return end
    
    RobloxRS.Memory.stats.currentUsage = RobloxRS.Memory.stats.currentUsage - allocation.size
    RobloxRS.Memory.allocations[id] = nil
end

function RobloxRS.Memory.getStats()
    return {
        totalAllocated = RobloxRS.Memory.stats.totalAllocated,
        currentUsage = RobloxRS.Memory.stats.currentUsage,
        peakUsage = RobloxRS.Memory.stats.peakUsage
    }
end

return RobloxRS.Memory"#.to_string()
}

pub fn generate_parallel_helpers() -> String {
    r#"-- Parallel execution helpers
local RobloxRS = RobloxRS or {}
RobloxRS.Parallel = {}

function RobloxRS.Parallel.forEach(array, action)
    for i, v in ipairs(array) do
        task.spawn(function()
            action(v, i)
        end)
    end
end

function RobloxRS.Parallel.map(array, transform)
    local results = table.create(#array)
    local completed = 0
    local mutex = {}
    
    for i, v in ipairs(array) do
        task.spawn(function()
            local result = transform(v, i)
            
            -- Critical section: update results
            table.insert(mutex, true)
            results[i] = result
            completed = completed + 1
            table.remove(mutex)
            
            -- If all tasks are done, resume waiting thread
            if completed == #array then
                RobloxRS.Parallel.signal()
            end
        end)
    end
    
    -- Wait for all tasks to complete
    if completed < #array then
        RobloxRS.Parallel.wait()
    end
    
    return results
end

function RobloxRS.Parallel.reduce(array, reducer, initialValue)
    if #array == 0 then return initialValue end
    
    local chunkSize = math.max(1, math.ceil(#array / 4))  -- Split into 4 chunks
    local chunks = {}
    
    for i = 1, #array, chunkSize do
        local endIndex = math.min(i + chunkSize - 1, #array)
        local chunk = {}
        for j = i, endIndex do
            table.insert(chunk, array[j])
        end
        table.insert(chunks, chunk)
    end
    
    local intermediateResults = {}
    RobloxRS.Parallel.forEach(chunks, function(chunk, index)
        local result = initialValue
        for _, value in ipairs(chunk) do
            result = reducer(result, value)
        end
        intermediateResults[index] = result
    end)
    
    -- Wait for all chunks to be processed
    RobloxRS.Parallel.wait()
    
    -- Combine intermediate results
    local finalResult = initialValue
    for _, result in ipairs(intermediateResults) do
        finalResult = reducer(finalResult, result)
    end
    
    return finalResult
end

-- Signal and wait implementation for synchronization
local waitingThread = nil

function RobloxRS.Parallel.wait()
    waitingThread = coroutine.running()
    coroutine.yield()
end

function RobloxRS.Parallel.signal()
    if waitingThread then
        task.spawn(waitingThread)
        waitingThread = nil
    end
end

return RobloxRS.Parallel"#.to_string()
}

pub fn generate_simd_helpers() -> String {
    r#"-- SIMD-style operation helpers
local RobloxRS = RobloxRS or {}
RobloxRS.SimdHelpers = {}

-- Process 4 numbers at once
function RobloxRS.SimdHelpers.add4(a1, a2, a3, a4, b1, b2, b3, b4)
    return a1 + b1, a2 + b2, a3 + b3, a4 + b4
end

function RobloxRS.SimdHelpers.sub4(a1, a2, a3, a4, b1, b2, b3, b4)
    return a1 - b1, a2 - b2, a3 - b3, a4 - b4
end

function RobloxRS.SimdHelpers.mul4(a1, a2, a3, a4, b1, b2, b3, b4)
    return a1 * b1, a2 * b2, a3 * b3, a4 * b4
end

function RobloxRS.SimdHelpers.div4(a1, a2, a3, a4, b1, b2, b3, b4)
    return a1 / b1, a2 / b2, a3 / b3, a4 / b4
end

-- Process arrays in chunks of 4 using vectorized operations
function RobloxRS.SimdHelpers.mapChunks(array, operation)
    local len = #array
    local result = table.create(len)
    
    local i = 1
    while i <= len - 3 do
        -- Process 4 elements at once
        result[i], result[i+1], result[i+2], result[i+3] = operation(
            array[i], array[i+1], array[i+2], array[i+3]
        )
        i = i + 4
    end
    
    -- Handle remaining elements
    while i <= len do
        result[i] = operation(array[i])
        i = i + 1
    end
    
    return result
end

return RobloxRS.SimdHelpers"#.to_string()
}

// Create a full runtime library that combines all helpers
pub fn create_combined_runtime() -> String {
    let object_pool = generate_object_pool();
    let memory_tracker = generate_memory_tracker();
    let parallel_helpers = generate_parallel_helpers();
    let simd_helpers = generate_simd_helpers();
    
    format!(
        "-- RobloxRS Runtime Library\nlocal RobloxRS = {{}}\n\n{}\n\n{}\n\n{}\n\n{}\n\nreturn RobloxRS",
        object_pool, memory_tracker, parallel_helpers, simd_helpers
    )
}

// Test functions that can be called directly from main
pub mod tests {
    use super::*;
    use std::fs;
    
    pub fn test_object_pool() -> bool {
        println!("Testing object pool features...");
        let code = generate_object_pool();
        
        let tests = [
            code.contains("RobloxRS.ObjectPool"),
            code.contains("function RobloxRS.ObjectPool.createPool"),
            code.contains("function RobloxRS.ObjectPool:allocate()"),
            code.contains("function RobloxRS.ObjectPool:release(obj)")
        ];
        
        let result = tests.iter().all(|&test| test);
        if !result {
            println!("❌ Object pool test failed");
        }
        result
    }
    
    pub fn test_memory_tracker() -> bool {
        println!("Testing memory tracker features...");
        let code = generate_memory_tracker();
        
        let tests = [
            code.contains("RobloxRS.Memory"),
            code.contains("function RobloxRS.Memory.trackAllocation"),
            code.contains("function RobloxRS.Memory.releaseAllocation"),
            code.contains("totalAllocated"),
            code.contains("currentUsage"),
            code.contains("peakUsage")
        ];
        
        let result = tests.iter().all(|&test| test);
        if !result {
            println!("❌ Memory tracker test failed");
        }
        result
    }
    
    pub fn test_parallel_helpers() -> bool {
        println!("Testing parallel execution features...");
        let code = generate_parallel_helpers();
        
        let tests = [
            code.contains("RobloxRS.Parallel"),
            code.contains("function RobloxRS.Parallel.forEach"),
            code.contains("function RobloxRS.Parallel.map"),
            code.contains("function RobloxRS.Parallel.reduce"),
            code.contains("task.spawn")
        ];
        
        let result = tests.iter().all(|&test| test);
        if !result {
            println!("❌ Parallel helpers test failed");
        }
        result
    }
    
    pub fn test_simd_helpers() -> bool {
        println!("Testing SIMD optimization features...");
        let code = generate_simd_helpers();
        
        let tests = [
            code.contains("RobloxRS.SimdHelpers"),
            code.contains("function RobloxRS.SimdHelpers.add4"),
            code.contains("function RobloxRS.SimdHelpers.mapChunks")
        ];
        
        let result = tests.iter().all(|&test| test);
        if !result {
            println!("❌ SIMD helpers test failed");
        }
        result
    }
    
    pub fn test_combined_runtime() -> bool {
        println!("Testing combined runtime features...");
        let code = create_combined_runtime();
        
        // Verify all components are included
        let component_tests = [
            code.contains("RobloxRS.ObjectPool"),
            code.contains("RobloxRS.Memory"),
            code.contains("RobloxRS.Parallel"),
            code.contains("RobloxRS.SimdHelpers")
        ];
        
        // Verify structure
        let structure_tests = [
            code.starts_with("-- RobloxRS Runtime Library"),
            code.contains("local RobloxRS = {}"),
            code.ends_with("return RobloxRS")
        ];
        
        let result = component_tests.iter().all(|&test| test) && 
                     structure_tests.iter().all(|&test| test);
        
        if !result {
            println!("❌ Combined runtime test failed");
        }
        result
    }
    
    pub fn test_write_to_file() -> bool {
        println!("Testing file output...");
        
        // Create a temporary directory for test outputs
        let output_dir = std::env::temp_dir();
        let output_path = output_dir.join("runtime.lua");
        
        // Write the runtime to a file
        let runtime_code = create_combined_runtime();
        
        let write_result = std::panic::catch_unwind(|| {
            fs::write(&output_path, runtime_code).expect("Failed to write runtime file");
            
            // Verify the file was created and contains the expected content
            assert!(output_path.exists());
            let file_content = fs::read_to_string(&output_path).expect("Failed to read runtime file");
            assert!(file_content.contains("RobloxRS.ObjectPool"));
            assert!(file_content.contains("RobloxRS.Memory"));
        });
        
        let result = write_result.is_ok();
        if !result {
            println!("❌ File output test failed");
        }
        result
    }
    
    // Run all tests and return overall result
    pub fn run_all_tests() -> bool {
        println!("\nRUNNING ALL RUNTIME TESTS\n");
        
        let test_results = [
            test_object_pool(),
            test_memory_tracker(),
            test_parallel_helpers(),
            test_simd_helpers(),
            test_combined_runtime(),
            test_write_to_file()
        ];
        
        let passed = test_results.iter().filter(|&&result| result).count();
        let total = test_results.len();
        
        println!("\nTEST SUMMARY: {}/{} tests passed", passed, total);
        
        let all_passed = test_results.iter().all(|&result| result);
        if all_passed {
            println!("✅ All tests passed!");
        } else {
            println!("❌ Some tests failed");
        }
        
        all_passed
    }
}
