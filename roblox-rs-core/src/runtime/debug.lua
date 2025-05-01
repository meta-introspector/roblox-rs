-- RobloxRS Debug Runtime
local RobloxRS = RobloxRS or {}
RobloxRS.Debug = RobloxRS.Debug or {}

-- Breakpoint management
RobloxRS.Debug.breakpoints = {}
RobloxRS.Debug.watches = {}
RobloxRS.Debug.locals = {}
RobloxRS.Debug.call_stack = {}

-- Add a breakpoint
function RobloxRS.Debug.add_breakpoint(line, condition)
    RobloxRS.Debug.breakpoints[line] = {
        condition = condition,
        enabled = true,
        hit_count = 0,
        watches = {}
    }
end

-- Add a watch expression
function RobloxRS.Debug.add_watch(expression, condition)
    RobloxRS.Debug.watches[expression] = {
        condition = condition,
        last_value = nil
    }
end

-- Evaluate a condition in the current context
function RobloxRS.Debug.evaluate_condition(condition, locals)
    -- Create a temporary environment with locals
    local env = setmetatable({}, {__index = _G})
    for name, value in pairs(locals) do
        env[name] = value
    end
    
    -- Create the function with the condition
    local fn, err = loadstring("return " .. condition)
    if not fn then
        return false
    end
    
    -- Set the environment and execute
    setfenv(fn, env)
    local success, result = pcall(fn)
    return success and result
end

-- Check if we should break at current line
function RobloxRS.Debug.check_breakpoint(line, locals)
    local breakpoint = RobloxRS.Debug.breakpoints[line]
    if not breakpoint or not breakpoint.enabled then
        return
    end
    
    breakpoint.hit_count = breakpoint.hit_count + 1
    
    -- Store locals for inspection
    RobloxRS.Debug.locals = locals
    
    -- Check condition if present
    if breakpoint.condition then
        if not RobloxRS.Debug.evaluate_condition(breakpoint.condition, locals) then
            return
        end
    end
    
    -- Update watches
    for expr, watch in pairs(RobloxRS.Debug.watches) do
        watch.last_value = RobloxRS.Debug.evaluate_condition(expr, locals)
    end
    
    -- Capture call stack
    RobloxRS.Debug.capture_call_stack()
    
    -- Signal debugger that we hit a breakpoint
    RobloxRS.Debug.on_breakpoint(line, locals)
end

-- Capture the current call stack
function RobloxRS.Debug.capture_call_stack()
    local stack = {}
    local level = 2 -- Start at 2 to skip this function
    
    while true do
        local info = debug.getinfo(level, "Sln")
        if not info then break end
        
        -- Get local variables
        local locals = {}
        local i = 1
        while true do
            local name, value = debug.getlocal(level, i)
            if not name then break end
            locals[name] = value
            i = i + 1
        end
        
        table.insert(stack, {
            func = info.name or "?",
            line = info.currentline,
            source = info.source,
            locals = locals
        })
        
        level = level + 1
    end
    
    RobloxRS.Debug.call_stack = stack
end

-- Get current debug state
function RobloxRS.Debug.get_state()
    return {
        breakpoints = RobloxRS.Debug.breakpoints,
        watches = RobloxRS.Debug.watches,
        locals = RobloxRS.Debug.locals,
        call_stack = RobloxRS.Debug.call_stack
    }
end

-- Toggle a breakpoint
function RobloxRS.Debug.toggle_breakpoint(line)
    if RobloxRS.Debug.breakpoints[line] then
        RobloxRS.Debug.breakpoints[line].enabled = not RobloxRS.Debug.breakpoints[line].enabled
        return RobloxRS.Debug.breakpoints[line].enabled
    end
    return false
end

-- Clear all breakpoints
function RobloxRS.Debug.clear_breakpoints()
    RobloxRS.Debug.breakpoints = {}
end

-- Add a watch to a breakpoint
function RobloxRS.Debug.add_watch_to_breakpoint(line, expression)
    if RobloxRS.Debug.breakpoints[line] then
        table.insert(RobloxRS.Debug.breakpoints[line].watches, expression)
        return true
    end
    return false
end

-- Default breakpoint handler (can be overridden)
function RobloxRS.Debug.on_breakpoint(line, locals)
    print(string.format("Breakpoint hit at line %d", line))
    print("Local variables:")
    for name, value in pairs(locals) do
        print(string.format("  %s = %s", name, tostring(value)))
    end
    print("Watches:")
    for expr, watch in pairs(RobloxRS.Debug.watches) do
        print(string.format("  %s = %s", expr, tostring(watch.last_value)))
    end
end

return RobloxRS.Debug
