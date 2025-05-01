--[[ 
    Comprehensive Roblox-RS Actor System Test
    This script tests all aspects of the actor system in a simulated Roblox environment.
]]

-- Mock Roblox API for local testing
local MockRoblox = {
    HttpService = {
        GenerateGUID = function(self, useAllowedChars)
            local guid = ""
            for i = 1, 32 do
                guid = guid .. string.char(math.random(97, 122))
            end
            return guid
        end
    },
    task = {
        wait = function(seconds)
            seconds = seconds or 0.03
            -- This is a mock - in a real environment this would yield
            print("task.wait(" .. tostring(seconds) .. ")")
        end,
        spawn = function(callback)
            -- In local testing, we run immediately
            callback()
        end,
        delay = function(seconds, callback)
            -- In local testing, we run immediately
            print("task.delay(" .. tostring(seconds) .. ")")
            callback()
        end
    }
}

-- Setup global environment to simulate Roblox
_G.game = {
    GetService = function(self, serviceName)
        return MockRoblox[serviceName]
    end
}
_G.task = MockRoblox.task

-- Load the actor system
local ActorSystem = require("actor_system")

-- Test harness
local ActorTests = {}

-- Run a single test with logging
function ActorTests.runTest(name, testFunc)
    print("\n===== Testing: " .. name .. " =====")
    local success, result = pcall(testFunc)
    if success then
        print("✅ PASSED: " .. name)
        return true
    else
        print("❌ FAILED: " .. name .. " - " .. tostring(result))
        return false
    end
end

-- Test simple actor creation and messaging
function ActorTests.testSimpleActors()
    -- Create a counter actor
    local counterActor = ActorSystem.spawn(function(state, message)
        if message.type == "increment" then
            state.count = (state.count or 0) + (message.value or 1)
            return state.count
        elseif message.type == "get" then
            return state.count or 0
        end
    end, {count = 0}, "Counter")
    
    -- Test sending messages and getting responses
    local result1 = counterActor:ask({type = "increment", value = 3})
    -- In mock mode this is immediate
    print("Increment result: " .. tostring(result1._result))
    assert(result1._result == 3, "Expected 3, got " .. tostring(result1._result))
    
    local result2 = counterActor:ask({type = "increment", value = 2})
    print("Increment result: " .. tostring(result2._result))
    assert(result2._result == 5, "Expected 5, got " .. tostring(result2._result))
    
    local getResult = counterActor:ask({type = "get"})
    print("Get result: " .. tostring(getResult._result))
    assert(getResult._result == 5, "Expected 5, got " .. tostring(getResult._result))
    
    -- Test termination
    counterActor:terminate()
    assert(not counterActor:isAlive(), "Actor should be terminated")
    
    return true
end

-- Test message priorities
function ActorTests.testMessagePriorities()
    -- Create an actor that records the order of received messages
    local priorityActor = ActorSystem.spawn(function(state, message)
        table.insert(state.received, message.priority)
        return state.received
    end, {received = {}}, "PriorityTest")
    
    -- Send messages with different priorities
    priorityActor:send({priority = "low"}, ActorSystem.Priority.LOW)
    priorityActor:send({priority = "high"}, ActorSystem.Priority.HIGH)
    priorityActor:send({priority = "normal"}, ActorSystem.Priority.NORMAL)
    
    -- Get the order of received messages
    local result = priorityActor:ask({type = "get"})
    print("Received order: " .. table.concat(result._result, ", "))
    
    -- Verify high priority was processed first
    assert(result._result[1] == "high", "Expected high priority first")
    assert(result._result[2] == "normal", "Expected normal priority second")
    assert(result._result[3] == "low", "Expected low priority last")
    
    priorityActor:terminate()
    return true
end

-- Test supervised actors
function ActorTests.testSupervisedActors()
    local crashCount = 0
    
    -- Create a supervised actor that crashes on demand
    local supervised = ActorSystem.supervise(
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
    local workResult = supervised:ask({type = "work", value = 42})
    print("Work result: " .. tostring(workResult._result))
    assert(workResult._result == "Processed: 42", "Work not processed correctly")
    
    -- Trigger crashes
    supervised:ask({type = "crash"})
    supervised:ask({type = "crash"})
    supervised:ask({type = "crash"})
    
    -- This should survive
    local finalResult = supervised:ask({type = "crash"})
    print("Final result: " .. tostring(finalResult._result))
    assert(string.find(tostring(finalResult._result), "Survived"), "Supervision didn't work correctly")
    
    supervised:terminate()
    return true
end

-- Test actor pools
function ActorTests.testActorPools()
    -- Create an actor pool with 4 workers
    local pool = ActorSystem.createPool(4, function(state, message)
        -- Simple work function: double the input value
        return message.value * 2
    end)
    
    -- Submit multiple items
    local promises = {}
    for i = 1, 5 do
        table.insert(promises, pool:submit({value = i}))
        print("Submitted work item " .. i)
    end
    
    -- Check results
    for i, promise in ipairs(promises) do
        print("Result for work item " .. i .. ": " .. tostring(promise._result))
        assert(promise._result == i * 2, "Expected " .. (i * 2) .. ", got " .. tostring(promise._result))
    end
    
    pool:terminate()
    return true
end

-- Test shared state
function ActorTests.testSharedState()
    -- Create shared state
    local shared = ActorSystem.createSharedState({
        counter = 0,
        status = "idle"
    })
    
    -- Use shared state from multiple actors
    local actors = {}
    for i = 1, 3 do
        actors[i] = ActorSystem.spawn(function(state, message)
            if message.type == "increment" then
                local current = shared:get("counter")
                shared:set("counter", current + 1)
                return shared:get("counter")
            end
        end, {id = i})
    end
    
    -- Have each actor increment the counter
    for i = 1, 3 do
        local result = actors[i]:ask({type = "increment"})
        print("Actor " .. i .. " incremented counter to: " .. tostring(result._result))
    end
    
    -- Get final counter value
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
    
    return true
end

-- Test promise-like behavior with andThen
function ActorTests.testPromiseBehavior()
    -- Create a calculator actor
    local calculator = ActorSystem.spawn(function(state, message)
        if message.type == "add" then
            return message.a + message.b
        elseif message.type == "multiply" then
            return message.a * message.b
        end
    end)
    
    -- Test the andThen pattern
    local addPromise = calculator:ask({type = "add", a = 3, b = 5})
    addPromise:andThen(function(success, result)
        print("Add result: " .. tostring(result))
        assert(result == 8, "Expected 8, got " .. tostring(result))
        
        -- Chain another operation
        local multiplyPromise = calculator:ask({type = "multiply", a = result, b = 2})
        multiplyPromise:andThen(function(success, multiplyResult)
            print("Multiply result: " .. tostring(multiplyResult))
            assert(multiplyResult == 16, "Expected 16, got " .. tostring(multiplyResult))
        end)
    end)
    
    calculator:terminate()
    return true
end

-- Run all tests
function ActorTests.runAllTests()
    print("\n===== Running Actor System Tests =====\n")
    local results = {
        ActorTests.runTest("Simple Actors", ActorTests.testSimpleActors),
        ActorTests.runTest("Message Priorities", ActorTests.testMessagePriorities),
        ActorTests.runTest("Supervised Actors", ActorTests.testSupervisedActors),
        ActorTests.runTest("Actor Pools", ActorTests.testActorPools),
        ActorTests.runTest("Shared State", ActorTests.testSharedState),
        ActorTests.runTest("Promise Behavior", ActorTests.testPromiseBehavior)
    }
    
    -- Count passed tests
    local passCount = 0
    for _, passed in ipairs(results) do
        if passed then passCount = passCount + 1 end
    end
    
    print("\n===== Test Results =====")
    print(passCount .. "/" .. #results .. " tests passed")
    
    if passCount == #results then
        print("✅ ALL TESTS PASSED")
    else
        print("❌ SOME TESTS FAILED")
    end
    
    return passCount == #results
end

-- Run all tests
ActorTests.runAllTests()
