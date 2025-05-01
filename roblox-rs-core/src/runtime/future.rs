// Future module for Roblox-RS runtime
// Provides promise-like future implementation for async operations

// Generate the future implementation
pub fn generate_future_lib() -> String {
    r#"
-- Future implementation for Roblox-RS
-- Provides a promise-like API for asynchronous operations
RobloxRS.Future = {}

-- Create a new future
function RobloxRS.Future.new()
    local future = {
        _resolved = false,
        _rejected = false,
        _value = nil,
        _error = nil,
        _onResolve = {},
        _onReject = {},
        _onFinally = {}
    }
    
    -- Resolve the future
    future.resolve = function(value)
        if future._resolved or future._rejected then
            return
        end
        
        future._resolved = true
        future._value = value
        
        -- Call resolve callbacks
        for _, callback in ipairs(future._onResolve) do
            task.spawn(function()
                callback(value)
            end)
        end
        
        -- Call finally callbacks
        for _, callback in ipairs(future._onFinally) do
            task.spawn(function()
                callback()
            end)
        end
    end
    
    -- Reject the future
    future.reject = function(error)
        if future._resolved or future._rejected then
            return
        end
        
        future._rejected = true
        future._error = error
        
        -- Call reject callbacks
        for _, callback in ipairs(future._onReject) do
            task.spawn(function()
                callback(error)
            end)
        end
        
        -- Call finally callbacks
        for _, callback in ipairs(future._onFinally) do
            task.spawn(function()
                callback()
            end)
        end
    end
    
    -- Then method to chain futures
    future.andThen = function(self, onResolve, onReject)
        local nextFuture = RobloxRS.Future.new()
        
        -- Handle resolution
        local resolveCallback = function(value)
            if onResolve then
                local success, result = pcall(onResolve, value)
                
                if success then
                    if type(result) == "table" and result.andThen then
                        -- Result is another future, chain them
                        result:andThen(function(chainedResult)
                            nextFuture:resolve(chainedResult)
                        end, function(chainedError)
                            nextFuture:reject(chainedError)
                        end)
                    else
                        -- Normal value, resolve with it
                        nextFuture:resolve(result)
                    end
                else
                    -- Error in callback, reject
                    nextFuture:reject(result)
                end
            else
                -- No onResolve, pass through the value
                nextFuture:resolve(value)
            end
        end
        
        -- Handle rejection
        local rejectCallback = function(error)
            if onReject then
                local success, result = pcall(onReject, error)
                
                if success then
                    nextFuture:resolve(result)
                else
                    nextFuture:reject(result)
                end
            else
                -- No onReject, pass through the error
                nextFuture:reject(error)
            end
        end
        
        -- Register callbacks
        if self._resolved then
            task.spawn(function()
                resolveCallback(self._value)
            end)
        elseif self._rejected then
            task.spawn(function()
                rejectCallback(self._error)
            end)
        else
            table.insert(self._onResolve, resolveCallback)
            table.insert(self._onReject, rejectCallback)
        end
        
        return nextFuture
    end
    
    -- Syntactic sugar for andThen(nil, onReject)
    future.catch = function(self, onReject)
        return self:andThen(nil, onReject)
    end
    
    -- Finally method to run code regardless of resolution
    future.finally = function(self, onFinally)
        if onFinally then
            if self._resolved or self._rejected then
                task.spawn(onFinally)
            else
                table.insert(self._onFinally, onFinally)
            end
        end
        
        return self
    end
    
    -- Await method that blocks until the future is resolved/rejected
    future.await = function(self, timeout)
        if self._resolved then
            return self._value
        elseif self._rejected then
            error(self._error)
        end
        
        -- Wait for resolution
        local result, waitError, timedOut = nil, nil, false
        local startTime = os.clock()
        
        -- Set up temporary callbacks
        local waitDone = false
        
        local tempResolve = function(value)
            if waitDone then return end
            result = value
            waitDone = true
        end
        
        local tempReject = function(err)
            if waitDone then return end
            waitError = err
            waitDone = true
        end
        
        table.insert(self._onResolve, tempResolve)
        table.insert(self._onReject, tempReject)
        
        -- Wait loop
        while not waitDone do
            task.wait(0.03)
            
            if timeout and os.clock() - startTime > timeout then
                timedOut = true
                waitDone = true
            end
        end
        
        -- Clean up callbacks
        for i, callback in ipairs(self._onResolve) do
            if callback == tempResolve then
                table.remove(self._onResolve, i)
                break
            end
        end
        
        for i, callback in ipairs(self._onReject) do
            if callback == tempReject then
                table.remove(self._onReject, i)
                break
            end
        end
        
        if timedOut then
            error("Future timed out after " .. timeout .. " seconds")
        elseif waitError then
            error(waitError)
        else
            return result
        end
    end
    
    return future
end

-- Create a resolved future
function RobloxRS.Future.resolve(value)
    local future = RobloxRS.Future.new()
    future:resolve(value)
    return future
end

-- Create a rejected future
function RobloxRS.Future.reject(error)
    local future = RobloxRS.Future.new()
    future:reject(error)
    return future
end

-- Wait for all futures to resolve
function RobloxRS.Future.all(futures)
    local allFuture = RobloxRS.Future.new()
    local results = {}
    local remaining = #futures
    
    if remaining == 0 then
        allFuture:resolve({})
        return allFuture
    end
    
    for i, future in ipairs(futures) do
        future:andThen(function(value)
            results[i] = value
            remaining = remaining - 1
            
            if remaining == 0 then
                allFuture:resolve(results)
            end
        end, function(error)
            allFuture:reject(error)
        end)
    end
    
    return allFuture
end

-- Wait for any future to resolve
function RobloxRS.Future.any(futures)
    local anyFuture = RobloxRS.Future.new()
    local errors = {}
    local remaining = #futures
    
    if remaining == 0 then
        anyFuture:reject("No futures provided to Future.any")
        return anyFuture
    end
    
    for _, future in ipairs(futures) do
        future:andThen(function(value)
            anyFuture:resolve(value)
        end, function(error)
            table.insert(errors, error)
            remaining = remaining - 1
            
            if remaining == 0 then
                anyFuture:reject({
                    message = "All futures rejected",
                    errors = errors
                })
            end
        end)
    end
    
    return anyFuture
end

-- Race futures - resolve/reject with the first one to complete
function RobloxRS.Future.race(futures)
    local raceFuture = RobloxRS.Future.new()
    
    for _, future in ipairs(futures) do
        future:andThen(function(value)
            raceFuture:resolve(value)
        end, function(error)
            raceFuture:reject(error)
        end)
    end
    
    return raceFuture
end
"#.to_string()
}
