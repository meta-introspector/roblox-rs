---
title: Welcome to roblox-rs
slug: /
---

# roblox-rs Documentation

**Modern Rust Development for Roblox Games**

Welcome to the documentation for **roblox-rs**: A powerful toolkit that brings Rust's type safety, performance, and rich ecosystem to Roblox game development!

This documentation is organized using the [Diátaxis framework](https://diataxis.fr/):

- **Tutorials**: Step-by-step lessons to get started with roblox-rs, perfect for beginners who want to learn by doing.
- **How-to Guides**: Practical guides to solve specific problems and achieve particular goals with roblox-rs.
- **Reference**: Technical reference for APIs and crates, focused on accurate, complete information about all roblox-rs components.
- **Explanation**: Background knowledge, concepts, and design philosophy behind roblox-rs and its architecture.

## What is roblox-rs?

roblox-rs is a Rust-based toolkit for Roblox game development, featuring:
- A Rust-to-Luau compiler with strong type checking
- A powerful Entity Component System (ECS) framework
- A Svelte-inspired reactive GUI system
- CLI tools for project scaffolding and compilation

Each project is a separate crate, but they work together to enable modern, type-safe Roblox development in Rust.

## Monorepo Structure

- `roblox-rs-core`: Core logic for parsing Rust AST, transforming to Luau AST, and generating/optimizing Luau code.
- `roblox-rs-ecs`: Bevy-inspired ECS framework for Roblox and native.
- `roblox-rs-cli`: CLI for compiling Rust to Luau and project scaffolding.
- `roblox-rs-gui`: Svelte-inspired reactive GUI framework for Roblox.
- `roblox-rs-rojo`: Planned for Rojo integration.

---

Ready to get started? Use the sidebar to explore tutorials, guides, references, and explanations for roblox-rs.
