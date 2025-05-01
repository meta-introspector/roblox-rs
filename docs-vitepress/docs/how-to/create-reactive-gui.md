---
title: How to Create Reactive GUIs
sidebar_label: Create Reactive GUIs
---

# How to Create Reactive GUIs

This guide shows you how to create reactive GUI elements using roblox-rs-gui.

## Prerequisites

- A roblox-rs project (see [Getting Started](../tutorials/getting-started.md))
- Basic understanding of Rust and Roblox UI

## Adding roblox-rs-gui to Your Project

First, add the roblox-rs-gui crate to your project's `Cargo.toml`:

```toml
[dependencies]
roblox-rs-gui = "0.1.0"
```

## Creating a Simple Counter Button

Let's create a simple counter button that increases its count when clicked.

### Step 1: Import the Required Components

```rust
use roblox_rs_gui::prelude::*;
```

### Step 2: Create a Signal for the Counter State

Signals are reactive values that automatically update the UI when changed:

```rust
// Create a signal with an initial value of 0
let count = create_signal(0);
```

### Step 3: Create the Button Component

```rust
// Create a button with reactive text
let button = Button::new()
    .text(move || format!("Count: {}", count.get()))
    .on_click(move || {
        // Update the count when clicked
        count.update(|c| c + 1);
    })
    .size(UDim2::new(0, 200, 0, 50))
    .position(UDim2::new(0.5, -100, 0.5, -25))
    .anchor_point(Vector2::new(0.5, 0.5));
```

### Step 4: Mount the Component to the Screen GUI

```rust
// Create a screen GUI and mount our button to it
let screen_gui = ScreenGui::new();
mount(screen_gui, button);
```

## Creating a Form with Two-Way Data Binding

Now let's create a form with two-way data binding:

### Step 1: Set Up Form State

```rust
// Create signals for our form fields
let username = create_signal(String::new());
let email = create_signal(String::new());
let age = create_signal(0);
```

### Step 2: Create Input Components

```rust
// Username input with two-way binding
let username_input = TextBox::new()
    .placeholder_text("Enter username")
    .text(username.get().clone())
    .on_text_changed(move |new_text| {
        username.set(new_text);
    })
    .size(UDim2::new(0, 200, 0, 30))
    .position(UDim2::new(0, 10, 0, 10));

// Email input with two-way binding
let email_input = TextBox::new()
    .placeholder_text("Enter email")
    .text(email.get().clone())
    .on_text_changed(move |new_text| {
        email.set(new_text);
    })
    .size(UDim2::new(0, 200, 0, 30))
    .position(UDim2::new(0, 10, 0, 50));

// Age input (numeric)
let age_input = TextBox::new()
    .placeholder_text("Enter age")
    .text(age.get().to_string())
    .on_text_changed(move |new_text| {
        if let Ok(new_age) = new_text.parse::<i32>() {
            age.set(new_age);
        }
    })
    .size(UDim2::new(0, 200, 0, 30))
    .position(UDim2::new(0, 10, 0, 90));
```

### Step 3: Create a Submit Button

```rust
// Submit button that validates and processes the form
let submit_button = Button::new()
    .text("Submit")
    .on_click(move || {
        if username.get().is_empty() {
            print!("Username is required");
            return;
        }
        
        if !email.get().contains('@') {
            print!("Invalid email");
            return;
        }
        
        if age.get() < 13 {
            print!("You must be at least 13 years old");
            return;
        }
        
        print!("Form submitted successfully!");
    })
    .size(UDim2::new(0, 200, 0, 30))
    .position(UDim2::new(0, 10, 0, 130));
```

### Step 4: Create a Frame and Mount All Components

```rust
// Create a frame to hold our form
let form_frame = Frame::new()
    .size(UDim2::new(0, 220, 0, 170))
    .position(UDim2::new(0.5, -110, 0.5, -85))
    .background_color3(Color3::new(0.9, 0.9, 0.9))
    .border_size_pixel(1);

// Add all components to the frame
mount(form_frame, vec![
    username_input,
    email_input, 
    age_input,
    submit_button,
]);

// Mount the frame to a screen GUI
let screen_gui = ScreenGui::new();
mount(screen_gui, form_frame);
```

## Creating a Dynamic List

Now let's create a dynamic list that can add and remove items:

### Step 1: Set Up List State

```rust
// Our list of items
let items = create_signal(vec!["Item 1".to_string(), "Item 2".to_string()]);

// Input for new items
let new_item = create_signal(String::new());
```

### Step 2: Create Input for Adding New Items

```rust
// Text input for new item
let item_input = TextBox::new()
    .placeholder_text("Enter new item")
    .text(new_item.get().clone())
    .on_text_changed(move |text| {
        new_item.set(text);
    })
    .size(UDim2::new(0, 150, 0, 30))
    .position(UDim2::new(0, 10, 0, 10));

// Add button
let add_button = Button::new()
    .text("Add")
    .on_click(move || {
        if !new_item.get().is_empty() {
            items.update(|mut list| {
                list.push(new_item.get().clone());
                new_item.set(String::new());
                list
            });
        }
    })
    .size(UDim2::new(0, 50, 0, 30))
    .position(UDim2::new(0, 170, 0, 10));
```

### Step 3: Create the Dynamic List

```rust
// Function to create a list item component
fn create_list_item(text: String, index: usize, items: Signal<Vec<String>>) -> Component {
    let frame = Frame::new()
        .size(UDim2::new(1, -20, 0, 30))
        .position(UDim2::new(0, 10, 0, (50 + index * 40) as f32))
        .background_color3(Color3::new(1, 1, 1))
        .border_size_pixel(1);
        
    let text_label = TextLabel::new()
        .text(text.clone())
        .size(UDim2::new(0.8, 0, 1, 0))
        .position(UDim2::new(0, 5, 0, 0))
        .text_color3(Color3::new(0, 0, 0))
        .background_transparency(1);
        
    let delete_button = Button::new()
        .text("X")
        .size(UDim2::new(0, 30, 0, 30))
        .position(UDim2::new(1, -35, 0, 0))
        .background_color3(Color3::new(0.9, 0.2, 0.2))
        .text_color3(Color3::new(1, 1, 1))
        .on_click(move || {
            items.update(|mut list| {
                if index < list.len() {
                    list.remove(index);
                }
                list
            });
        });
        
    mount(frame, vec![text_label, delete_button]);
    frame
}

// Create a scrolling frame to hold our list
let list_frame = ScrollingFrame::new()
    .size(UDim2::new(0, 250, 0, 300))
    .position(UDim2::new(0.5, -125, 0.5, -100))
    .background_color3(Color3::new(0.95, 0.95, 0.95))
    .can_vas_size(UDim2::new(0, 230, 0, 0))
    .automatic_canvas_size(Enum::Y);
    
// Mount input components
mount(list_frame, vec![item_input, add_button]);

// Create reactive binding for list items
create_effect(move || {
    let current_items = items.get();
    let item_components = current_items
        .iter()
        .enumerate()
        .map(|(i, item)| create_list_item(item.clone(), i, items.clone()))
        .collect::<Vec<_>>();
        
    // Update canvas size based on number of items
    list_frame.set_canvas_size(UDim2::new(
        0, 
        230, 
        0, 
        (50 + current_items.len() * 40) as f32
    ));
        
    // Clear and remount all items
    list_frame.clear_children_where(|child| {
        child.name() != "ItemInput" && child.name() != "AddButton"
    });
    
    for component in item_components {
        mount(list_frame, component);
    }
});

// Mount list to screen
let screen_gui = ScreenGui::new();
mount(screen_gui, list_frame);
```

## Working with Derived State

Derived state is state that's calculated based on other state values:

```rust
// Original signals
let first_name = create_signal("John".to_string());
let last_name = create_signal("Doe".to_string());

// Derived signal that updates when either first or last name changes
let full_name = create_memo(move || {
    format!("{} {}", first_name.get(), last_name.get())
});

// Use the derived signal in a UI component
let name_label = TextLabel::new()
    .text(move || full_name.get())
    .size(UDim2::new(0, 200, 0, 30))
    .position(UDim2::new(0.5, -100, 0.5, -15));
```

## Best Practices

1. **Keep Components Small**: Break down complex UIs into smaller, reusable components
2. **Minimize State**: Only use signals for values that need to trigger UI updates
3. **Use Derived State**: Use `create_memo` for values derived from other signals
4. **Clean Up Effects**: Ensure effects are cleaned up to prevent memory leaks

## Next Steps

- Learn how to [Style Your GUI Components](./style-gui-components.md)
- Explore [Advanced State Management](./advanced-state-management.md)
- See how to [Integrate GUIs with ECS](./integrate-gui-with-ecs.md) 