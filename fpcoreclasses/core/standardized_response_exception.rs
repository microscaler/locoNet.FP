use serde::{Deserialize, Serialize};
use std::fmt;

/// Enum to represent the type of status message
#[derive(Debug, Serialize, Deserialize)]
pub enum StatusMessageType {
    Error,
    Warning,
    Info,
    // Add more variants as needed
}

/// Custom exception for standardized status messages
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardizedStatusMessageException {
    pub message: String,
    pub code: String,
    pub status_type: StatusMessageType,
}

impl StandardizedStatusMessageException {
    /// Creates a new instance of the exception with a message.
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            code: String::new(),
            status_type: StatusMessageType::Error,
        }
    }

    /// Creates a new instance of the exception with a message and code.
    pub fn with_code(message: &str, code: &str) -> Self {
        Self {
            message: message.to_string(),
            code: code.to_string(),
            status_type: StatusMessageType::Error,
        }
    }

    /// Creates a new instance of the exception with a message, code, and status type.
    pub fn with_code_and_type(message: &str, code: &str, status_type: StatusMessageType) -> Self {
        Self {
            message: message.to_string(),
            code: code.to_string(),
            status_type,
        }
    }

    /// Creates a new instance of the exception with a message and a cause (inner exception).
    pub fn with_inner(message: &str, inner: Box<dyn std::error::Error>) -> Self {
        Self {
            message: message.to_string(),
            code: String::new(),
            status_type: StatusMessageType::Error,
            // Note: Inner error handling can be managed differently in Rust
        }
    }
}

// Implementing fmt::Display to customize the display of the exception
impl fmt::Display for StandardizedStatusMessageException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {}, Code: {}, Type: {:?}",
            self.message, self.code, self.status_type
        )
    }
}

// Implementing std::error::Error trait for compatibility
impl std::error::Error for StandardizedStatusMessageException {}
