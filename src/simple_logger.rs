#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Level {
    Error,
    Warn,
    Info,
}

impl core::fmt::Display for Level {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
        };
        f.write_str(s)
    }
}

pub struct SimpleLogger {
    level: Level,
}

static LOGGER: std::sync::OnceLock<SimpleLogger> = std::sync::OnceLock::new();

impl SimpleLogger {
    pub fn init() {
        let level = match std::env::var("RUST_LOG").as_deref() {
            Ok("error") => Level::Error,
            Ok("warn") => Level::Warn,
            _ => Level::Info,
        };
        LOGGER.get_or_init(|| SimpleLogger { level });
    }

    fn log(&self, level: Level, msg: &str) {
        if level <= self.level {
            println!("[{}] {}", level, msg);
        }
    }
}

pub fn log(level: Level, msg: &str) {
    if let Some(logger) = LOGGER.get() {
        logger.log(level, msg);
    } else {
        println!("[{}] {}", level, msg);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::simple_logger::log($crate::simple_logger::Level::Info, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::simple_logger::log($crate::simple_logger::Level::Warn, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::simple_logger::log($crate::simple_logger::Level::Error, &format!($($arg)*));
    };
}
