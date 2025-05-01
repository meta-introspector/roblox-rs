# Dependency Compatibility for Roblox

This document outlines the compatibility status of roblox-rs-ecs dependencies when compiling to Luau for the Roblox platform.

## Core Dependencies

| Dependency     | Status          | Strategy                                  | Notes                                      |
|----------------|-----------------|-------------------------------------------|-------------------------------------------|
| hecs           | ✅ Compatible   | Custom Luau implementation               | Core ECS functionality with simplified API |
| slotmap        | ✅ Compatible   | Custom Luau implementation               | Uses Luau tables with numeric keys         |
| smallvec       | ✅ Compatible   | Luau tables                              | Luau tables are efficient for small collections |
| bitflags       | ✅ Compatible   | Custom Luau bit32 implementation         | Uses Luau's built-in bit32 library        |
| parking_lot    | ⚠️ Partial      | Simplified mutex for single-threaded env | No actual locking needed in Roblox        |
| rayon          | ❌ Incompatible | Excluded in Roblox builds               | Parallelism not available in Roblox        |

## Utility Dependencies

| Dependency     | Status          | Strategy                                  | Notes                                      |
|----------------|-----------------|-------------------------------------------|-------------------------------------------|
| thiserror      | ⚠️ Compile-time | Transpiled to basic Luau errors           | Used only at compile time                  |
| anyhow         | ⚠️ Compile-time | Transpiled to basic Luau errors           | Used only at compile time                  |
| log            | ✅ Compatible   | Maps to Roblox print/warn/error          | Simplified logging API                     |
| env_logger     | ❌ Incompatible | Excluded in Roblox builds               | Not applicable to Roblox                   |

## Transpilation Strategy

For each dependency, we follow one of these strategies:

1. **Direct Translation**: The dependency's code is parsed and translated directly to equivalent Luau code.
2. **Simplified Reimplementation**: We create a simplified version of the dependency that provides the core functionality.
3. **Roblox-Native Alternative**: We use Roblox's built-in functionality instead of the dependency.
4. **Build-time Only**: The dependency is used only during build and not included in the compiled output.

## Custom Implementations

For key dependencies like `hecs`, we provide custom Luau implementations that maintain the same API but use Luau-idiomatic code. These implementations are located in the build output directory and are included in the final Roblox bundle.

## Adding New Dependencies

When adding new dependencies, follow these guidelines:

1. Add the dependency with `default-features = false` to minimize included code
2. Mark the dependency as `optional = true` if it's not needed in Roblox
3. Create a platform abstraction if the dependency provides platform-specific functionality
4. Add an entry to this document explaining the compatibility strategy
5. Create a Luau implementation for any core dependency that needs to be available in Roblox

## Testing Dependencies

Dependencies are tested for Roblox compatibility using:

1. **Unit tests** running in the Luau VM
2. **Integration tests** running in Roblox Studio
3. **API compatibility checks** to ensure the Luau implementation matches the Rust API

## Updating Dependencies

When updating dependencies, the Luau implementation must be reviewed and potentially updated to match any API changes. 