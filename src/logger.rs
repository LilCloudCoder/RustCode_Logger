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
        let prefix = if self.color_enabled {
            let colors = self.colors.as_ref().cloned().unwrap_or_default();
            let (c, tag) = match self.level {
                Level::Info => (&colors.info, "INFO"),
                Level::Warn => (&colors.warn, "WARN"),
                Level::Error => (&colors.error, "ERROR"),
                Level::Debug => (&colors.debug, "DEBUG"),
            };
            format!("{}[{}]{}", c, tag, colors.reset)
        } else {
            format!("[{}]", match self.level { Level::Info => "INFO", Level::Warn => "WARN", Level::Error => "ERROR", Level::Debug => "DEBUG" })
        };

        // Timestamp
        let ts = if self.timestamp {
            let fmt = self.ts_format.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S");
            format!("[{}] ", Local::now().format(fmt))
        } else {
            String::new()
        };

        // Optional code
        let code_str = self.code.map(|c| format!(" (code {})", c)).unwrap_or_default();

        // Select output stream: warnings/errors to stderr; others to stdout
        match self.level {
            Level::Warn | Level::Error => eprintln!("{}{} {}{}", ts, prefix, self.message, code_str),
            _ => println!("{}{} {}{}", ts, prefix, self.message, code_str),
        }
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
    pub ts_format: Option<String>,
    pub colors: Option<AnsiColors>,
    pub color_enabled: bool,
}

impl LoggerBuilder {
    pub fn new(message: String) -> Self {
        Self { message, code: None, timestamp: false, ts_format: None, colors: None, color_enabled: true }
    }

    /// Set a custom code to be appended to the message.
    pub fn code(mut self, code: i32) -> Self {
        self.code = Some(code);
        self
    }

    /// Enable timestamps with default format.
    pub fn timestamp(mut self) -> Self {
        self.timestamp = true;
        if self.ts_format.is_none() { self.ts_format = Some("%Y-%m-%d %H:%M:%S".to_string()); }
        self
    }

    /// Use a custom timestamp format (enables timestamping).
    pub fn timestamp_format<S: Into<String>>(mut self, fmt: S) -> Self {
        self.timestamp = true;
        self.ts_format = Some(fmt.into());
        self
    }

    /// Disable ANSI colors in output.
    pub fn no_color(mut self) -> Self {
        self.color_enabled = false;
        self
    }

    /// Provide full custom ANSI color codes for all levels.
    pub fn colors(mut self, colors: AnsiColors) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Override color for a specific level with a raw ANSI code prefix (e.g., "\x1b[35m").
    pub fn color_for_level<S: Into<String>>(mut self, level: Level, ansi_prefix: S) -> Self {
        let mut cfg = self.colors.take().unwrap_or_default();
        let s = ansi_prefix.into();
        match level {
            Level::Info => cfg.info = s,
            Level::Warn => cfg.warn = s,
            Level::Error => cfg.error = s,
            Level::Debug => cfg.debug = s,
        }
        self.colors = Some(cfg);
        self
    }

    fn build(self, level: Level) -> Logger {
        Logger {
            message: self.message,
            level,
            code: self.code,
            timestamp: self.timestamp,
            ts_format: self.ts_format,
            colors: self.colors,
            color_enabled: self.color_enabled,
        }
    }

    pub fn info(self) -> Logger { self.build(Level::Info) }
    pub fn warn(self) -> Logger { self.build(Level::Warn) }
    pub fn error(self) -> Logger { self.build(Level::Error) }
    pub fn debug(self) -> Logger { self.build(Level::Debug) }
}

/// Entry function
pub fn log(message: String) -> LoggerBuilder {
    LoggerBuilder::new(message)
}
