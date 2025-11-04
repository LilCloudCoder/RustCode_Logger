use chrono::Local;

/// Log levels
#[derive(Debug, Clone, Copy)]
pub enum Level {
    Info,
    Warn,
    Error,
    Debug,
}

/// ANSI color configuration for each level.
#[derive(Debug, Clone)]
pub struct AnsiColors {
    pub info: String,
    pub warn: String,
    pub error: String,
    pub debug: String,
    pub reset: String,
}

impl Default for AnsiColors {
    fn default() -> Self {
        Self {
            info: "\x1b[32m".to_string(),  // green
            warn: "\x1b[33m".to_string(),  // yellow
            error: "\x1b[31m".to_string(), // red
            debug: "\x1b[34m".to_string(), // blue
            reset: "\x1b[0m".to_string(),
        }
    }
}

/// Logger struct
pub struct Logger {
    pub message: String,
    pub level: Level,
    pub code: Option<i32>,
    pub timestamp: bool,
    pub ts_format: Option<String>,
    pub colors: Option<AnsiColors>,
    pub color_enabled: bool,
}

impl Logger {
    pub fn print(&self) {
        // Resolve color prefix
        let (prefix, reset) = if self.color_enabled {
            let colors = self.colors.as_ref().unwrap_or(&AnsiColors::default());
            let (c, tag) = match self.level {
                Level::Info => (&colors.info, "INFO"),
                Level::Warn => (&colors.warn, "WARN"),
                Level::Error => (&colors.error, "ERROR"),
                Level::Debug => (&colors.debug, "DEBUG"),
            };
            (format!("{}[{}]{}", c, tag, colors.reset), colors.reset.clone())
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
