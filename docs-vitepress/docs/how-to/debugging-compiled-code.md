---
title: Debugging Compiled Code
---

# Debugging Compiled Code

Learn how to debug issues in your Rust-to-Luau compiled code for Roblox.

## Enable Debug Info

- Use the `--debug` flag to include debug information in the generated Luau code.
- This adds comments and metadata to help trace errors.

## Common Issues

### Syntax Errors
- Check the generated Luau code for syntax issues.
- Ensure your Rust code uses supported features.

### Runtime Errors
- Use Roblox's output and error windows to view stack traces.
- Add print statements in both Rust and Luau for tracing.

### Mapping Errors Back to Rust
- Use debug comments in the Luau output to find the corresponding Rust source line.
- Keep your Rust code modular for easier tracing.

## Example: Debug Print

```rust
println!("Reached here!");
```

## Tips
- Use small, incremental changes to isolate bugs.
- Test with simple examples before scaling up.
- Use the `--debug` flag liberally during development.

## Next Steps
- [Optimize Performance](./optimize-performance)
- [Advanced Compiler Options](./advanced-compiler-options) 