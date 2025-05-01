use std::fmt;

/// Actor system implementation for Roblox-RS
pub struct ActorSystem;

impl ActorSystem {
    /// Generate the actor system for Roblox
    pub fn generate() -> String {
        // Include our SimpleActorSystem implementation
        String::from(r#"-- Actor System for Roblox-RS
-- Implements a Rust-like actor model that works within Roblox's limitations
local HttpService = game:GetService("HttpService")

-- Actor system implementation
local ActorSystem = {}

-- Private state
local actors = {}
local messageQueues = {}
local handlers = {}
local actorCounter = 0
local callbacks = {}

-- Message priority levels
ActorSystem.Priority = {
    HIGH = 1,
    NORMAL = 2,
    LOW = 3
}

-- Actor handle object
local ActorHandle = {}
ActorHandle.__index = ActorHandle

function ActorHandle.new(id)
    local self = setmetatable({}, ActorHandle)
    self.id = id
    return self
end

function ActorHandle:send(message, priority)
    priority = priority or ActorSystem.Priority.NORMAL
    
    if not messageQueues[self.id] then
        return false, "Actor not found"
    end
    
    -- Add to the message queue with the appropriate priority
    table.insert(messageQueues[self.id][priority], message)
    return true
end

function ActorHandle:ask(message, timeout)
    -- Generate a unique response ID
    local responseId = HttpService:GenerateGUID(false)
    
    -- Create a promise-like object
    local promise = {
        _resolved = false,
        _result = nil,
        _success = false,
        _callbacks = {}
    }
    
    -- Set up the callback for when we get a response
    callbacks[responseId] = function(success, result)
        promise._resolved = true
        promise._success = success
        promise._result = result
        
        -- Call any attached callbacks
        for _, callback in ipairs(promise._callbacks) do
            task.spawn(function()
                callback(success, result)
            end)
        end
    end
    
    -- Add responseId to the message
    if type(message) ~= "table" then
        message = { value = message }
    end
    message._responseId = responseId
    
    -- Send the message with high priority
    self:send(message, ActorSystem.Priority.HIGH)
    
    -- Set up timeout if requested
    if timeout then
        task.delay(timeout, function()
            if not promise._resolved then
                callbacks[responseId](false, "Timeout waiting for response")
            end
        end)
    end
    
    -- Add methods to the promise
    function promise:andThen(callback)
        if self._resolved then
            task.spawn(function()
                callback(self._success, self._result)
            end)
        else
            table.insert(self._callbacks, callback)
        end
        return self
    end
    
    function promise:await()
        -- Block until resolved
        while not self._resolved do
            task.wait(0.01)
        end
        
        if self._success then
            return self._result
        else
            error(self._result)
        end
    end
    
    return promise
end

function ActorHandle:isAlive()
    return actors[self.id] ~= nil
end

function ActorHandle:terminate()
    if not actors[self.id] then
        return false, "Actor not found"
    end
    
    -- Clean up actor resources
    actors[self.id] = nil
    messageQueues[self.id] = nil
    handlers[self.id] = nil
    
    -- Additional cleanup for supervised actors, etc.
    
    return true
end

-- Create a new actor
function ActorSystem.spawn(handler, initialState, name)
    actorCounter = actorCounter + 1
    local id = HttpService:GenerateGUID(false)
    local displayName = name or "Actor_" .. actorCounter
    
    -- Initialize actor state
    actors[id] = {
        id = id,
        name = displayName,
        state = initialState or {},
        created = os.clock()
    }
    
    -- Create message queues (one for each priority)
    messageQueues[id] = {
        [ActorSystem.Priority.HIGH] = {},
        [ActorSystem.Priority.NORMAL] = {},
        [ActorSystem.Priority.LOW] = {}
    }
    
    -- Store handler function
    handlers[id] = handler
    
    -- Start the message processing loop
    task.spawn(function()
        while actors[id] do
            local message = nil
            
            -- Check queues in priority order
            for priority = ActorSystem.Priority.HIGH, ActorSystem.Priority.LOW do
                if #messageQueues[id][priority] > 0 then
                    message = table.remove(messageQueues[id][priority], 1)
                    break
                end
            end
            
            if message then
                -- Process the message
                local success, result = pcall(function()
                    return handlers[id](actors[id].state, message)
                end)
                
                -- Handle response if needed
                if message._responseId and callbacks[message._responseId] then
                    callbacks[message._responseId](success, result)
                    callbacks[message._responseId] = nil
                end
                
                -- Immediate processing of next message
            else
                -- No messages, wait a bit
                task.wait(0.05)
            end
        end
    end)
    
    -- Return handle to the actor
    return ActorHandle.new(id)
end

-- Create a supervised actor that will be restarted if it crashes
function ActorSystem.supervise(childHandler, initialState, name, options)
    local opts = options or {
        maxRestarts = 10,
        restartDelay = 1,
        resetCountAfter = 60
    }
    
    -- Create supervisor actor
    local supervisor = ActorSystem.spawn(function(state, message)
        if message.type == "monitor" then
            -- Child terminated, handle restart logic
            local restartCount = state.restarts[message.actorId] or 0
            restartCount = restartCount + 1
            state.restarts[message.actorId] = restartCount
            
            -- Check if should restart
            if restartCount <= state.maxRestarts then
                -- Schedule restart
                task.delay(state.restartDelay, function()
                    local newChild = ActorSystem.spawn(state.childHandler, state.initialState, state.childName)
                    
                    -- Update current child
                    state.currentChild = newChild.id
                    
                    -- Add to children list
                    state.children[newChild.id] = true
                end)
                
                return "Child " .. message.actorId .. " scheduled for restart"
            else
                return "Max restarts reached for child " .. message.actorId
            end
        elseif message.type == "forward" then
            -- Forward message to current child
            if state.currentChild then
                -- Create a handle for the child
                local childHandle = ActorHandle.new(state.currentChild)
                
                if message._responseId then
                    -- For 'ask' pattern, directly forward the payload and return the result
                    local childPromise = childHandle:ask(message.content)
                    
                    -- Instead of returning immediately, we want to return the actual result
                    -- from the child actor, so we'll wait for it and return that
                    local success, result = pcall(function()
                        return childPromise:await()
                    end)
                    
                    -- Return the actual child result
                    if success then
                        return result
                    else
                        return "Error from child: " .. tostring(result)
                    end
                else
                    -- Simple send
                    childHandle:send(message.content)
                    return "Message forwarded"
                end
            else
                return {error = "No active child actor"}
            end
        else
            -- Unknown message type
            return {error = "Unknown message type"}
        end
    end, {
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
    
    -- Create initial child
    local child = ActorSystem.spawn(childHandler, initialState, name)
    
    -- Set the current child in supervisor's state
    actors[supervisor.id].state.currentChild = child.id
    actors[supervisor.id].state.children = {[child.id] = true}
    
    -- Create a special handle that forwards to child but provides supervision
    local handle = {
        id = supervisor.id,
        childId = child.id,
        send = function(self, message)
            local supervisorHandle = ActorHandle.new(self.id)
            return supervisorHandle:send({
                type = "forward",
                content = message
            })
        end,
        ask = function(self, message, timeout)
            local supervisorHandle = ActorHandle.new(self.id)
            return supervisorHandle:ask({
                type = "forward",
                content = message
            }, timeout)
        end,
        terminate = function(self)
            -- Terminate the supervisor and all children
            local supervisorHandle = ActorHandle.new(self.id)
            return supervisorHandle:terminate()
        end
    }
    
    return handle
end

-- Create a pool of workers for load balancing
function ActorSystem.createPool(workerCount, workerHandler, initialState, options)
    local opts = options or {
        distributionStrategy = "round-robin"  -- or "least-busy"
    }
    
    -- Create a pool manager actor
    local poolManager = ActorSystem.spawn(function(state, message)
        if message.type == "work" then
            -- Distribute work among workers
            local selectedWorker
            
            if state.distributionStrategy == "round-robin" then
                -- Simple round-robin
                state.currentWorkerIndex = (state.currentWorkerIndex % #state.workers) + 1
                selectedWorker = state.workers[state.currentWorkerIndex]
            else
                -- Use "least-busy" strategy by default
                local leastBusyWorker = state.workers[1]
                local lowestQueueSize = #messageQueues[leastBusyWorker][ActorSystem.Priority.NORMAL]
                
                for i = 2, #state.workers do
                    local workerIndex = state.workers[i]
                    local queueSize = #messageQueues[workerIndex][ActorSystem.Priority.NORMAL]
                    if queueSize < lowestQueueSize then
                        lowestQueueSize = queueSize
                        leastBusyWorker = workerIndex
                    end
                end
                
                selectedWorker = leastBusyWorker
            end
            
            if selectedWorker then
                -- Forward the work to the selected worker
                local workerHandle = ActorHandle.new(selectedWorker)
                
                if message._responseId then
                    -- For 'ask' pattern, directly forward the payload and return the result
                    local workerPromise = workerHandle:ask(message.payload)
                    local success, result = pcall(function()
                        return workerPromise:await()
                    end)
                    
                    -- Return the actual worker result
                    if success then
                        return result
                    else
                        return "Error from worker: " .. tostring(result)
                    end
                else
                    -- Simple send
                    workerHandle:send(message.payload)
                    return true
                end
            else
                return {error = "No workers available"}
            end
        else
            return {error = "Unknown message type"}
        end
    end, {
        workers = {},
        currentWorkerIndex = 0,
        distributionStrategy = opts.distributionStrategy
    })
    
    -- Create workers
    local workerIds = {}
    for i = 1, workerCount do
        local worker = ActorSystem.spawn(workerHandler, initialState, "Worker_" .. i)
        table.insert(workerIds, worker.id)
    end
    
    -- Set workers in the pool manager's state
    actors[poolManager.id].state.workers = workerIds
    
    -- Return a handle to the pool
    local poolHandle = {
        id = poolManager.id,
        
        -- Submit work to the pool
        submit = function(self, payload, timeout)
            local poolHandle = ActorHandle.new(self.id)
            return poolHandle:ask({
                type = "work",
                payload = payload
            }, timeout)
        end,
        
        -- Terminate the entire pool
        terminate = function(self)
            local poolHandle = ActorHandle.new(self.id)
            
            -- First terminate all workers
            for _, workerId in ipairs(actors[self.id].state.workers) do
                local workerHandle = ActorHandle.new(workerId)
                workerHandle:terminate()
            end
            
            -- Then terminate the pool manager
            return poolHandle:terminate()
        end
    }
    
    return poolHandle
end

-- Create a shared state that's safe for multiple actors to access
function ActorSystem.createSharedState(initialState)
    -- Create an actor to manage the shared state
    local stateManager = ActorSystem.spawn(function(state, message)
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
                for actorId, _ in pairs(state.subscribers[message.key]) do
                    local actor = ActorHandle.new(actorId)
                    actor:send({
                        type = "state_changed",
                        key = message.key,
                        value = message.value
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
                    for actorId, _ in pairs(state.subscribers[k]) do
                        local actor = ActorHandle.new(actorId)
                        actor:send({
                            type = "state_changed",
                            key = k,
                            value = v
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
                for actorId, _ in pairs(state.subscribers[message.key]) do
                    local actor = ActorHandle.new(actorId)
                    actor:send({
                        type = "state_changed",
                        key = message.key,
                        value = nil
                    })
                end
            end
            
            return true
        elseif message.type == "subscribe" then
            -- Subscribe to changes on a key
            if not state.subscribers[message.key] then
                state.subscribers[message.key] = {}
            end
            
            state.subscribers[message.key][message.actorId] = true
            return true
        elseif message.type == "unsubscribe" then
            -- Unsubscribe from changes
            if state.subscribers[message.key] then
                state.subscribers[message.key][message.actorId] = nil
                return true
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
            local handle = ActorHandle.new(self.id)
            return handle:ask({
                type = "get",
                key = key
            }):await()
        end,
        
        -- Set a value
        set = function(self, key, value)
            local handle = ActorHandle.new(self.id)
            return handle:ask({
                type = "set",
                key = key,
                value = value
            }):await()
        end,
        
        -- Update multiple values at once
        update = function(self, values)
            local handle = ActorHandle.new(self.id)
            return handle:ask({
                type = "update",
                values = values
            }):await()
        end,
        
        -- Delete a key
        delete = function(self, key)
            local handle = ActorHandle.new(self.id)
            return handle:ask({
                type = "delete",
                key = key
            }):await()
        end,
        
        -- Subscribe to changes on a key
        subscribe = function(self, key, actorId)
            local handle = ActorHandle.new(self.id)
            return handle:ask({
                type = "subscribe",
                key = key,
                actorId = actorId
            }):await()
        end,
        
        -- Unsubscribe from changes
        unsubscribe = function(self, key, actorId)
            local handle = ActorHandle.new(self.id)
            return handle:ask({
                type = "unsubscribe",
                key = key,
                actorId = actorId
            }):await()
        end
    }
    
    return sharedState
end

return ActorSystem"#)
    }
}

/// Module to properly define and export Rust's standard async/await actors pattern to Luau
pub mod actors {
    use super::ActorSystem;
}

/// Generate the Rust-compatible actor system interface
pub fn generate_actor_system() -> String {
    ActorSystem::generate()
}
    
/// Generate the Rust actor trait interface
pub fn generate_actor_trait() -> String {
        String::from(r#"-- Rust Actor trait interface for Roblox-RS
local RustActor = {}

-- Actor trait implementation
function RustActor.implement(handler)
    -- Create an actor with the Roblox-RS actor system
    local actor = RobloxRS.Actors.spawn(handler)
    
    -- Return a Rust-like interface
    return {
        tell = function(self, message)
            actor:send(message)
        end,
        
        ask = function(self, message, timeout)
            return actor:ask(message, timeout)
        end,
        
        stop = function(self)
            actor:terminate()
        end
    }
end

-- Context implementation (similar to Rust's ActorContext)
function RustActor.Context(state)
    return {
        state = state,
        
        -- Access to self (actor reference)
        self = function(self)
            return state.self
        end,
        
        -- Watch other actors (monitor them)
        watch = function(self, actor)
            -- Implementation depends on actor system's monitoring capabilities
            if actor and actor.id then
                -- Add watcher
                actor:monitor(function(actorId, reason)
                    -- Handle termination notification
                    if state.watchers and state.watchers[actorId] then
                        local callback = state.watchers[actorId]
                        if callback then
                            callback(reason)
                        end
                    end
                end)
                
                return true
            end
            return false
        end,
        
        -- Spawn a child actor
        spawn = function(self, childHandler, childInitialState, childName)
            local child = RobloxRS.Actors.spawn(childHandler, childInitialState, childName)
            
            -- Register as a child
            if not state.children then
                state.children = {}
            end
            state.children[child.id] = child
            
            return child
        end
    }
end

return RustActor"#)
    }
    
/// Generate the Rust async/await compatibility layer
pub fn generate_async_compatibility() -> String {
        String::from(r#"-- Rust async/await compatibility for Roblox-RS
local RustAsync = {}

-- Future implementation (similar to Rust's Future trait)
function RustAsync.Future(promise)
    -- Take a Promise-like object and adapt it to Rust Future interface
    return {
        -- Poll method (similar to Rust's Future::poll)
        poll = function(self)
            if promise._resolved then
                if promise._success then
                    return { ready = true, value = promise._result }
                else
                    return { ready = true, error = promise._result }
                end
            else
                return { ready = false }
            end
        end,
        
        -- Await the future (block until complete - use carefully!)
        await = function(self)
            return promise:await()
        end,
        
        -- Add callback for when future completes
        then_do = function(self, callback)
            promise:andThen(function(success, result)
                callback(success, result)
            end)
            return self
        end
    }
end

-- Create a future that resolves after a delay (similar to Rust's tokio::time::sleep)
function RustAsync.sleep(seconds)
    local promise = {
        _resolved = false,
        _result = nil,
        _success = false,
        _callbacks = {}
    }
    
    -- Schedule resolution
    task.delay(seconds, function()
        promise._resolved = true
        promise._success = true
        promise._result = nil
        
        -- Call any attached callbacks
        for _, callback in ipairs(promise._callbacks) do
            task.spawn(function()
                callback(true, nil)
            end)
        end
    end)
    
    -- Add methods to the promise
    function promise:andThen(callback)
        if self._resolved then
            task.spawn(function()
                callback(self._success, self._result)
            end)
        else
            table.insert(self._callbacks, callback)
        end
        return self
    end
    
    function promise:await()
        -- Block until resolved
        while not self._resolved do
            task.wait(0.01)
        end
        
        if self._success then
            return self._result
        else
            error(self._result)
        end
    end
    
    return RustAsync.Future(promise)
end

-- Join multiple futures (similar to Rust's futures::join)
function RustAsync.join(...)
    local futures = {...}
    local results = {}
    local promise = {
        _resolved = false,
        _result = nil,
        _success = false,
        _callbacks = {}
    }
    
    local completed = 0
    local total = #futures
    
    -- Wait for all futures to complete
    for i, future in ipairs(futures) do
        future:then_do(function(success, result)
            results[i] = {
                success = success,
                result = result
            }
            
            completed = completed + 1
            
            -- If all are done, resolve the promise
            if completed >= total then
                promise._resolved = true
                promise._success = true
                promise._result = results
                
                -- Call any attached callbacks
                for _, callback in ipairs(promise._callbacks) do
                    task.spawn(function()
                        callback(true, results)
                    end)
                end
            end
        end)
    end
    
    -- Add methods to the promise
    function promise:andThen(callback)
        if self._resolved then
            task.spawn(function()
                callback(self._success, self._result)
            end)
        else
            table.insert(self._callbacks, callback)
        end
        return self
    end
    
    function promise:await()
        -- Block until resolved
        while not self._resolved do
            task.wait(0.01)
        end
        
        if self._success then
            return self._result
        else
            error(self._result)
        end
    end
    
    return RustAsync.Future(promise)
end

return RustAsync"#)
}
