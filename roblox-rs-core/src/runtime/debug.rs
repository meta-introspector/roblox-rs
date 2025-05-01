// Debug module for Roblox-RS runtime
// Provides debugging utilities for runtime code

// Debug tracing function
pub fn generate_debug_tracer() -> String {
    r#"
-- Debug tracing module
RobloxRS.Debug = {}

-- Enable/disable debug tracing
RobloxRS.Debug.enabled = false

-- Trace a function call
function RobloxRS.Debug.traceCall(funcName, args)
    if RobloxRS.Debug.enabled then
        local argsString = ""
        for i, arg in ipairs(args) do
            if type(arg) == "string" then
                argsString = argsString .. "\"" .. arg .. "\""
            else
                argsString = argsString .. tostring(arg)
            end
            
            if i < #args then
                argsString = argsString .. ", "
            end
        end
        
        print("TRACE: " .. funcName .. "(" .. argsString .. ")")
    end
    
    return args
end

-- Get stack trace
function RobloxRS.Debug.getStackTrace()
    local level = 1
    local stackTrace = {}
    
    while true do
        local info = debug.info(level, "sl")
        if not info then break end
        
        table.insert(stackTrace, {
            source = info.source,
            line = info.currentline
        })
        
        level = level + 1
    end
    
    return stackTrace
end
"#.to_string()
}
