# Code_Logger

A simple, colorful, and flexible logging library for Rust with support for timestamps, log levels, custom error codes, and customizable ANSI colors. Built using a builder pattern for ergonomic usage.

![Crates.io](https://img.shields.io/crates/v/Code_Logger) ![Rust](https://img.shields.io/badge/rust-1.70+-orange)

---

## Features

- Four log levels: `Info`, `Warn`, `Error`, `Debug`
- Optional timestamps with custom format via `timestamp_format("%H:%M:%S")`
- Optional custom error codes appended as `(code 123)`
- Color-coded output in the terminal with:
  - Global disable via `.no_color()`
  - Full custom color map via `.colors(AnsiColors { .. })`
  - Per-level override via `.color_for_level(Level::Error, "\x1b[97;41m")`
- Smart output stream selection: `Warn` and `Error` to `stderr`, others to `stdout`
- Ergonomic builder pattern

---

## Installation

Add `code_logger` to your `Cargo.toml`:

```toml
[dependencies]
code_logger = "0.1"
```

---

## Quick start

```rust
use code_logger::log; // if used as a library crate

fn main() {
    // Basic with timestamp and code
    log("Hello, world!".to_string())
        .timestamp()
        .code(42)
        .info()
        .print();
}
```

---

## Advanced usage

```rust
use code_logger::logger::{log, AnsiColors, Level};

fn main() {
    // Custom timestamp format
    log("Custom time format".to_string())
        .timestamp_format("%H:%M:%S")
        .debug()
        .print();

    // Disable colors entirely (useful for logs to file)
    log("No colors here".to_string())
        .no_color()
        .warn()
        .print();

    // Provide custom colors
    let mut custom = AnsiColors::default();
    custom.info = "\x1b[35m".to_string(); // magenta
    custom.warn = "\x1b[36m".to_string(); // cyan

    log("Custom color config".to_string())
        .colors(custom)
        .info()
        .print();

    // Override a single level color quickly
    log("Only ERROR is white on red".to_string())
        .color_for_level(Level::Error, "\x1b[97;41m")
        .error()
        .print();
}
```

---

## API overview

- `log(message: String) -> LoggerBuilder`
- `LoggerBuilder` methods:
  - `.code(i32)`
  - `.timestamp()`
  - `.timestamp_format(fmt: &str)`
  - `.no_color()`
  - `.colors(AnsiColors)`
  - `.color_for_level(Level, &str)`
  - `.info()/.warn()/.error()/.debug()` → returns `Logger`
- `Logger::print()` to emit the log immediately

---

## Notes

- ANSI colors are commonly supported by terminals. Use `.no_color()` when redirecting output to files.
- Default timestamp format is `%Y-%m-%d %H:%M:%S`.
- `Warn`/`Error` go to `stderr`, which helps when piping or filtering output.
