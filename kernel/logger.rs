use crate::println;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct Logger;

impl log::Log for Logger {
    #[cfg(feature = "log_error")]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Error
    }
    #[cfg(feature = "log_warn")]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Warn
    }
    #[cfg(feature = "log_debug")]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }
    #[cfg(feature = "log_info")]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    #[cfg(feature = "log_trace")]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }
    #[cfg(not(any(
        feature = "log_error",
        feature = "log_warn",
        feature = "log_debug",
        feature = "log_info",
        feature = "log_trace"
    )))]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

#[cfg(feature = "log_error")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Error))
}

#[cfg(feature = "log_warn")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Warn))
}

#[cfg(feature = "log_debug")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

#[cfg(feature = "log_info")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

#[cfg(feature = "log_trace")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

#[cfg(not(any(
    feature = "log_error",
    feature = "log_warn",
    feature = "log_debug",
    feature = "log_info",
    feature = "log_trace"
)))]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
