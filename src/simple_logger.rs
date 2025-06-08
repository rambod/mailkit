use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::sync::OnceLock;

pub struct SimpleLogger {
    level: LevelFilter,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl SimpleLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        static LOGGER: OnceLock<SimpleLogger> = OnceLock::new();
        let level = match std::env::var("RUST_LOG").as_deref() {
            Ok("trace") => LevelFilter::Trace,
            Ok("debug") => LevelFilter::Debug,
            Ok("warn") => LevelFilter::Warn,
            Ok("error") => LevelFilter::Error,
            _ => LevelFilter::Info,
        };
        let logger = LOGGER.get_or_init(|| SimpleLogger { level });
        log::set_logger(logger)?;
        log::set_max_level(level);
        Ok(())
    }
}
