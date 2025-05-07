# Rust Println Macro Guide 🦀

The `println!` macro is a fundamental debugging and output tool in Rust that prints formatted text to the console.

## Basic Usage

```rust
let life = 99;
println!("You have {} lives left", life);  // Basic interpolation
println!("{:?} {:?}", life, life);         // Debug format
println!("{life}");                        // Direct variable name