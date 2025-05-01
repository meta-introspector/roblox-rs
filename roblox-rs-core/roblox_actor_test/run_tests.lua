-- Entry point script for run-in-roblox
-- This will execute our tests and return the results

-- Run the actor tests
local success, errorMessage = pcall(function()
    -- Make sure we load tests from the correct location
    print("Starting RobloxRS Actor System Tests...")
    local ServerScriptService = game:GetService("ServerScriptService")
    require(ServerScriptService:WaitForChild("ActorTests"))
end)

if not success then
    print("Test execution failed: " .. tostring(errorMessage))
    os.exit(1)
end

-- Exit with success code
os.exit(0)
