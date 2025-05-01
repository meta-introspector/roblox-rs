---
title: roblox-rs-gui Reference
---

# roblox-rs-gui Reference

The `roblox-rs-gui` crate provides a reactive GUI framework for Roblox games, inspired by Svelte and modern web frameworks.

## Core Concepts

- **Component**: UI element (e.g., Button, Frame, TextLabel).
- **Signal**: Reactive value that updates the UI automatically.
- **Effect**: Runs code in response to signal changes.
- **Mount**: Attach components to the UI tree.

## Example: Reactive Button

```rust
let count = create_signal(0);
let button = Button::new()
    .text(move || format!("Count: {}", count.get()))
    .on_click(move || count.update(|c| c + 1));
```

## Two-Way Binding

```rust
let username = create_signal(String::new());
let input = TextBox::new().text(move || username.get()).on_change(move |val| username.set(val));
```

## Best Practices
- Use signals for all reactive state.
- Use `create_memo` for derived state.
- Clean up effects to avoid memory leaks.

See [Advanced State Management](../how-to/advanced-state-management) for more. 