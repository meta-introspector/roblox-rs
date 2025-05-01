# Roblox-RS Actor System

The Roblox-RS Actor System is now fully integrated into the runtime, providing robust actor-based concurrency for Rust-to-Luau transpiled code. This document outlines the implementation, features, and test coverage.

## Implementation

The actor system is implemented in pure Luau and integrated into the Roblox-RS runtime. The system provides:

- Actor creation and management
- Message passing with priorities
- Actor supervision for fault tolerance
- Actor pools for load balancing
- Shared state management
- Promise-like pattern for request-response

## Features

### Core Features

- **Actor Spawning**: Create actors with unique IDs and mailboxes
- **Message Passing**: Send messages with high/normal/low priorities
- **Ask Pattern**: Promise-like request-response with `await()` and `andThen()`
- **Actor Supervision**: Automatic restarts on failure with configurable policies
- **Actor Pools**: Load balancing across multiple actors with round-robin or least-busy strategies
- **Shared State**: Safe, actor-managed state with subscriptions
- **Actor Termination**: Clean shutdown of actors and resources

### Integration

The actor system is integrated into the Roblox-RS runtime through three components:

1. `RobloxRS.Actors` - The core actor system implementation
2. `RobloxRS.ActorTrait` - Rust-like trait interface for actors
3. `RobloxRS.Async` - Compatibility layer for async/await patterns

## Test Coverage

The implementation has been thoroughly tested in both standalone environments and in a simulated Roblox environment. Tests cover:

### Basic Actor Tests
- Actor creation with initial state
- Message sending and receiving
- Actor termination and cleanup

### Priority Message Tests
- Verifying high priority messages are processed first
- Message ordering based on priority (high/normal/low)

### Supervision Tests
- Actor restart on failure
- Maximum restart limits
- Recovery after restarts

### Actor Pool Tests
- Load distribution across multiple workers
- Parallel task processing
- Pool termination and cleanup

### Shared State Tests
- Accessing and modifying shared state
- Atomic updates to multiple values
- State synchronization across actors

### Promise Behavior Tests
- Promise-like behavior with `await()` and `andThen()`
- Timeout handling
- Error propagation

## Usage Example

```lua
-- Create an actor
local counter = RobloxRS.Actors.spawn(function(state, message)
    if message.type == "increment" then
        state.count = state.count + 1
        return state.count
    elseif message.type == "get" then
        return state.count
    end
end, {count = 0})

-- Send a message and wait for response
local result = counter:ask({type = "increment"}):await()
print("Counter: " .. result) -- Counter: 1

-- Use promise-like pattern
counter:ask({type = "increment"}):andThen(function(success, result)
    if success then
        print("New value: " .. result) -- New value: 2
    end
end)

-- Terminate when done
counter:terminate()
```

## Integration Status

The actor system is fully integrated into the Roblox-RS runtime and ready for use in Rust-to-Luau transpiled code. All tests pass, confirming the reliability and correctness of the implementation.
