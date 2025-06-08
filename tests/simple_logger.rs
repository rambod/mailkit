use mailkit::SimpleLogger;
use mailkit::{info, warn, error};

#[test]
fn logging_macros() {
    SimpleLogger::init();
    info!("info message");
    warn!("warn message");
    error!("error message");
}
