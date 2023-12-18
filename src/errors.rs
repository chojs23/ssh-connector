use core::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    ConnectionError(String),
    ConfigError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}
