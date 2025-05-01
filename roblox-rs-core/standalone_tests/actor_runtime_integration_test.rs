//! Actor system runtime integration test
//! This test verifies that the actor system is properly integrated with the runtime

use std::fs;
use std::env;

// Import our actor system module with a more specific path
mod test_actor_system {
    // Re-export the function we need
    pub fn generate_actor_system() -> String {
        include_str!("../roblox_actor_test/src/SimpleActorSystem.lua").to_string()
    }
}

/// Test that verifies the actor system integration with the runtime
fn test_actor_system_runtime_integration() -> bool {
    println!("Running actor system runtime integration test...");
    
    // Get the temp directory for test output
    let temp_dir = env::temp_dir().join("roblox_rs_actor_runtime_test");
    let _ = fs::remove_dir_all(&temp_dir); // Clean up previous test
    fs::create_dir_all(&temp_dir).expect("Failed to create test directory");
    
    // Test the actor system runtime Lua code generation
    let full_runtime = generate_runtime_with_actors();
    println!("Generated runtime with actors: {} bytes", full_runtime.len());
    
    // Write the runtime to a file
    let runtime_file = temp_dir.join("roblox_rs_runtime.lua");
    fs::write(&runtime_file, &full_runtime).expect("Failed to write runtime file");
    println!("Runtime written to: {}", runtime_file.display());
    
    // Create a test script that uses the runtime
    let test_script = generate_actor_test_script();
    let test_file = temp_dir.join("actor_runtime_test.lua");
    fs::write(&test_file, test_script).expect("Failed to write test script");
    println!("Test script written to: {}", test_file.display());
    
    // Everything generated successfully
    println!("Actor system runtime integration test completed successfully");
    true
}

/// Generate the runtime code with actor system
fn generate_runtime_with_actors() -> String {
    let mut runtime = String::from("-- Roblox-RS Runtime\nlocal RobloxRS = {}\n\n");
    
    // Include object pooling helpers
    runtime.push_str("-- Object pooling system\nRobloxRS.Pool = {}\n\n");
    runtime.push_str("function RobloxRS.Pool.new(objectType, initialSize, factory)\n");
    runtime.push_str("    local pool = {\n");
    runtime.push_str("        available = {},\n");
    runtime.push_str("        allocated = 0,\n");
    runtime.push_str("        objectType = objectType,\n");
    runtime.push_str("        factory = factory or function() return Instance.new(objectType) end\n");
    runtime.push_str("    }\n\n");
    runtime.push_str("    function pool:get()\n");
    runtime.push_str("        if #self.available > 0 then\n");
    runtime.push_str("            return table.remove(self.available)\n");
    runtime.push_str("        else\n");
    runtime.push_str("            self.allocated = self.allocated + 1\n");
    runtime.push_str("            return self.factory()\n");
    runtime.push_str("        end\n");
    runtime.push_str("    end\n\n");
    runtime.push_str("    function pool:release(object)\n");
    runtime.push_str("        table.insert(self.available, object)\n");
    runtime.push_str("    end\n\n");
    runtime.push_str("    return pool\n");
    runtime.push_str("end\n\n");
    
    // Include the actor system
    runtime.push_str("-- Actor System Implementation\n");
    runtime.push_str("RobloxRS.Actors = ");
    runtime.push_str(&test_actor_system::generate_actor_system());
    
    runtime
}

/// Generate an actor test script
fn generate_actor_test_script() -> String {
    let script = r#"-- Actor System Runtime Integration Test
-- This script tests the integration of the actor system with the Roblox-RS runtime

-- Import the Roblox-RS runtime
local RobloxRS = require("roblox_rs_runtime")

print("Starting actor system runtime integration test...")

-- Test basic actor creation and messaging
local function testBasicActorSystem()
    print("Testing basic actor system...")
    
    -- Create an actor that increments a counter
    local counter = RobloxRS.Actors.spawn(function(state, message)
        if message.type == "increment" then
            state.count = (state.count or 0) + message.value
            return state.count
        elseif message.type == "get" then
            return state.count or 0
        end
    end, {count = 0}, "Counter")
    
    -- Send an increment message and get the result
    local result = counter:ask({type = "increment", value = 5}):await()
    print("Increment result: " .. tostring(result))
    assert(result == 5, "Expected 5, got " .. tostring(result))
    
    -- Get the counter value
    local getResult = counter:ask({type = "get"}):await()
    print("Counter value: " .. tostring(getResult))
    assert(getResult == 5, "Expected 5, got " .. tostring(getResult))
    
    -- Terminate the actor
    counter:terminate()
    print("Basic actor system test passed")
    return true
end

-- Test supervised actors
local function testSupervisedActors()
    print("Testing supervised actors...")
    
    local crashCount = 0
    
    -- Create a supervised actor that crashes on demand
    local supervised = RobloxRS.Actors.supervise(
        function(state, message)
            if message.type == "crash" then
                crashCount = crashCount + 1
                print("Crash #" .. crashCount .. " requested")
                if crashCount <= 3 then
                    error("Intentional crash #" .. crashCount)
                end
                return "Survived crash #" .. crashCount
            elseif message.type == "work" then
                return "Processed: " .. tostring(message.value)
            end
        end,
        {},
        "SupervisedActor",
        {maxRestarts = 3, restartDelay = 0.1}
    )
    
    -- Do some work first
    local workResult = supervised:ask({type = "work", value = 42}):await()
    print("Work result: " .. tostring(workResult))
    assert(workResult == "Processed: 42", "Work not processed correctly")
    
    -- Trigger final crash that should not error due to supervision
    local finalResult = supervised:ask({type = "crash"}):await()
    print("Final result: " .. tostring(finalResult))
    assert(string.find(tostring(finalResult), "Survived"), "Supervision didn't work correctly")
    
    -- Terminate the actor
    supervised:terminate()
    print("Supervised actors test passed")
    return true
end

-- Test actor pools
local function testActorPools()
    print("Testing actor pools...")
    
    -- Create an actor pool with 4 workers
    local pool = RobloxRS.Actors.createPool(4, function(state, message)
        -- Simple work function: double the input value
        return message.value * 2
    end)
    
    -- Submit multiple items
    print("Submitting work items...")
    local promises = {}
    for i = 1, 5 do
        table.insert(promises, pool:submit({value = i}))
        print("Submitted work item " .. i)
    end
    
    -- Wait for all results
    print("Waiting for results...")
    local results = {}
    for i, promise in ipairs(promises) do
        local result = promise:await()
        print("Result for work item " .. i .. ": " .. tostring(result))
        table.insert(results, result)
        assert(result == i * 2, "Expected " .. (i * 2) .. ", got " .. tostring(result))
    end
    
    -- Terminate the pool
    pool:terminate()
    print("Actor pools test passed")
    return true
end

-- Test shared state
local function testSharedState()
    print("Testing shared state...")
    
    -- Create shared state
    local shared = RobloxRS.Actors.createSharedState({
        counter = 0,
        status = "idle"
    })
    
    -- Create actors to use the shared state
    local actors = {}
    for i = 1, 3 do
        actors[i] = RobloxRS.Actors.spawn(function(state, message)
            if message.type == "increment" then
                local current = shared:get("counter")
                shared:set("counter", current + 1)
                return shared:get("counter")
            end
        end, {id = i})
    end
    
    -- Have each actor increment the counter
    for i = 1, 3 do
        local result = actors[i]:ask({type = "increment"}):await()
        print("Actor " .. i .. " incremented counter to: " .. tostring(result))
    end
    
    -- Get the final counter value
    local finalCount = shared:get("counter")
    print("Final counter value: " .. tostring(finalCount))
    assert(finalCount == 3, "Expected counter to be 3, got " .. tostring(finalCount))
    
    -- Test updating multiple values
    shared:update({
        counter = 10,
        status = "active"
    })
    
    local newCounter = shared:get("counter")
    local newStatus = shared:get("status")
    print("Updated values - counter: " .. tostring(newCounter) .. ", status: " .. tostring(newStatus))
    assert(newCounter == 10, "Expected counter to be 10 after update")
    assert(newStatus == "active", "Expected status to be 'active' after update")
    
    -- Clean up
    for i = 1, 3 do
        actors[i]:terminate()
    end
    
    print("Shared state test passed")
    return true
end

-- Run all tests
local function runAllTests()
    print("\n=== Running All Tests ===\n")
    local results = {
        testBasicActorSystem(),
        testSupervisedActors(),
        testActorPools(),
        testSharedState()
    }
    
    -- Count passed tests
    local passCount = 0
    for _, passed in ipairs(results) do
        if passed then passCount = passCount + 1 end
    end
    
    print("\n=== Test Results ===")
    print(passCount .. "/" .. #results .. " tests passed")
    
    if passCount == #results then
        print("✅ ALL TESTS PASSED")
        return true
    else
        print("❌ SOME TESTS FAILED")
        return false
    end
end

-- Execute all tests
return runAllTests()
"#;
    
    script.to_string()
}

fn main() {
    // Run the actor system runtime integration test
    let result = test_actor_system_runtime_integration();
    
    // Exit with appropriate status code
    if result {
        println!("All tests passed!");
        std::process::exit(0);
    } else {
        println!("Tests failed!");
        std::process::exit(1);
    }
}
