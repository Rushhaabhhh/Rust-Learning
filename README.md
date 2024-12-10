# Rust Learning Guide

This guide will help you get started with Rust, from setting up your environment to building intermediate-level projects. Whether you're a complete beginner or have some programming experience, this guide will walk you through the essential steps.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Setting Up Your Rust Environment](#setting-up-your-rust-environment)
3. [Hello, World!](#hello-world)
4. [Understanding Rust Basics](#understanding-rust-basics)
5. [Building Your First Project](#building-your-first-project)
6. [Intermediate Topics](#intermediate-topics)
7. [Resources for Further Learning](#resources-for-further-learning)

---

## Getting Started

Rust is a systems programming language focusing on safety, speed, and concurrency. It has a strong type system and guarantees memory safety without needing a garbage collector.

### Why Learn Rust?
- High performance.
- Memory safety without a garbage collector.
- Modern tooling.
- Ideal for web assembly, embedded systems, and large-scale projects.

---

## Setting Up Your Rust Environment

### Install Rust

1. Install Rust using [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Verify installation:
   ```bash
   rustc --version
   ```

### Update Rust
Keep your Rust installation up to date:
```bash
rustup update
```

---

## Hello, World!

Create your first Rust program:

1. Create a new file `main.rs`:
   ```rust
   fn main() {
       println!("Hello, world!");
   }
   ```

2. Compile and run the program:
   ```bash
   rustc main.rs
   ./main
   ```

---

## Understanding Rust Basics

### Variables
Rust variables are immutable by default:
```rust
let x = 5; // Immutable
let mut y = 10; // Mutable
```

### Data Types
```rust
let int: i32 = 10;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = 'A';
```

### Control Flow
```rust
if x > 5 {
    println!("x is greater than 5");
} else {
    println!("x is 5 or less");
}

for i in 0..5 {
    println!("Iteration: {}", i);
}
```

### Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("The sum is {}", result);
}
```

---

## Building Your First Project

1. Initialize a new project using Cargo:
   ```bash
   cargo new my_project
   cd my_project
   ```

2. Explore the project structure:
   - `src/main.rs`: Entry point of your application.
   - `Cargo.toml`: Contains project metadata and dependencies.

3. Run your project:
   ```bash
   cargo run
   ```

4. Build your project:
   ```bash
   cargo build
   ```

---

## Intermediate Topics

### Ownership and Borrowing
Rust’s ownership model ensures memory safety:
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

### Error Handling
- Using `Result` and `Option`:
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

### Structs and Enums
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

### Modules and Crates
Organize your code using modules and import external crates:
```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = math::add(5, 3);
    println!("Result: {}", result);
}
```

### Working with Iterators and Closures
```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

---

## Resources for Further Learning

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)

---

Happy coding in Rust!
