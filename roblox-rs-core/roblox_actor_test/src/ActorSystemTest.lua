--[[
    RobloxRS Actor System Test Suite
    This script tests all aspects of the actor system in a real Roblox environment.
]]

-- Get the actor system from ReplicatedStorage
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Actors = require(ReplicatedStorage.RobloxRS.SimpleActorSystem)

-- Test results collector
local Results = {
    tests = 0,
    passed = 0,
    failed = 0,
    failures = {}
}

-- Test harness function
local function runTest(name, testFunc)
    print("Running test: " .. name)
    Results.tests = Results.tests + 1
    
    local success, result = pcall(function()
        return testFunc()
    end)
    
    if success and result then
        print("✅ Test PASSED: " .. name)
        Results.passed = Results.passed + 1
        return true
    else
        local errorMsg = success and "Test returned false" or result
        print("❌ Test FAILED: " .. name .. " - " .. tostring(errorMsg))
        Results.failed = Results.failed + 1
        table.insert(Results.failures, {name = name, error = errorMsg})
        return false
    end
end

-- Test suite
local Tests = {}

-- Test 1: Basic actor creation and messaging
function Tests.BasicActor()
    print("Creating counter actor...")
    local counter = Actors.spawn(function(state, message)
        if message.type == "increment" then
            state.count = (state.count or 0) + (message.value or 1)
            return state.count
        elseif message.type == "get" then
            return state.count or 0
        end
    end, {count = 0}, "Counter")
    
    -- Send increment message
    print("Sending increment message...")
    local result = counter:ask({type = "increment", value = 5}):await()
    print("Increment result: " .. tostring(result))
    assert(result == 5, "Expected 5, got " .. tostring(result))
    
    -- Get counter value
    print("Getting counter value...")
    local getResult = counter:ask({type = "get"}):await()
    print("Counter value: " .. tostring(getResult))
    assert(getResult == 5, "Expected 5, got " .. tostring(getResult))
    
    -- Clean up
    counter:terminate()
    
    return true
end

-- Test 2: Message priorities
function Tests.MessagePriorities()
    print("Creating priority test actor...")
    local priorityTest = Actors.spawn(function(state, message)
        table.insert(state.received, message.priority or "unknown")
        return state.received
    end, {received = {}}, "PriorityTest")
    
    -- Send messages with different priorities
    print("Sending low priority message first...")
    priorityTest:send({priority = "low"}, Actors.Priority.LOW)
    
    print("Sending high priority message second...")
    priorityTest:send({priority = "high"}, Actors.Priority.HIGH)
    
    print("Sending normal priority message third...")
    priorityTest:send({priority = "normal"}, Actors.Priority.NORMAL)
    
    -- Wait for processing
    task.wait(0.2)
    
    -- Get the order of received messages
    print("Getting received message order...")
    local result = priorityTest:ask({type = "get"}):await()
    print("Received order: " .. table.concat(result, ", "))
    
    -- Verify high priority was processed first
    assert(result[1] == "high", "Expected high priority first, got: " .. tostring(result[1]))
    
    -- Clean up
    priorityTest:terminate()
    
    return true
end

-- Test 3: Supervised actors
function Tests.SupervisedActors()
    print("Creating supervised actor...")
    
    -- Track crash count 
    local crashCount = 0
    
    local supervised = Actors.supervise(
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
    print("Sending work message...")
    local workResult = supervised:ask({type = "work", value = 42}):await()
    print("Work result: " .. tostring(workResult))
    assert(workResult == "Processed: 42", "Work not processed correctly")
    
    -- Trigger crashes and wait for restarts
    print("Triggering crash 1...")
    supervised:ask({type = "crash"})
    task.wait(0.2) -- Wait for restart
    
    print("Triggering crash 2...")
    supervised:ask({type = "crash"})
    task.wait(0.2) -- Wait for restart
    
    print("Triggering crash 3...")
    supervised:ask({type = "crash"})
    task.wait(0.2) -- Wait for restart
    
    -- Should survive this one
    print("Triggering final crash...")
    local finalResult = supervised:ask({type = "crash"}):await()
    print("Final result: " .. tostring(finalResult))
    assert(string.find(tostring(finalResult), "Survived"), "Supervision didn't work correctly")
    
    -- Clean up
    supervised:terminate()
    
    return true
end

-- Test 4: Actor pools
function Tests.ActorPools()
    print("Creating actor pool with 4 workers...")
    local pool = Actors.createPool(4, function(state, message)
        -- Simple work function: double the input value
        task.wait(0.1) -- Simulate work being done
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
    
    -- Clean up
    pool:terminate()
    
    return true
end

-- Test 5: Shared state
function Tests.SharedState()
    print("Creating shared state...")
    local shared = Actors.createSharedState({
        counter = 0,
        status = "idle"
    })
    
    -- Create actors to use the shared state
    print("Creating actors to use shared state...")
    local actors = {}
    for i = 1, 3 do
        actors[i] = Actors.spawn(function(state, message)
            if message.type == "increment" then
                print("Actor " .. state.id .. " incrementing counter")
                local current = shared:get("counter")
                shared:set("counter", current + 1)
                return shared:get("counter")
            end
        end, {id = i})
    end
    
    -- Have each actor increment the counter
    print("Having each actor increment the shared counter...")
    for i = 1, 3 do
        local result = actors[i]:ask({type = "increment"}):await()
        print("Actor " .. i .. " incremented counter to: " .. tostring(result))
    end
    
    -- Get final counter value
    print("Getting final counter value...")
    local finalCount = shared:get("counter")
    print("Final counter value: " .. tostring(finalCount))
    assert(finalCount == 3, "Expected shared counter to be 3, got " .. tostring(finalCount))
    
    -- Test updating multiple values
    print("Testing multi-value update...")
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
    print("Cleaning up actors...")
    for i = 1, 3 do
        actors[i]:terminate()
    end
    
    return true
end

-- Test 6: Promise behavior
function Tests.PromiseBehavior()
    print("Creating actor for promise testing...")
    local delayedActor = Actors.spawn(function(state, message)
        if message.type == "delay" then
            -- Simulate long-running work
            print("Actor received delay request, waiting...")
            task.wait(0.3)
            return "Delayed response"
        else
            return "Quick response"
        end
    end)
    
    -- Test timeout
    print("Testing timeout (should fail after 0.1s)...")
    local timedOutPromise = delayedActor:ask({type = "delay"}, 0.1)
    local timedOut = false
    
    timedOutPromise:andThen(function(success, result)
        print("Timeout result - success: " .. tostring(success) .. ", result: " .. tostring(result))
        if not success and string.find(tostring(result), "Timeout") then
            timedOut = true
        end
    end)
    
    -- Wait for timeout to occur
    task.wait(0.5)
    
    print("Timeout occurred: " .. tostring(timedOut))
    assert(timedOut, "Expected timeout did not occur")
    
    -- Normal request should succeed
    print("Testing normal request (should succeed)...")
    local normalPromise = delayedActor:ask({type = "normal"})
    local normalResult = normalPromise:await()
    
    print("Normal result: " .. tostring(normalResult))
    assert(normalResult == "Quick response", "Expected 'Quick response', got " .. tostring(normalResult))
    
    -- Clean up
    delayedActor:terminate()
    
    return true
end

-- Run all tests
function RunAllTests()
    print("\n===== Running Actor System Tests =====\n")
    
    -- Run individual tests
    runTest("Basic Actor", Tests.BasicActor)
    runTest("Message Priorities", Tests.MessagePriorities)
    runTest("Supervised Actors", Tests.SupervisedActors)
    runTest("Actor Pools", Tests.ActorPools)
    runTest("Shared State", Tests.SharedState)
    runTest("Promise Behavior", Tests.PromiseBehavior)
    
    -- Print results
    print("\n===== Test Results =====")
    print(Results.passed .. "/" .. Results.tests .. " tests passed")
    
    if Results.failed > 0 then
        print("\nFailed Tests:")
        for _, failure in ipairs(Results.failures) do
            print("- " .. failure.name .. ": " .. tostring(failure.error))
        end
        print("\n❌ SOME TESTS FAILED")
    else
        print("\n✅ ALL TESTS PASSED")
    end
    
    return Results.failed == 0
end

-- Run tests
return RunAllTests()
