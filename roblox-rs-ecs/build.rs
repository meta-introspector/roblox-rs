use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Check if we're building for Roblox
    let is_roblox_target = env::var("CARGO_FEATURE_ROBLOX").is_ok();
    
    if is_roblox_target {
        println!("cargo:warning=Building for Roblox target");
        
        // Set up output directories
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let luau_out_dir = out_dir.join("luau");
        fs::create_dir_all(&luau_out_dir).unwrap();
        
        // Process our own code
        transpile_crate_to_luau();
        
        // Process dependencies
        transpile_dependencies_to_luau(&luau_out_dir);
    }
    
    // Tell Cargo to re-run this script if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
}

/// Transpile the current crate's Rust code to Luau
fn transpile_crate_to_luau() {
    // This would be much more complex in reality, likely calling into 
    // roblox-rs-core's compiler functionality
    println!("cargo:warning=Transpiling crate code to Luau");
    
    // Scan source files
    let src_dir = PathBuf::from("src");
    if src_dir.exists() {
        // In a real implementation, we would:
        // 1. Parse Rust files with syn
        // 2. Transform to an intermediate AST
        // 3. Generate Luau code
        // 4. Write to output directory
    }
}

/// Transpile dependencies to Luau
fn transpile_dependencies_to_luau(output_dir: &Path) {
    // List of dependencies that need to be transpiled
    let supported_deps = [
        "hecs",
        "slotmap",
        "smallvec",
        "bitflags",
    ];
    
    println!("cargo:warning=Transpiling dependencies to Luau");
    
    for dep in supported_deps {
        transpile_dependency(dep, output_dir);
    }
}

/// Transpile a specific dependency to Luau
fn transpile_dependency(dep_name: &str, output_dir: &Path) {
    println!("cargo:warning=Transpiling dependency: {}", dep_name);
    
    // Create a directory for this dependency
    let dep_dir = output_dir.join(dep_name);
    fs::create_dir_all(&dep_dir).unwrap();
    
    // In a real implementation:
    // 1. Find the dependency's source code (cargo metadata can help)
    // 2. Parse its Rust files
    // 3. Transform to Luau code
    // 4. Write to the dependency directory
    
    // For now, we'll just write a placeholder implementation
    let placeholder_code = match dep_name {
        "hecs" => generate_hecs_placeholder(),
        "slotmap" => generate_slotmap_placeholder(),
        "smallvec" => generate_smallvec_placeholder(),
        "bitflags" => generate_bitflags_placeholder(),
        _ => String::new(),
    };
    
    if !placeholder_code.is_empty() {
        let main_file = dep_dir.join("init.lua");
        fs::write(main_file, placeholder_code).unwrap();
    }
}

/// Generate a minimal working Luau implementation of hecs
fn generate_hecs_placeholder() -> String {
    r#"-- Minimal hecs implementation for Roblox
local hecs = {}

-- Entity implementation
hecs.Entity = {}
hecs.Entity.__index = hecs.Entity

function hecs.Entity.new(id)
    return setmetatable({id = id or 0}, hecs.Entity)
end

function hecs.Entity:id()
    return self.id
end

-- World implementation
hecs.World = {}
hecs.World.__index = hecs.World

function hecs.World.new()
    return setmetatable({
        entities = {},
        components = {},
        next_entity_id = 1,
    }, hecs.World)
end

function hecs.World:spawn(components)
    local id = self.next_entity_id
    self.next_entity_id = self.next_entity_id + 1
    
    local entity = hecs.Entity.new(id)
    self.entities[id] = entity
    self.components[id] = components or {}
    
    return entity
end

function hecs.World:despawn(entity)
    if type(entity) == "table" then
        entity = entity.id
    end
    
    self.entities[entity] = nil
    self.components[entity] = nil
end

function hecs.World:get(entity, component_type)
    if type(entity) == "table" then
        entity = entity.id
    end
    
    local entity_components = self.components[entity]
    if not entity_components then
        return nil
    end
    
    return entity_components[component_type]
end

function hecs.World:query(component_types)
    local results = {}
    
    for id, entity in pairs(self.entities) do
        local components = self.components[id]
        local match = true
        
        for _, component_type in ipairs(component_types) do
            if not components[component_type] then
                match = false
                break
            end
        end
        
        if match then
            local result = {entity}
            for _, component_type in ipairs(component_types) do
                table.insert(result, components[component_type])
            end
            table.insert(results, result)
        end
    end
    
    return results
end

return hecs
"#.to_string()
}

/// Generate a minimal working Luau implementation of slotmap
fn generate_slotmap_placeholder() -> String {
    r#"-- Minimal slotmap implementation for Roblox
local slotmap = {}

-- SlotMap implementation
slotmap.SlotMap = {}
slotmap.SlotMap.__index = slotmap.SlotMap

function slotmap.SlotMap.new()
    return setmetatable({
        slots = {},
        free_list = nil,
        next_key = 1,
    }, slotmap.SlotMap)
end

function slotmap.SlotMap:insert(value)
    local key = self.next_key
    self.next_key = self.next_key + 1
    
    self.slots[key] = value
    return key
end

function slotmap.SlotMap:get(key)
    return self.slots[key]
end

function slotmap.SlotMap:remove(key)
    local value = self.slots[key]
    self.slots[key] = nil
    return value
end

function slotmap.SlotMap:contains(key)
    return self.slots[key] ~= nil
end

return slotmap
"#.to_string()
}

/// Generate a minimal working Luau implementation of smallvec
fn generate_smallvec_placeholder() -> String {
    r#"-- Minimal smallvec implementation for Roblox
-- For Luau, we can just use tables directly since they're efficient for small collections
local smallvec = {}

function smallvec.new(initial_capacity)
    return {
        inline_capacity = initial_capacity or 4,
        length = 0,
        -- In Luau, tables are already heap-allocated and efficient for both small and large collections
    }
end

function smallvec.with_capacity(capacity)
    return smallvec.new(capacity)
end

function smallvec.from_slice(slice)
    local vec = smallvec.new(#slice)
    for i, v in ipairs(slice) do
        vec[i] = v
    end
    vec.length = #slice
    return vec
end

-- For Luau, we don't need special implementations of push, pop, etc.
-- since tables already handle these operations efficiently

return smallvec
"#.to_string()
}

/// Generate a minimal working Luau implementation of bitflags
fn generate_bitflags_placeholder() -> String {
    r#"-- Minimal bitflags implementation for Roblox
local bitflags = {}

-- Create a bitflags module
function bitflags.create(name, flags)
    local module = {
        flags = flags,
        _name = name,
    }
    
    -- Create constants for each flag
    for flag_name, value in pairs(flags) do
        module[flag_name] = value
    end
    
    -- Create methods
    function module.empty()
        return 0
    end
    
    function module.all()
        local result = 0
        for _, value in pairs(flags) do
            result = bit32.bor(result, value)
        end
        return result
    end
    
    function module.contains(flags, other)
        return bit32.band(flags, other) == other
    end
    
    function module.intersects(flags, other)
        return bit32.band(flags, other) ~= 0
    end
    
    return module
end

return bitflags
"#.to_string()
} 