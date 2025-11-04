mod logger;
use logger::{log, AnsiColors, Level};

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