-- RobloxRS Actor System Tests
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Actors = require(ReplicatedStorage.RobloxRS.SimpleActorSystem)

print("Loaded SimpleActorSystem")

print("===== RobloxRS Actor System Tests =====")

local function runTest(name, testFunc)
    print("\n=== Test: " .. name .. " ===")
    local success, result = pcall(testFunc)
    if success then
        print("✅ PASSED: " .. name)
        return true
    else
        print("❌ FAILED: " .. name .. " - " .. tostring(result))
        return false
    end
end

local results = {}

-- Test 1: Basic actor spawning and messaging
table.insert(results, runTest("Basic actor", function()
    local counter = Actors.spawn(function(state, message)
        if message.type == "increment" then
            state.count = (state.count or 0) + (message.value or 1)
            return state.count
        elseif message.type == "get" then
            return state.count or 0
        end
    end, {count = 0}, "Counter")
    
    print("  Sending increment message...")
    local result = counter:ask({type = "increment", value = 5}):await()
    print("  Result: " .. tostring(result))
    assert(result == 5, "Expected 5, got " .. tostring(result))
    
    print("  Getting counter value...")
    local getResult = counter:ask({type = "get"}):await()
    print("  Result: " .. tostring(getResult))
    assert(getResult == 5, "Expected stored value 5, got " .. tostring(getResult))
    
    print("  Terminating actor...")
    counter:terminate()
    assert(not counter:isAlive(), "Actor should be terminated")
    
    return true
end))

-- Test 2: Message priorities
table.insert(results, runTest("Message priorities", function()
    local priorityTest = Actors.spawn(function(state, message)
        table.insert(state.received, message.priority or "unknown")
        return state.received
    end, {received = {}}, "PriorityTest")
    
    print("  Sending low priority message first...")
    priorityTest:send({priority = "low"}, Actors.Priority.LOW)
    
    print("  Sending high priority message second...")
    priorityTest:send({priority = "high"}, Actors.Priority.HIGH)
    
    print("  Sending normal priority message third...")
    priorityTest:send({priority = "normal"}, Actors.Priority.NORMAL)
    
    -- Wait for processing
    print("  Waiting for message processing...")
    task.wait(0.5)
    
    -- Get the order of received messages
    print("  Getting received message order...")
    local result = priorityTest:ask({type = "get"}):await()
    print("  Received order: " .. table.concat(result, ", "))
    
    -- Verify high priority was processed first
    assert(result[1] == "high", "Expected high priority first, got: " .. tostring(result[1]))
    
    priorityTest:terminate()
    return true
end))

-- Test 3: Supervised actors
table.insert(results, runTest("Supervised actors", function()
    local crashCount = 0
    
    print("  Creating supervised actor...")
    local supervised = Actors.supervise(
        function(state, message)
            if message.type == "crash" then
                crashCount = crashCount + 1
                print("  Crash #" .. crashCount .. " requested")
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
    print("  Sending work message...")
    local workResult = supervised:ask({type = "work", value = 42}):await()
    print("  Work result: " .. tostring(workResult))
    assert(string.find(workResult, "Processed: 42"), "Work not processed correctly")
    
    -- Trigger crashes
    print("  Triggering crash 1...")
    local _crashResult1 = supervised:ask({type = "crash"})
    task.wait(0.2) -- Wait for restart
    
    print("  Triggering crash 2...")
    local _crashResult2 = supervised:ask({type = "crash"})
    task.wait(0.2) -- Wait for restart
    
    print("  Triggering crash 3...")
    local _crashResult3 = supervised:ask({type = "crash"})
    task.wait(0.2) -- Wait for restart
    
    -- Should survive this one
    print("  Triggering final crash...")
    local finalResult = supervised:ask({type = "crash"}):await()
    print("  Final result: " .. tostring(finalResult))
    assert(string.find(tostring(finalResult), "Survived"), "Supervision didn't work correctly")
    
    supervised:terminate()
    return true
end))

-- Test 4: Actor pools
table.insert(results, runTest("Actor pools", function()
    print("  Creating actor pool with 4 workers...")
    local pool = Actors.createPool(4, function(state, message)
        -- Simple work function: double the input value
        task.wait(0.1) -- Simulate work being done
        return message.value * 2
    end)
    
    -- Submit multiple items
    print("  Submitting 10 work items...")
    local promises = {}
    for i = 1, 10 do
        table.insert(promises, pool:submit({value = i}))
        print("  Submitted work: " .. i)
    end
    
    -- Wait for all results
    print("  Waiting for results...")
    task.wait(0.5)
    
    -- Verify at least the first result
    print("  Checking results...")
    local firstResult = promises[1]:await()
    print("  First result: " .. tostring(firstResult))
    assert(firstResult == 2, "Expected pool to return 2 for first task, got " .. tostring(firstResult))
    
    -- Wait for more results and verify them
    local allResults = {}
    for i, promise in ipairs(promises) do
        if i <= 5 then -- Just check the first 5 to save time
            local result = promise:await()
            print("  Result for work item " .. i .. ": " .. tostring(result))
            table.insert(allResults, result)
            assert(result == i * 2, "Expected result " .. (i * 2) .. ", got " .. tostring(result))
        end
    end
    
    print("  Terminating pool...")
    pool:terminate()
    return true
end))

-- Test 5: Shared state
table.insert(results, runTest("Shared state", function()
    print("  Creating shared state...")
    local shared = Actors.createSharedState({
        counter = 0,
        status = "idle"
    })
    
    -- Use shared state from multiple actors
    print("  Creating actors to use shared state...")
    local actors = {}
    for i = 1, 3 do
        actors[i] = Actors.spawn(function(state, message)
            if message.type == "increment" then
                print("  Actor " .. state.id .. " incrementing counter")
                local current = shared:get("counter")
                shared:set("counter", current + 1)
                return shared:get("counter")
            elseif message.type == "state_changed" then
                -- Handle state change notifications
                return "State changed: " .. message.key .. " = " .. tostring(message.value)
            end
        end, {id = i})
        
        -- Subscribe to counter changes
        shared:subscribe("counter", actors[i].id)
    end
    
    -- Have each actor increment the counter
    print("  Having each actor increment the shared counter...")
    for i = 1, 3 do
        local result = actors[i]:ask({type = "increment"}):await()
        print("  Actor " .. i .. " incremented counter to: " .. tostring(result))
    end
    
    -- Get final counter value
    print("  Getting final counter value...")
    local finalCount = shared:get("counter")
    print("  Final counter value: " .. tostring(finalCount))
    assert(finalCount == 3, "Expected shared counter to be 3, got " .. tostring(finalCount))
    
    -- Test updating multiple values
    print("  Testing multi-value update...")
    shared:update({
        counter = 10,
        status = "active"
    })
    
    local newCounter = shared:get("counter")
    local newStatus = shared:get("status")
    print("  Updated values - counter: " .. tostring(newCounter) .. ", status: " .. tostring(newStatus))
    assert(newCounter == 10, "Expected counter to be 10 after update")
    assert(newStatus == "active", "Expected status to be 'active' after update")
    
    -- Test deleting a value
    print("  Testing delete...")
    shared:delete("status")
    local deletedStatus = shared:get("status")
    print("  Status after delete: " .. tostring(deletedStatus))
    assert(deletedStatus == nil, "Expected status to be nil after delete")
    
    -- Clean up
    print("  Cleaning up actors...")
    for i = 1, 3 do
        actors[i]:terminate()
    end
    
    return true
end))

-- Test 6: Ask with timeout
table.insert(results, runTest("Ask with timeout", function()
    print("  Creating actor with delayed response...")
    local timeoutActor = Actors.spawn(function(state, message)
        if message.type == "delay" then
            -- Simulate long-running work
            print("  Actor received delay request, waiting...")
            task.wait(0.3)
            return "Delayed response"
        else
            return "Quick response"
        end
    end)
    
    -- This should time out
    print("  Testing timeout (should fail after 0.1s)...")
    local timedOutPromise = timeoutActor:ask({type = "delay"}, 0.1)
    local timedOut = false
    
    timedOutPromise:andThen(function(success, result)
        print("  Timeout result - success: " .. tostring(success) .. ", result: " .. tostring(result))
        if not success and string.find(tostring(result), "Timeout") then
            timedOut = true
        end
    end)
    
    -- Wait for timeout to occur
    task.wait(0.2)
    
    print("  Timeout occurred: " .. tostring(timedOut))
    assert(timedOut, "Expected timeout did not occur")
    
    -- This should complete normally
    print("  Testing normal request (should succeed)...")
    local normalPromise = timeoutActor:ask({type = "normal"})
    local normalResult = normalPromise:await()
    
    print("  Normal result: " .. tostring(normalResult))
    assert(normalResult == "Quick response", "Expected 'Quick response', got " .. tostring(normalResult))
    
    print("  Terminating actor...")
    timeoutActor:terminate()
    return true
end))

-- Calculate and display test results
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
