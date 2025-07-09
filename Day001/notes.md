# 🦀 Day 1 – Intro to Rust

## ✅ Why Rust?
- High-level language features **without performance penalties**
- Compile-time guarantees (ownership, borrowing)
- First-class multithreading support
- Growing ecosystem and awesome tooling (`cargo`)
- Friendly developer community

## 🔧 Rust Concepts Learned
- `main()` function → program entry point
- Variables (`let`, `mut`)
- Constants (`const`)
- Shadowing (`let` with same name again)
- File naming: snake_case with `.rs` extension

## 📦 Cargo Basics
- `cargo build` → compile project (debug)
- `cargo run` → compile + run
- `cargo check` → check for errors only
- `cargo build --release` → optimized build

## 🧠 Notes
- Variables are immutable by default  
- Use `mut` to make them mutable  
- Shadowing lets you redeclare with `let`  
- Constants require type annotation and `const` keyword  
- File names must match module names in snake_case

## 📁 Example Files
```rust
// main.rs
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    let mut y = 10;
    let y = y + 1; // shadowing
    println!("x: {}, y: {}", x, y);
}
