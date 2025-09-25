mod logger;
use logger::log;

fn main() {
    log("Server started".to_string()).info();  // prints immediately
    log("Disk space low".to_string()).warn();
    log("Failed to connect".to_string()).error();
}