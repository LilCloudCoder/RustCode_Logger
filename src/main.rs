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
        .error(); // builds Logger

    logger.print(); // prints it
}