---
title: Style GUI Components
---

# Style GUI Components

Learn how to style your roblox-rs-gui components for a polished Roblox game UI.

## Basic Styling

You can set properties like size, position, color, and more directly on components:

```rust
let button = Button::new()
    .text("Click Me!")
    .size(UDim2::new(0, 200, 0, 50))
    .position(UDim2::new(0.5, -100, 0.5, -25))
    .background_color3(Color3::new(0.2, 0.6, 1.0))
    .text_color3(Color3::new(1, 1, 1));
```

## Responsive Layouts

Use anchors and layout containers to create responsive UIs:

```rust
let frame = Frame::new()
    .anchor_point(Vector2::new(0.5, 0.5))
    .size(UDim2::new(0.8, 0, 0.2, 0));
```

## Theming

Define color and style constants for consistency:

```rust
const PRIMARY_COLOR: Color3 = Color3::new(0.1, 0.4, 0.8);

let label = TextLabel::new()
    .background_color3(PRIMARY_COLOR)
    .text("Welcome!");
```

## Best Practices
- Use constants for colors and sizes.
- Group related components in frames.
- Use signals for dynamic style changes (e.g., highlight on hover).

## Next Steps
- [Create Reactive GUIs](./create-reactive-gui)
- [Advanced State Management](./advanced-state-management) 