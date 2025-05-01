/// Generate the runtime helper library as a Luau string
pub fn generate_runtime_lib() -> String {
    r#"--[[
    Roblox-RS Runtime Helpers
    This module provides runtime support for Rust-compiled Luau code.
    Includes utilities for:
    - Object pooling
    - Parallel execution
    - Debugging/tracing
    - Profiling
    - Native Roblox type helpers
    ]]--

local RobloxRS = {}

-- Object Pooling API
RobloxRS.Pool = {}

-- Create a new instance pool
function RobloxRS.Pool.new(className, initialSize)
    initialSize = initialSize or 10
    
    local pool = {
        className = className,
        available = {},
        active = {},
        created = 0
    }
    
    -- Pre-populate the pool
    for i = 1, initialSize do
        local instance = Instance.new(className)
        instance.Name = className .. "_pooled_" .. i
        table.insert(pool.available, instance)
        pool.created = pool.created + 1
    end
    
    -- Get an object from the pool or create a new one
    function pool:get()
        local instance
        
        if #self.available > 0 then
            instance = table.remove(self.available)
        else
            instance = Instance.new(self.className)
            instance.Name = self.className .. "_pooled_" .. (self.created + 1)
            self.created = self.created + 1
        end
        
        self.active[instance] = true
        return instance
    end
    
    -- Return an object to the pool
    function pool:release(instance)
        if self.active[instance] then
            self.active[instance] = nil
            table.insert(self.available, instance)
        end
    end
    
    -- Get pool statistics
    function pool:stats()
        local activeCount = 0
        for _ in pairs(self.active) do
            activeCount = activeCount + 1
        end
        
        return {
            allocated = self.created,
            available = #self.available,
            active = activeCount
        }
    end
    
    return pool
end

-- Parallel Execution API
RobloxRS.Parallel = {}

-- Parallel forEach implementation using coroutines
function RobloxRS.Parallel.forEach(array, callback)
    local threads = {}
    local chunkSize = math.max(1, math.floor(#array / 8)) -- Divide into 8 chunks
    
    for i = 1, #array, chunkSize do
        local thread = coroutine.create(function()
            for j = i, math.min(i + chunkSize - 1, #array) do
                callback(array[j], j, array)
            end
        end)
        
        table.insert(threads, thread)
    end
    
    -- Resume all threads
    while #threads > 0 do
        local i = 1
        while i <= #threads do
            local thread = threads[i]
            local success, error = coroutine.resume(thread)
            
            if not success then
                warn("Error in parallel forEach: " .. tostring(error))
            end
            
            if coroutine.status(thread) == "dead" then
                table.remove(threads, i)
            else
                i = i + 1
            end
            
            -- Yield to prevent freezing
            task.wait()
        end
    end
end

-- Parallel map implementation
function RobloxRS.Parallel.map(array, callback)
    local result = table.create(#array)
    local resultLock = {}
    
    local threads = {}
    local chunkSize = math.max(1, math.floor(#array / 8))
    
    for i = 1, #array, chunkSize do
        local thread = coroutine.create(function()
            for j = i, math.min(i + chunkSize - 1, #array) do
                result[j] = callback(array[j], j, array)
            end
        end)
        
        table.insert(threads, thread)
    end
    
    -- Resume all threads
    while #threads > 0 do
        local i = 1
        while i <= #threads do
            local thread = threads[i]
            local success, error = coroutine.resume(thread)
            
            if not success then
                warn("Error in parallel map: " .. tostring(error))
            end
            
            if coroutine.status(thread) == "dead" then
                table.remove(threads, i)
            else
                i = i + 1
            end
            
            -- Yield to prevent freezing
            task.wait()
        end
    end
    
    return result
end

-- Parallel filter implementation
function RobloxRS.Parallel.filter(array, predicate)
    local temp = {}
    local tempLock = {}
    
    RobloxRS.Parallel.forEach(array, function(value, index)
        if predicate(value, index, array) then
            table.insert(temp, value)
        end
    end)
    
    return temp
end

-- Rust-like Result type
RobloxRS.Result = {}

function RobloxRS.Result.ok(value)
    return {
        success = true,
        value = value,
        error = nil,
        
        -- Methods
        isOk = function(self)
            return self.success
        end,
        
        isErr = function(self)
            return not self.success
        end,
        
        unwrap = function(self)
            if self.success then
                return self.value
            else
                error("Called unwrap on an error result: " .. tostring(self.error), 2)
            end
        end,
        
        unwrapOr = function(self, default)
            if self.success then
                return self.value
            else
                return default
            end
        end
    }
end

function RobloxRS.Result.err(error)
    return {
        success = false,
        value = nil,
        error = error,
        
        -- Methods
        isOk = function(self)
            return self.success
        end,
        
        isErr = function(self)
            return not self.success
        end,
        
        unwrap = function(self)
            if self.success then
                return self.value
            else
                error("Called unwrap on an error result: " .. tostring(self.error), 2)
            end
        end,
        
        unwrapOr = function(self, default)
            if self.success then
                return self.value
            else
                return default
            end
        end
    }
end

-- Debug / Profiling API
RobloxRS.Debug = {
    callStack = {},
    breakpoints = {},
    watches = {}
}

-- Trace a function call
function RobloxRS.Debug.traceCall(funcName, args)
    table.insert(RobloxRS.Debug.callStack, {
        name = funcName,
        args = args or {},
        line = debug.info(2, "l"),
        time = os.clock()
    })
    
    -- Keep the stack from growing too large
    if #RobloxRS.Debug.callStack > 100 then
        table.remove(RobloxRS.Debug.callStack, 1)
    end
end

-- Get the current call stack
function RobloxRS.Debug.getCallStack()
    return RobloxRS.Debug.callStack
end

-- Clear the call stack
function RobloxRS.Debug.clearCallStack()
    RobloxRS.Debug.callStack = {}
end

-- Set a breakpoint
function RobloxRS.Debug.setBreakpoint(line, condition)
    RobloxRS.Debug.breakpoints[line] = condition or true
end

-- Check if we hit a breakpoint
function RobloxRS.Debug.checkBreakpoint(line, context)
    local bp = RobloxRS.Debug.breakpoints[line]
    if bp == nil then
        return false
    end
    
    if type(bp) == "function" then
        return bp(context)
    else
        return true
    end
end

-- Watch a variable
function RobloxRS.Debug.watch(name, getValue)
    RobloxRS.Debug.watches[name] = getValue
end

-- Get all watch values
function RobloxRS.Debug.getWatchValues()
    local values = {}
    for name, getter in pairs(RobloxRS.Debug.watches) do
        local success, value = pcall(getter)
        values[name] = success and value or "Error: " .. tostring(value)
    end
    return values
end

-- Profiling API
RobloxRS.Profiler = {
    enabled = false,
    data = {
        calls = {},
        times = {},
        memory = {}
    }
}

-- Start the profiler
function RobloxRS.Profiler.start(funcName)
    if not RobloxRS.Profiler.enabled then
        return function() end
    end
    
    local startTime = os.clock()
    local startMemory = gcinfo()
    
    RobloxRS.Profiler.data.calls[funcName] = (RobloxRS.Profiler.data.calls[funcName] or 0) + 1
    
    return function()
        local endTime = os.clock()
        local endMemory = gcinfo()
        
        RobloxRS.Profiler.data.times[funcName] = (RobloxRS.Profiler.data.times[funcName] or 0) + (endTime - startTime)
        RobloxRS.Profiler.data.memory[funcName] = (RobloxRS.Profiler.data.memory[funcName] or 0) + (endMemory - startMemory)
    end
end

-- Enable/disable the profiler
function RobloxRS.Profiler.enable(enabled)
    RobloxRS.Profiler.enabled = enabled == true
    
    if not enabled then
        RobloxRS.Profiler.reset()
    end
    
    return RobloxRS.Profiler.enabled
end

-- Reset profiler data
function RobloxRS.Profiler.reset()
    RobloxRS.Profiler.data = {
        calls = {},
        times = {},
        memory = {}
    }
end

-- Get profiling results
function RobloxRS.Profiler.getResults()
    local results = {}
    
    for funcName, callCount in pairs(RobloxRS.Profiler.data.calls) do
        table.insert(results, {
            name = funcName,
            calls = callCount,
            totalTime = RobloxRS.Profiler.data.times[funcName] or 0,
            avgTime = (RobloxRS.Profiler.data.times[funcName] or 0) / callCount,
            totalMemory = RobloxRS.Profiler.data.memory[funcName] or 0,
            avgMemory = (RobloxRS.Profiler.data.memory[funcName] or 0) / callCount
        })
    end
    
    -- Sort by total time
    table.sort(results, function(a, b)
        return a.totalTime > b.totalTime
    end)
    
    return results
end

-- Table utilities
RobloxRS.Table = {}

-- Deep copy a table
function RobloxRS.Table.deepCopy(original)
    local copy = {}
    for k, v in pairs(original) do
        if type(v) == "table" then
            copy[k] = RobloxRS.Table.deepCopy(v)
        else
            copy[k] = v
        end
    end
    return copy
end

-- Shallow copy a table
function RobloxRS.Table.shallowCopy(original)
    local copy = {}
    for k, v in pairs(original) do
        copy[k] = v
    end
    return copy
end

-- Vector helper utilities
RobloxRS.Vector = {}

-- Convert a table to Vector3
function RobloxRS.Vector.toVector3(tbl)
    return Vector3.new(tbl.x or tbl[1] or 0, tbl.y or tbl[2] or 0, tbl.z or tbl[3] or 0)
end

-- Convert Vector3 to a table
function RobloxRS.Vector.toTable(vec)
    return {x = vec.X, y = vec.Y, z = vec.Z}
end

-- Color helper utilities
RobloxRS.Color = {}

-- Convert a table to Color3
function RobloxRS.Color.toColor3(tbl)
    return Color3.new(tbl.r or tbl[1] or 0, tbl.g or tbl[2] or 0, tbl.b or tbl[3] or 0)
end

-- Convert Color3 to a table
function RobloxRS.Color.toTable(color)
    return {r = color.R, g = color.G, b = color.B}
end

return RobloxRS
"#.to_string()
}
