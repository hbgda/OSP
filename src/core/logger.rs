use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

const LOG_ERROR_COLOUR:     &'static str = "\x1B[38;5;196m";
const LOG_WARNING_COLOUR:   &'static str = "\x1B[38;5;208m";
const LOG_INFO_COLOUR:      &'static str = "\x1B[38;5;255m";
const LOG_SUCCESS_COLOUR:   &'static str = "\x1B[38;5;40m";
const LOG_DEBUG_COLOUR:     &'static str = "\x1B[38;5;8m";
const LOG_TRACE_COLOUR:     &'static str = "\x1B[38;5;240m";

pub struct Logger {
    options: LoggerOptions,
    max_level: LevelFilter,
}

#[derive(Clone, Copy)]
pub struct LoggerOptions {
    pub use_level_colour: bool,
    pub prefix_datetime: bool,
    pub prefix_level: bool,
}

impl LoggerOptions {
    pub fn default() -> Self {
        LoggerOptions {
            use_level_colour: true,
            prefix_datetime: true,
            prefix_level: true,
        }
    }
}

#[derive(Copy, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Success,
    Debug,
    Trace
}

impl Logger {
    // pub fn get() -> Self {
    //     Logger::with_source("Logger")
    // }

    //pub fn with_source(source: &'static str) -> Self {
    //    Logger::with_options(source, LoggerOptions::default())
    //}

    pub fn new() -> Self {
        Self { max_level: LevelFilter::Debug, options: LoggerOptions::default() }
    }

    pub fn options(&mut self, options: LoggerOptions) -> &mut Self {
        self.options = options;
        self
    }

    pub fn max_level(&mut self, level: LevelFilter) -> &mut Self {
        self.max_level = level;
        self
    }

    pub fn set(&self) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(Self { max_level: self.max_level, options: self.options }))?;
        log::set_max_level(self.max_level);
        log::error!("error");
        log::warn!("warn");
        log::info!("info");
        log::debug!("debug");
        log::trace!("trace");
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = &record.args().to_string();
            match record.level() {
                Level::Error => self.log(LogLevel::Error, msg),
                Level::Warn => self.log(LogLevel::Warning, msg),
                Level::Info => self.log(LogLevel::Info, msg),
                Level::Debug => self.log(LogLevel::Debug, msg),
                Level::Trace => self.log(LogLevel::Trace, msg),
            }
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub fn log(&self, level: LogLevel, msg: &str) {
        let log_str = self.make_log_str(level, &msg);
        println!("{log_str}");
    }

    fn make_log_str(&self, level: LogLevel, msg: &str) -> String {
        let log_prefix = self.make_log_prefix(level);
        format!("{log_prefix} {msg} \x1B[0m")
    }

    fn make_log_prefix(&self, level: LogLevel) -> String {
        let mut prefix = String::new();
        if self.options.prefix_datetime {
            let timestamp = chrono::Local::now().format("%Y-%d-%m %H:%M:%S");
            prefix.push_str(&timestamp.to_string());
        }
        if self.options.use_level_colour {
            let colour = match level {
                LogLevel::Info => LOG_INFO_COLOUR,
                LogLevel::Warning => LOG_WARNING_COLOUR,
                LogLevel::Error => LOG_ERROR_COLOUR,
                LogLevel::Success => LOG_SUCCESS_COLOUR,
                LogLevel::Debug => LOG_DEBUG_COLOUR,
                LogLevel::Trace => LOG_TRACE_COLOUR,
            };
            prefix.push_str(colour);
        }
        if self.options.prefix_level {
            let level_name = match level {
                LogLevel::Info => "INFO",
                LogLevel::Warning => "WARNING",
                LogLevel::Error => "ERROR",
                LogLevel::Success => "SUCCESS",
                LogLevel::Debug => "DEBUG",
                LogLevel::Trace => "TRACE",
            };
            prefix.push_str(&format!(" [{}]: ", level_name));
        }

        prefix
    }
}
