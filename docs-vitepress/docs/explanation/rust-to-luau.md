---
title: How Rust Code Becomes Luau
sidebar_label: Rust to Luau
---

# How Rust Code Becomes Luau

This document explains the process by which roblox-rs transforms Rust code into Luau code for Roblox.

## The Fundamental Challenge

Rust and Luau are fundamentally different languages:

- **Rust** is statically typed, compiled, memory-safe without garbage collection, and has rich compile-time features.
- **Luau** is dynamically typed, interpreted, garbage-collected, and designed for scripting.

Converting between them requires careful mapping of concepts and features.

## The Transformation Process

### 1. Parsing Rust Code

The first step is to parse Rust source code into an Abstract Syntax Tree (AST) that can be analyzed and transformed. roblox-rs-core uses the `syn` crate to parse Rust code:

```rust
use syn::{parse_file, File};

fn parse_rust(source: &str) -> Result<File, Error> {
    parse_file(source)
}
```

This gives us a structured representation of the Rust code, including all functions, types, expressions, and statements.

### 2. Type Analysis

Rust's type system is much more complex than Luau's. The compiler performs type analysis to:

- Infer types where not explicitly stated
- Resolve generic types
- Check trait bounds
- Build a type map for the entire program

This information is crucial for correctly translating Rust constructs to Luau.

### 3. Handling Rust-Specific Features

Many Rust features have no direct equivalent in Luau. Here's how some key features are handled:

#### Ownership and Borrowing

Rust's ownership and borrowing system doesn't exist in Luau. The compiler:

- Transforms borrowed references (`&T`, `&mut T`) into regular values
- Converts ownership semantics into reference-based semantics
- Implements `Clone` and `Drop` traits as needed

For example, this Rust code:

```rust
fn process(value: &mut String) {
    value.push_str(" processed");
}

fn main() {
    let mut s = String::from("Hello");
    process(&mut s);
    println!("{}", s);
}
```

Becomes something like this in Luau:

```lua
local function process(value)
    value = value .. " processed"
    return value
end

local function main()
    local s = "Hello"
    s = process(s)
    print(s)
end
```

#### Enums and Pattern Matching

Rust's enums and pattern matching are transformed into Luau tables with type tags:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn handle_result(result: Result<i32, String>) {
    match result {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }
}
```

Becomes:

```lua
local Result = {}

local function handle_result(result)
    if result.tag == "Ok" then
        print("Success: " .. tostring(result.value))
    elseif result.tag == "Err" then
        print("Error: " .. tostring(result.value))
    end
end
```

#### Traits and Generics

Rust's traits and generics are challenging to represent in Luau. The compiler:

- Monomorphizes generic code where possible (creates specialized versions for each type)
- Implements trait methods directly on types
- Uses dynamic dispatch (function tables) for trait objects

For example:

```rust
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("Integer: {}", self);
    }
}

impl Printable for String {
    fn print(&self) {
        println!("String: {}", self);
    }
}

fn print_value<T: Printable>(value: T) {
    value.print();
}
```

Might become:

```lua
local Printable_i32 = {}
function Printable_i32.print(self)
    print("Integer: " .. tostring(self))
end

local Printable_String = {}
function Printable_String.print(self)
    print("String: " .. tostring(self))
end

local function print_value_i32(value)
    Printable_i32.print(value)
end

local function print_value_String(value)
    Printable_String.print(value)
end
```

### 4. Optimizing the Intermediate Representation

Before generating final Luau code, the compiler applies various optimizations:

- **Dead code elimination**: Removing unused functions and variables
- **Constant folding**: Computing constant expressions at compile time
- **Inlining**: Replacing function calls with the function body
- **Loop unrolling**: Converting loops to straight-line code for performance

### 5. Generating Luau Code

Finally, the optimized intermediate representation is converted to Luau code. The generator:

- Creates proper Luau syntax for each construct
- Handles variable scoping appropriately
- Deals with Luau's statement-based nature versus Rust's expression-based design
- Adds necessary runtime support (for example, implementations of Rust standard library functions)

## Runtime Support

Some Rust features need runtime support in Luau. roblox-rs includes a runtime library that provides:

- **Standard library functions**: Implementations of key Rust std functions
- **Memory management**: Utilities for managing memory-like constructs (e.g., `Vec`)
- **Error handling**: Support for Rust's `Result` and `Option` types
- **Iterator utilities**: Functions to support Rust's iterator patterns

## Type Mappings

Here's how common Rust types map to Luau:

| Rust Type | Luau Representation |
|-----------|---------------------|
| Integers (`i32`, etc.) | Numbers |
| Floats (`f32`, etc.) | Numbers |
| `bool` | Boolean |
| `char` | String (length 1) |
| `String` | String |
| `&str` | String |
| Tuples | Tables with numeric indices |
| Structs | Tables with named fields |
| Enums | Tables with a tag field |
| Arrays | Tables with numeric indices |
| `Vec<T>` | Tables with methods |
| `Option<T>` | Table (`{tag="Some", value=x}` or `{tag="None"}`) |
| `Result<T, E>` | Table (`{tag="Ok", value=x}` or `{tag="Err", value=e}`) |
| Trait objects | Tables with methods |
| Closures | Functions with captured variables |

## Limitations

While roblox-rs aims to support as much of Rust as possible, some features are inherently difficult to translate to Luau:

- **Unsafe code**: Cannot be properly checked or translated
- **Advanced type system features**: Traits like `Sized`, `Send`, etc.
- **Lifetimes**: These are compile-time constructs with no runtime representation
- **Macros**: Procedural macros are particularly challenging
- **Low-level memory operations**: Bit manipulation, raw pointers, etc.
- **Async/await**: Requires significant runtime support

## Examples

### Example 1: Simple Function

Rust:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Luau:
```lua
local function add(a, b)
    return a + b
end
```

### Example 2: Struct and Method

Rust:
```rust
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
    
    fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

Luau:
```lua
local Point = {}
Point.__index = Point

function Point.new(x, y)
    local self = {x = x, y = y}
    return setmetatable(self, Point)
end

function Point:distance(other)
    local dx = self.x - other.x
    local dy = self.y - other.y
    return math.sqrt(dx * dx + dy * dy)
end
```

### Example 3: Pattern Matching

Rust:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
    }
}
```

Luau:
```lua
local Message = {}

function Message.Quit()
    return {tag = "Quit"}
end

function Message.Move(x, y)
    return {tag = "Move", x = x, y = y}
end

function Message.Write(text)
    return {tag = "Write", value = text}
end

function Message.ChangeColor(r, g, b)
    return {tag = "ChangeColor", r = r, g = g, b = b}
end

local function process_message(msg)
    if msg.tag == "Quit" then
        print("Quit")
    elseif msg.tag == "Move" then
        print("Move to " .. tostring(msg.x) .. ", " .. tostring(msg.y))
    elseif msg.tag == "Write" then
        print("Text: " .. tostring(msg.value))
    elseif msg.tag == "ChangeColor" then
        print("Color: " .. tostring(msg.r) .. ", " .. tostring(msg.g) .. ", " .. tostring(msg.b))
    end
end
```

## Conclusion

The translation of Rust to Luau is a complex process that involves mapping between two very different programming paradigms. roblox-rs makes this possible through careful analysis, transformation, and runtime support, allowing Rust developers to leverage their skills and the Rust ecosystem when developing for the Roblox platform. 