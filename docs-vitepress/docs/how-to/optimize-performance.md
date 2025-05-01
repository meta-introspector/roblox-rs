---
title: Optimize Performance
---

# Optimize Performance

Tips and techniques for optimizing the performance of your roblox-rs projects.

## Compiler Optimizations

- Use `--optimize aggressive` for maximum Luau code optimization.
- Enable `--release` mode in Rust for best native performance (if targeting native).
- Minimize debug info in production builds.

## Code-Level Tips

- Prefer simple data structures and avoid deep nesting.
- Use signals and effects efficiently; avoid unnecessary recomputation.
- Minimize the number of UI updates by batching changes.
- Use ECS queries efficiently; filter only what you need.

## Profiling and Debugging

- Use Roblox's built-in profiler to identify bottlenecks.
- Add logging in Rust and Luau to trace slow code paths.
- Profile both the Rust and Luau sides if using native and Roblox targets.

## Example: Optimized Signal Usage

```rust
let expensive = create_signal(0);
let derived = create_memo(move || {
    // Only recompute when `expensive` changes
    expensive.get() * 2
});
```

## Next Steps
- [Advanced Compiler Options](./advanced-compiler-options)
- [Debugging Compiled Code](./debugging-compiled-code) 