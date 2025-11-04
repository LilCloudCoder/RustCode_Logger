mod logger;
use logger::{log, AnsiColors, Level};

fn main() {
    // Basic with timestamp and code
    log("Hello, world!".to_string())
        .timestamp()
        .code(42)
        .info()
        .print();

    // Error goes to stderr
    log("Something went wrong".to_string())
        .code(404)
        .error()
        .print();

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

    logger.print(); // prints it
}