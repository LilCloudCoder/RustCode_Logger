use chrono::Local;

/// Log levels
#[derive(Debug)]
pub enum Level {
    Info,
    Warn,
    Error,
    Debug,
}

/// Logger struct
pub struct Logger {
    pub message: String,
    pub level: Level,
    pub code: Option<i32>,
    pub timestamp: bool,
}

impl Logger {
    pub fn print(&self) {
        let prefix = match self.level {
            Level::Info => "\x1b[32m[INFO]\x1b[0m",
            Level::Warn => "\x1b[33m[WARN]\x1b[0m",
            Level::Error => "\x1b[31m[ERROR]\x1b[0m",
            Level::Debug => "\x1b[34m[DEBUG]\x1b[0m",
        };

        let ts = if self.timestamp {
            format!("[{}] ", Local::now().format("%Y-%m-%d %H:%M:%S"))
        } else {
            "".to_string()
        };

        let code_str = if let Some(c) = self.code {
            format!("(code {})", c)
        } else {
            "".to_string()
        };

        println!("{}{} {} {}", ts, prefix, self.message, code_str);
    }

    pub fn info(&self) { self.print(); }
    pub fn warn(&self) { self.print(); }
    pub fn error(&self) { self.print(); }
    pub fn debug(&self) { self.print(); }
}

/// Builder pattern
pub struct LoggerBuilder {
    pub message: String,
    pub code: Option<i32>,
    pub timestamp: bool,
}

impl LoggerBuilder {
    pub fn new(message: String) -> Self {
        Self { message, code: None, timestamp: false }
    }

    pub fn code(mut self, code: i32) -> Self {
        self.code = Some(code);
        self
    }

    pub fn timestamp(mut self) -> Self {
        self.timestamp = true;
        self
    }

    pub fn info(self) -> Logger {
        Logger { message: self.message, level: Level::Info, code: self.code, timestamp: self.timestamp }
    }

    pub fn warn(self) -> Logger {
        Logger { message: self.message, level: Level::Warn, code: self.code, timestamp: self.timestamp }
    }

    pub fn error(self) -> Logger {
        Logger { message: self.message, level: Level::Error, code: self.code, timestamp: self.timestamp }
    }

    pub fn debug(self) -> Logger {
        Logger { message: self.message, level: Level::Debug, code: self.code, timestamp: self.timestamp }
    }
}

/// Entry function
pub fn log(message: String) -> LoggerBuilder {
    LoggerBuilder::new(message)
}
