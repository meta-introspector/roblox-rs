---
title: Advanced State Management
---

# Advanced State Management

Learn advanced techniques for managing state in roblox-rs-gui, including signals, derived state, and best practices for complex UIs.

## Signals and Reactivity

Signals are reactive values that update the UI automatically:

```rust
let count = create_signal(0);

let button = Button::new()
    .text(move || format!("Count: {}", count.get()))
    .on_click(move || count.update(|c| c + 1));
```

## Derived State

Use `create_memo` to derive state from other signals:

```rust
let first = create_signal("Jane".to_string());
let last = create_signal("Doe".to_string());
let full = create_memo(move || format!("{} {}", first.get(), last.get()));

let label = TextLabel::new().text(move || full.get());
```

## Two-Way Data Binding

Bind input fields to signals for forms:

```rust
let username = create_signal(String::new());
let input = TextBox::new().text(move || username.get()).on_change(move |val| username.set(val));
```

## Best Practices
- Minimize the number of signals; use derived state for computed values.
- Clean up effects to avoid memory leaks.
- Use context or resources for global/shared state.

## Next Steps
- [Style GUI Components](./style-gui-components)
- [Integrate GUI with ECS](./integrate-gui-with-ecs) 