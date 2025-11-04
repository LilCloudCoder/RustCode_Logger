# Code_Logger

A simple, colorful, and flexible logging library for Rust with support for timestamps, log levels, and custom error codes. Built using a **builder pattern** for ergonomic usage.

![Crates.io](https://img.shields.io/crates/v/Code_Logger) ![Rust](https://img.shields.io/badge/rust-1.70+-orange)

---

## Features

- Four log levels: `Info`, `Warn`, `Error`, `Debug`
- Optional timestamps
- Optional custom error codes
- Color-coded output in the terminal
- Ergonomic builder pattern

---

## Installation

Add `code_logger` to your `Cargo.toml`:

```toml
[dependencies]
code_logger = "..."
```

---

## Quick start

```rust
use code_logger::log; // if used as a library crate

fn main() {
    // Using builder
    log("Hello, world!".to_string())
        .timestamp()
        .code(42)
        .info()   // builds the Logger
        .print(); // actually prints it

    // Or directly
    let logger = log("Something went wrong".to_string())
        .code(404)
        .error(); // builds Logger

    logger.print(); // prints it
}
```
