use std::io;

use failure::Fail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Fail, Serialize, Deserialize)]
pub enum KvsError {
    /// can not open log file
    #[fail(display = "log file io error: {}", msg)]
    LogFileIOError { msg: String },
    /// unknown command for kvs
    #[fail(display = "unknown command")]
    UnknownCmd,
    #[fail(display = "serialization error: {}", msg)]
    SerializationError { msg: String },
    /// key does not exist
    #[fail(display = "Key not found")]
    KeyNotFound,
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;

impl From<io::Error> for KvsError {
    fn from(value: io::Error) -> Self {
        KvsError::LogFileIOError {
            msg: value.to_string(),
        }
    }
}
