// Debugger module for Roblox-RS runtime
// Provides advanced debugging tools for runtime code

// Generate the debugger module
pub fn generate_debugger() -> String {
    r#"
-- Advanced debugger for Roblox-RS
RobloxRS.Debugger = {}

-- Store breakpoints
RobloxRS.Debugger.breakpoints = {}

-- Add a breakpoint
function RobloxRS.Debugger.addBreakpoint(file, line, condition)
    local key = file .. ":" .. tostring(line)
    RobloxRS.Debugger.breakpoints[key] = {
        file = file,
        line = line,
        condition = condition,
        enabled = true
    }
end

-- Remove a breakpoint
function RobloxRS.Debugger.removeBreakpoint(file, line)
    local key = file .. ":" .. tostring(line)
    RobloxRS.Debugger.breakpoints[key] = nil
end

-- Enable/disable a breakpoint
function RobloxRS.Debugger.toggleBreakpoint(file, line, enabled)
    local key = file .. ":" .. tostring(line)
    local bp = RobloxRS.Debugger.breakpoints[key]
    
    if bp then
        bp.enabled = enabled ~= false
    end
end

-- Check if breakpoint is hit
function RobloxRS.Debugger.checkBreakpoint(file, line, context)
    local key = file .. ":" .. tostring(line)
    local bp = RobloxRS.Debugger.breakpoints[key]
    
    if bp and bp.enabled then
        if bp.condition then
            -- Evaluate condition in the current context
            local conditionFunc = loadstring("return " .. bp.condition)
            setfenv(conditionFunc, context)
            local success, result = pcall(conditionFunc)
            
            if success and result then
                return true
            end
        else
            return true
        end
    end
    
    return false
end

-- Variable inspector
function RobloxRS.Debugger.inspectVariable(var, depth)
    depth = depth or 0
    local maxDepth = 3
    local indent = string.rep("  ", depth)
    
    if depth > maxDepth then
        return indent .. "..."
    end
    
    if type(var) == "table" then
        local result = indent .. "{\n"
        
        for k, v in pairs(var) do
            result = result .. indent .. "  [" .. tostring(k) .. "] = "
            
            if type(v) == "table" then
                result = result .. "\n" .. RobloxRS.Debugger.inspectVariable(v, depth + 1)
            else
                result = result .. tostring(v)
            end
            
            result = result .. ",\n"
        end
        
        result = result .. indent .. "}"
        return result
    else
        return indent .. tostring(var)
    end
end

-- Performance monitoring
RobloxRS.Debugger.performanceStats = {
    functionCalls = {},
    startTime = os.clock()
}

-- Track function performance
function RobloxRS.Debugger.trackCall(funcName)
    local stats = RobloxRS.Debugger.performanceStats.functionCalls
    
    if not stats[funcName] then
        stats[funcName] = {
            calls = 0,
            totalTime = 0,
            maxTime = 0
        }
    end
    
    local startTime = os.clock()
    
    return function()
        local endTime = os.clock()
        local duration = endTime - startTime
        
        stats[funcName].calls = stats[funcName].calls + 1
        stats[funcName].totalTime = stats[funcName].totalTime + duration
        stats[funcName].maxTime = math.max(stats[funcName].maxTime, duration)
    end
end

-- Get performance report
function RobloxRS.Debugger.getPerformanceReport()
    local stats = RobloxRS.Debugger.performanceStats.functionCalls
    local report = {}
    
    for funcName, data in pairs(stats) do
        table.insert(report, {
            name = funcName,
            calls = data.calls,
            totalTime = data.totalTime,
            averageTime = data.totalTime / math.max(1, data.calls),
            maxTime = data.maxTime
        })
    end
    
    -- Sort by total time (most expensive first)
    table.sort(report, function(a, b)
        return a.totalTime > b.totalTime
    end)
    
    return report
end

-- Reset performance stats
function RobloxRS.Debugger.resetPerformanceStats()
    RobloxRS.Debugger.performanceStats = {
        functionCalls = {},
        startTime = os.clock()
    }
end
"#.to_string()
}
