// Rust facade for Roblox Instance API
// This module provides Rust wrappers that transpile to Luau Instance operations

/// Generates Luau code for Instance interaction library
pub fn generate_instance_lib() -> String {
    r#"-- RobloxRS Instance Library
RobloxRS.Instance = {}

-- Create a new Instance
function RobloxRS.Instance.new(className, parent)
    local instance = Instance.new(className)
    if parent then
        instance.Parent = parent
    end
    return instance
end

-- Find first child by name
function RobloxRS.Instance.findFirstChild(parent, name)
    return parent:FindFirstChild(name)
end

-- Find first child which is a descendant
function RobloxRS.Instance.findFirstDescendant(parent, name)
    return parent:FindFirstDescendant(name)
end

-- Get children as an array
function RobloxRS.Instance.getChildren(parent)
    local children = parent:GetChildren()
    local result = {}
    for i = 1, #children do
        result[i] = children[i]
    end
    return result
end

-- Get descendants as an array
function RobloxRS.Instance.getDescendants(parent)
    local descendants = parent:GetDescendants()
    local result = {}
    for i = 1, #descendants do
        result[i] = descendants[i]
    end
    return result
end

-- Set property
function RobloxRS.Instance.setProperty(instance, property, value)
    instance[property] = value
    return instance
end

-- Get property
function RobloxRS.Instance.getProperty(instance, property)
    return instance[property]
end

-- Connect to event
function RobloxRS.Instance.connect(instance, eventName, callback)
    local connection = instance[eventName]:Connect(callback)
    return {
        disconnect = function() 
            connection:Disconnect()
        end
    }
end

-- Wait for child
function RobloxRS.Instance.waitForChild(parent, name, timeout)
    if timeout then
        return parent:WaitForChild(name, timeout)
    else
        return parent:WaitForChild(name)
    end
end

-- Destroy instance
function RobloxRS.Instance.destroy(instance)
    instance:Destroy()
end

-- Clone instance
function RobloxRS.Instance.clone(instance)
    return instance:Clone()
end

-- Check if instance is A of class
function RobloxRS.Instance.isA(instance, className)
    return instance:IsA(className)
end

-- Get service
function RobloxRS.Instance.getService(serviceName)
    return game:GetService(serviceName)
end

-- Create a folder for organization
function RobloxRS.Instance.createFolder(name, parent)
    local folder = Instance.new("Folder")
    folder.Name = name
    if parent then
        folder.Parent = parent
    end
    return folder
end

-- Set attributes (Roblox attributes system)
function RobloxRS.Instance.setAttribute(instance, attributeName, value)
    instance:SetAttribute(attributeName, value)
    return instance
end

-- Get attribute
function RobloxRS.Instance.getAttribute(instance, attributeName)
    return instance:GetAttribute(attributeName)
end

-- Helper to create hierarchy of instances
function RobloxRS.Instance.hierarchy(structure, parent)
    local root = nil
    
    for i, item in ipairs(structure) do
        local instance
        
        if type(item) == "string" then
            -- Just a className with default name
            instance = Instance.new(item)
        elseif type(item) == "table" then
            if item.className then
                -- Create the instance
                instance = Instance.new(item.className)
                
                -- Set name if specified
                if item.name then
                    instance.Name = item.name
                end
                
                -- Set properties if specified
                if item.properties then
                    for prop, value in pairs(item.properties) do
                        instance[prop] = value
                    end
                end
                
                -- Set attributes if specified
                if item.attributes then
                    for attr, value in pairs(item.attributes) do
                        instance:SetAttribute(attr, value)
                    end
                end
                
                -- Handle children recursively
                if item.children then
                    RobloxRS.Instance.hierarchy(item.children, instance)
                end
            end
        end
        
        if instance then
            -- Set parent
            if parent then
                instance.Parent = parent
            end
            
            -- Keep track of the root instance
            if i == 1 and not root then
                root = instance
            end
        end
    end
    
    return root
end
"#.to_string()
}
