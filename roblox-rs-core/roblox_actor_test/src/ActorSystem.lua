-- Actor System for Roblox-RS
-- Implements a Rust-like actor model that works within Roblox's limitations
local HttpService = game:GetService("HttpService")

local Actors = {
    actors = {},             -- Registered actors
    mailboxes = {},          -- Message queues for each actor
    supervisors = {},        -- Tracks child-parent relationships
    terminated = {},         -- List of terminated actors
    callbacks = {},          -- Callback registry
    initialized = false      -- Whether the system has been initialized
}

-- Actor states
local ActorStatus = {
    IDLE = "idle",           -- Actor is waiting for messages
    PROCESSING = "processing", -- Actor is processing a message
    TERMINATED = "terminated"  -- Actor has been terminated
}

-- Message priority levels
local MessagePriority = {
    HIGH = 1,
    NORMAL = 2,
    LOW = 3
}

-- Actor handle that's returned to callers
local ActorHandle = {}
ActorHandle.__index = ActorHandle

function ActorHandle.new(id)
    local self = setmetatable({}, ActorHandle)
    self.id = id
    return self
end

-- Send a message to this actor
function ActorHandle:send(message, priority)
    return Actors.send(self.id, message, priority)
end

-- Send a message and wait for a response (uses a promise-like pattern)
function ActorHandle:ask(message, timeout)
    return Actors.ask(self.id, message, timeout)
end

-- Check if actor is alive
function ActorHandle:isAlive()
    return Actors.isActorAlive(self.id)
end

-- Terminate this actor
function ActorHandle:terminate()
    return Actors.terminate(self.id)
end

-- Monitor this actor for termination
function ActorHandle:monitor(callback)
    Actors.monitorActor(self.id, callback)
    return self
end

-- Create a new actor with its own state and message queue
-- Initialize the actor system
function Actors.init()
    if Actors.initialized then
        return
    end
    
    -- Set initialized flag
    Actors.initialized = true
    
    -- Create a container for our actors if needed
    if not workspace:FindFirstChild("RobloxRSActors") then
        local container = Instance.new("Folder")
        container.Name = "RobloxRSActors"
        container.Parent = workspace
    end
    
    print("RobloxRS Actor System initialized successfully")
end

function Actors.spawn(handler, initialState, name)
    -- Make sure the system is initialized
    if not Actors.initialized then
        Actors.init()
    end
    
    -- Generate unique ID for this actor
    local id = HttpService:GenerateGUID(false)
    local displayName = name or "Actor_" .. id:sub(1, 8)
    
    -- Create actor instance
    local actor = {
        id = id,
        name = displayName,
        mailbox = {},             -- Message queue
        state = initialState or {}, -- Actor's internal state
        status = ActorStatus.IDLE,
        lastActivity = os.clock(),
        children = {},            -- Child actors spawned by this actor
        priorityQueue = {         -- Separate queues for different priorities
            [MessagePriority.HIGH] = {},
            [MessagePriority.NORMAL] = {},
            [MessagePriority.LOW] = {}
        },
        monitors = {},            -- Callbacks that monitor this actor
        handler = handler         -- Store the message handler function
    }
    
    -- Register actor in the system
    Actors.actors[id] = actor
    Actors.mailboxes[id] = actor.mailbox
    
    -- Create the actual Roblox Actor instance
    local robloxActor = Instance.new("Actor")
    robloxActor.Name = displayName
    actor.instance = robloxActor
    
    -- Create a script to handle this actor's messages
    local script = Instance.new("Script")
    script.Name = "ActorScript"
    script.Parent = robloxActor
    
    -- Write the handler logic to process messages
    script.Source = [[
        local actor = script.Parent
        local actorId = "]] .. id .. [["
        
        -- Process messages in a loop
        local function processNext()
            -- Call back to main thread to get next message
            local success, message = actor:SendMessage("GetNextMessage", actorId)
            
            if success and message then
                -- Process the message
                local result = actor:SendMessage("ProcessMessage", actorId, message)
                
                -- Schedule next message processing
                task.defer(processNext)
            else
                -- No messages to process, check again after a short delay
                task.delay(0.05, processNext)
            end
        end
        
        -- Start processing
        task.spawn(processNext)
    ]]
    
    -- Set up message handlers on the actor
    robloxActor:BindToMessage("GetNextMessage", function(requestId)
        if requestId ~= id then return false, "Invalid actor ID" end
        
        local actor = Actors.actors[id]
        if not actor then return false, "Actor not found" end
        
        -- Check if there are messages to process, starting with high priority
        for priority = MessagePriority.HIGH, MessagePriority.LOW do
            if #actor.priorityQueue[priority] > 0 then
                local nextMessage = table.remove(actor.priorityQueue[priority], 1)
                actor.status = ActorStatus.PROCESSING
                actor.lastActivity = os.clock()
                return true, nextMessage
            end
        end
        
        -- No messages available
        actor.status = ActorStatus.IDLE
        return false, nil
    end)
    
    -- Handler to process messages
    robloxActor:BindToMessage("ProcessMessage", function(requestId, message)
        if requestId ~= id then return "Invalid actor ID" end
        
        local actor = Actors.actors[id]
        if not actor then return "Actor not found" end
        
        -- Process message using handler
        local success, result = pcall(function()
            return actor.handler(actor.state, message)
        end)
        
        -- Update status
        actor.status = ActorStatus.IDLE
        
        -- If message has a response channel, send the result back
        if message and message._responseChannel then
            Actors.callbacks[message._responseChannel](success, result)
            return "Response sent"
        end
        
        if success then
            return result or "Message processed"
        else
            warn("Actor " .. actor.name .. " error: " .. tostring(result))
            return "Error processing message"
        end
    end)
    
    -- Add actor to the container
    robloxActor.Parent = workspace.RobloxRSActors
    
    -- Return handle to the actor
    return ActorHandle.new(id)
end

-- Send a message to an actor
function Actors.send(actorId, message, priority)
    local actor = Actors.actors[actorId]
    if not actor then
        return false, "Actor not found"
    end
    
    -- Default to normal priority
    local msgPriority = priority or MessagePriority.NORMAL
    
    -- Add message to the appropriate priority queue
    table.insert(actor.priorityQueue[msgPriority], message)
    
    return true
end

-- Send a message and wait for a response
function Actors.ask(actorId, message, timeout)
    local actor = Actors.actors[actorId]
    if not actor then
        error("Actor not found: " .. actorId)
        return nil
    end
    
    -- Create a unique channel for the response
    local responseChannel = HttpService:GenerateGUID(false)
    
    -- Create a promise-like object
    local responsePromise = {
        _resolved = false,
        _result = nil,
        _success = false,
        _callbacks = {}
    }
    
    -- Set up callback for the response
    Actors.callbacks[responseChannel] = function(success, result)
        responsePromise._resolved = true
        responsePromise._success = success
        responsePromise._result = result
        
        -- Call any attached callbacks
        for _, callback in ipairs(responsePromise._callbacks) do
            task.spawn(function()
                callback(success, result)
            end)
        end
        
        -- Clean up
        Actors.callbacks[responseChannel] = nil
    end
    
    -- Add response channel to message
    local messageWithResponse = (type(message) == "table") and message or { content = message }
    messageWithResponse._responseChannel = responseChannel
    
    -- Send the message
    Actors.send(actorId, messageWithResponse, MessagePriority.HIGH)
    
    -- Set up timeout if specified
    if timeout then
        task.delay(timeout, function()
            if not responsePromise._resolved then
                Actors.callbacks[responseChannel](false, "Timeout waiting for response")
            end
        end)
    end
    
    -- Return promise-like object with then method
    function responsePromise:andThen(callback)
        if self._resolved then
            -- Already resolved, call immediately
            task.spawn(function()
                callback(self._success, self._result)
            end)
        else
            -- Add to callback list
            table.insert(self._callbacks, callback)
        end
        return self
    end
    
    function responsePromise:await()
        -- Block until resolved (dangerous! can cause deadlocks if not careful)
        while not self._resolved do
            task.wait(0.01)
        end
        
        if self._success then
            return self._result
        else
            error(self._result)
        end
    end
    
    return responsePromise
end

-- Check if an actor is alive
function Actors.isActorAlive(actorId)
    return Actors.actors[actorId] ~= nil
end

-- Terminate an actor
function Actors.terminate(actorId)
    local actor = Actors.actors[actorId]
    if not actor then
        return false, "Actor not found"
    end
    
    -- Get the Roblox Actor instance for this actor ID
    local robloxActor = actor.instance
    
    -- Set status to terminated
    actor.status = ActorStatus.TERMINATED
    
    -- Record termination
    Actors.terminated[actorId] = os.clock()
    
    -- Notify monitors
    for _, callback in ipairs(actor.monitors) do
        task.spawn(function()
            callback(actorId, "terminated")
        end)
    end
    
    -- Remove from active actors
    Actors.actors[actorId] = nil
    Actors.mailboxes[actorId] = nil
    
    -- Terminate children if any
    for childId, _ in pairs(actor.children) do
        Actors.terminate(childId)
    end
    
    -- Remove the Roblox Actor instance
    if robloxActor then
        robloxActor:Destroy()
    end
    
    return true
end

-- Monitor an actor for termination
function Actors.monitorActor(actorId, callback)
    local actor = Actors.actors[actorId]
    if not actor then
        -- Actor already terminated, call immediately
        task.spawn(function()
            callback(actorId, "terminated")
        end)
        return false
    end
    
    -- Add monitor
    table.insert(actor.monitors, callback)
    return true
end

-- Create a supervised actor that will be restarted if it crashes
function Actors.supervise(childHandler, initialState, name, options)
    local opts = options or {
        maxRestarts = 10,
        restartDelay = 1,
        resetCountAfter = 60
    }
    
    -- Create supervisor actor
    local supervisor = Actors.spawn(function(state, message)
        if message.type == "monitor" then
            -- Child terminated, handle restart logic
            local restartCount = state.restarts[message.actorId] or 0
            restartCount = restartCount + 1
            state.restarts[message.actorId] = restartCount
            
            -- Check if should restart
            if restartCount <= state.maxRestarts then
                -- Schedule restart
                task.delay(state.restartDelay, function()
                    local newChild = Actors.spawn(state.childHandler, state.initialState, state.childName)
                    
                    -- Update current child
                    state.currentChild = newChild.id
                    
                    -- Monitor the new child
                    Actors.monitorActor(newChild.id, function(actorId)
                        Actors.send(state.id, {
                            type = "monitor",
                            actorId = actorId
                        })
                    end)
                    
                    -- Add to children list
                    state.children[newChild.id] = true
                    
                    -- Reset restart counter after a period of successful operation
                    task.delay(state.resetCountAfter, function()
                        if Actors.isActorAlive(newChild.id) then
                            state.restarts[newChild.id] = 0
                        end
                    end)
                end)
                
                return "Child " .. message.actorId .. " scheduled for restart"
            else
                return "Max restarts reached for child " .. message.actorId
            end
        elseif message.type == "forward" then
            -- Forward message to current child
            if state.currentChild then
                return Actors.ask(state.currentChild, message.content)
            else
                return {error = "No active child actor"}
            end
        else
            -- Unknown message type
            return {error = "Unknown message type"}
        end
    end, {
        id = nil,  -- Will be filled after creation
        childHandler = childHandler,
        initialState = initialState,
        childName = name,
        maxRestarts = opts.maxRestarts,
        restartDelay = opts.restartDelay,
        resetCountAfter = opts.resetCountAfter,
        restarts = {},
        children = {},
        currentChild = nil
    })
    
    -- Set the ID in the state
    local supervisorState = Actors.actors[supervisor.id].state
    supervisorState.id = supervisor.id
    
    -- Create initial child
    local child = Actors.spawn(childHandler, initialState, name)
    supervisorState.currentChild = child.id
    supervisorState.children[child.id] = true
    
    -- Monitor child
    Actors.monitorActor(child.id, function(actorId)
        Actors.send(supervisor.id, {
            type = "monitor",
            actorId = actorId
        })
    end)
    
    -- Create a special handle that forwards to child but provides supervision
    local handle = {
        id = supervisor.id,
        childId = child.id,
        send = function(self, message)
            return Actors.send(supervisor.id, {
                type = "forward",
                content = message
            })
        end,
        ask = function(self, message, timeout)
            return Actors.ask(supervisor.id, {
                type = "forward",
                content = message
            }, timeout)
        end,
        terminate = function(self)
            -- Terminate the supervisor and all children
            return Actors.terminate(self.id)
        end
    }
    
    return handle
end

-- Utility for creating an actor pool for load balancing
function Actors.createPool(workerCount, workerHandler, initialState, options)
    local opts = options or {
        distributionStrategy = "round-robin"  -- or "least-busy"
    }
    
    -- Create a pool manager actor
    local poolManager = Actors.spawn(function(state, message)
        if message.type == "internal_status" then
            -- Update worker status
            state.workerStatus[message.workerId] = message.status
            return "Status updated"
        elseif message.type == "work" then
            -- Distribute work among workers
            local selectedWorker
            
            if state.distributionStrategy == "round-robin" then
                -- Simple round-robin
                state.currentWorkerIndex = (state.currentWorkerIndex % #state.workers) + 1
                selectedWorker = state.workers[state.currentWorkerIndex]
            elseif state.distributionStrategy == "least-busy" then
                -- Find least busy worker
                local leastBusyWorker
                local lowestQueueSize = math.huge
                
                for _, worker in ipairs(state.workers) do
                    local queueSize = state.workerQueueSize[worker.id] or 0
                    if queueSize < lowestQueueSize then
                        lowestQueueSize = queueSize
                        leastBusyWorker = worker
                    end
                end
                
                selectedWorker = leastBusyWorker
            end
            
            if selectedWorker then
                -- Increment queue size
                state.workerQueueSize[selectedWorker.id] = (state.workerQueueSize[selectedWorker.id] or 0) + 1
                
                -- Forward the work
                local response = Actors.ask(selectedWorker.id, message.payload)
                
                -- Handle response - set up callback to decrement queue when done
                response:andThen(function()
                    state.workerQueueSize[selectedWorker.id] = math.max(0, (state.workerQueueSize[selectedWorker.id] or 1) - 1)
                end)
                
                return response
            else
                return {error = "No workers available"}
            end
        else
            return {error = "Unknown message type"}
        end
    end, {
        workers = {},
        workerStatus = {},
        workerQueueSize = {},
        currentWorkerIndex = 0,
        distributionStrategy = opts.distributionStrategy
    })
    
    -- Create workers
    local poolState = Actors.actors[poolManager.id].state
    for i = 1, workerCount do
        local worker = Actors.spawn(workerHandler, initialState, "Worker_" .. i)
        table.insert(poolState.workers, worker)
        poolState.workerStatus[worker.id] = ActorStatus.IDLE
        poolState.workerQueueSize[worker.id] = 0
        
        -- Monitor worker status
        Actors.monitorActor(worker.id, function(actorId)
            -- Worker terminated, replace it
            for i, w in ipairs(poolState.workers) do
                if w.id == actorId then
                    local newWorker = Actors.spawn(workerHandler, initialState, "Worker_" .. i)
                    poolState.workers[i] = newWorker
                    poolState.workerStatus[newWorker.id] = ActorStatus.IDLE
                    poolState.workerQueueSize[newWorker.id] = 0
                    
                    -- Monitor the new worker too
                    Actors.monitorActor(newWorker.id, function(actorId)
                        -- This will recursively replace terminated workers
                        Actors.send(poolManager.id, {
                            type = "internal_status",
                            workerId = actorId,
                            status = ActorStatus.TERMINATED
                        })
                    end)
                    
                    break
                end
            end
        end)
    end
    
    -- Return a handle to the pool
    local poolHandle = {
        id = poolManager.id,
        workerCount = workerCount,
        
        -- Submit work to the pool
        submit = function(self, payload, timeout)
            return Actors.ask(self.id, {
                type = "work",
                payload = payload
            }, timeout)
        end,
        
        -- Terminate the entire pool
        terminate = function(self)
            -- Get the workers before terminating the manager
            local workers = Actors.actors[self.id].state.workers
            
            -- Terminate all workers first
            for _, worker in ipairs(workers) do
                Actors.terminate(worker.id)
            end
            
            -- Then terminate the manager
            return Actors.terminate(self.id)
        end
    }
    
    return poolHandle
end

-- Create a shared state that's safe for multiple actors to access
function Actors.createSharedState(initialState)
    -- Create an actor to manage the shared state
    local stateManager = Actors.spawn(function(state, message)
        if message.type == "get" then
            -- Return the entire state or a specific key
            if message.key then
                return state.data[message.key]
            else
                -- Return a copy to prevent direct modification
                local copy = {}
                for k, v in pairs(state.data) do
                    copy[k] = v
                end
                return copy
            end
        elseif message.type == "set" then
            -- Set a specific key
            state.data[message.key] = message.value
            
            -- Notify subscribers
            if state.subscribers[message.key] then
                for _, subInfo in ipairs(state.subscribers[message.key]) do
                    Actors.send(subInfo.actorId, {
                        type = "state_changed",
                        key = message.key,
                        value = message.value,
                        id = subInfo.id
                    })
                end
            end
            
            return true
        elseif message.type == "update" then
            -- Update multiple keys at once
            for k, v in pairs(message.values) do
                state.data[k] = v
                
                -- Notify subscribers for this key
                if state.subscribers[k] then
                    for _, subInfo in ipairs(state.subscribers[k]) do
                        Actors.send(subInfo.actorId, {
                            type = "state_changed",
                            key = k,
                            value = v,
                            id = subInfo.id
                        })
                    end
                end
            end
            
            return true
        elseif message.type == "delete" then
            -- Delete a key
            state.data[message.key] = nil
            
            -- Notify subscribers
            if state.subscribers[message.key] then
                for _, subInfo in ipairs(state.subscribers[message.key]) do
                    Actors.send(subInfo.actorId, {
                        type = "state_changed",
                        key = message.key,
                        value = nil,
                        id = subInfo.id
                    })
                end
            end
            
            return true
        elseif message.type == "subscribe" then
            -- Subscribe to changes on a key
            if not state.subscribers[message.key] then
                state.subscribers[message.key] = {}
            end
            
            local subId = HttpService:GenerateGUID(false)
            table.insert(state.subscribers[message.key], {
                id = subId,
                actorId = message.actorId,
                callback = message.callback
            })
            
            return subId
        elseif message.type == "unsubscribe" then
            -- Unsubscribe from changes
            if state.subscribers[message.key] then
                for i, subInfo in ipairs(state.subscribers[message.key]) do
                    if subInfo.id == message.subscriptionId then
                        table.remove(state.subscribers[message.key], i)
                        return true
                    end
                end
            end
            
            return false
        else
            return {error = "Unknown message type"}
        end
    end, {
        data = initialState or {},
        subscribers = {}
    })
    
    -- Create a proxy object for easier use
    local sharedState = {
        id = stateManager.id,
        
        -- Get a value
        get = function(self, key)
            return Actors.ask(self.id, {
                type = "get",
                key = key
            }):await()
        end,
        
        -- Set a value
        set = function(self, key, value)
            return Actors.ask(self.id, {
                type = "set",
                key = key,
                value = value
            }):await()
        end,
        
        -- Update multiple values at once
        update = function(self, values)
            return Actors.ask(self.id, {
                type = "update",
                values = values
            }):await()
        end,
        
        -- Delete a key
        delete = function(self, key)
            return Actors.ask(self.id, {
                type = "delete",
                key = key
            }):await()
        end,
        
        -- Subscribe to changes on a key
        subscribe = function(self, key, actorId, callback)
            return Actors.ask(self.id, {
                type = "subscribe",
                key = key,
                actorId = actorId,
                callback = callback
            }):await()
        end,
        
        -- Unsubscribe from changes
        unsubscribe = function(self, key, subscriptionId)
            return Actors.ask(self.id, {
                type = "unsubscribe",
                key = key,
                subscriptionId = subscriptionId
            }):await()
        end
    }
    
    return sharedState
end

-- Constants exposed in the public API
Actors.Priority = {
    HIGH = MessagePriority.HIGH,
    NORMAL = MessagePriority.NORMAL,
    LOW = MessagePriority.LOW
}

return Actors
