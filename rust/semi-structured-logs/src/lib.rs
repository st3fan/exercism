/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Debug => debug(message),
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}

pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}

pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}
