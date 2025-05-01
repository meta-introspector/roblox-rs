---
title: Getting Started with roblox-rs
sidebar_label: Getting Started
---

# Getting Started with roblox-rs

This tutorial will guide you through setting up a basic Roblox game project using roblox-rs. By the end, you'll have a working Rust project that compiles to Luau and runs on Roblox.

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/downloads)
- A code editor (such as [VS Code](https://code.visualstudio.com/))
- [Roblox Studio](https://www.roblox.com/create)

## Step 1: Install the roblox-rs CLI

The first step is to install the roblox-rs command-line interface:

```bash
cargo install roblox-rs-cli
```

Verify the installation by running:

```bash
roblox-rs --version
```

You should see the version number displayed in your terminal.

## Step 2: Create a New Project

Let's create a new project using the CLI:

```bash
roblox-rs new my-first-game
cd my-first-game
```

This command creates a new directory with a basic project structure:

```
my-first-game/
├── Cargo.toml          # Rust project configuration
├── src/
│   └── main.rs         # Main Rust code file
├── roblox-rs.toml      # roblox-rs configuration
└── README.md           # Project documentation
```

## Step 3: Explore the Default Code

Open `src/main.rs` in your editor. You'll see some basic Rust code:

```rust
fn main() {
    println!("Hello, Roblox!");
}
```

This is a simple Rust program that will be compiled to Luau code for Roblox.

## Step 4: Build Your Project

Let's compile the Rust code to Luau:

```bash
roblox-rs build
```

This command processes your Rust code and generates Luau files in the `out/` directory.

## Step 5: Import Into Roblox Studio

1. Open Roblox Studio
2. Create a new place or open an existing one
3. In the Explorer window, right-click on "ServerScriptService"
4. Select "Insert from File..."
5. Navigate to your project's `out/` directory and select the main Luau file

## Step 6: Run Your Game

Press the Play button in Roblox Studio to run your game. You should see "Hello, Roblox!" in the Output window.

## Next Steps

Congratulations! You've successfully created your first roblox-rs project. Here are some suggestions for what to explore next:

- Try the [Hello World ECS Tutorial](./hello-world-ecs.md) to learn how to use the Entity Component System
- Learn how to create a [Simple GUI](./simple-gui.md) using roblox-rs-gui

## Troubleshooting

- **Command not found**: Make sure Cargo's bin directory is in your PATH
- **Build errors**: Check that you have the latest version of roblox-rs-cli
- **Import issues**: Ensure the output directory contains the generated Luau files 