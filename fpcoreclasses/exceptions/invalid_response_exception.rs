use std::fmt;

/// A custom error type for invalid responses.
#[derive(Debug)]
pub struct InvalidResponseError {
    message: String,
    // Optionally, you can store the cause of the error
    source: Option<Box<dyn std::error::Error>>,
}

impl InvalidResponseError {
    /// Creates a new `InvalidResponseError` with a message.
    pub fn new(message: &str) -> Self {
        InvalidResponseError {
            message: message.to_string(),
            source: None,
        }
    }

    /// Creates a new `InvalidResponseError` with a message and a source.
    pub fn with_source(message: &str, source: Box<dyn std::error::Error>) -> Self {
        InvalidResponseError {
            message: message.to_string(),
            source: Some(source),
        }
    }
}

// Implementing the `std::fmt::Display` trait for error messages.
impl fmt::Display for InvalidResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref source) = self.source {
            write!(f, "{}: {}", self.message, source)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

// Implementing the `std::error::Error` trait for the error type.
impl std::error::Error for InvalidResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref())
    }
}
