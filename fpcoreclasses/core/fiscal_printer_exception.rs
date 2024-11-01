use std::error::Error;
use std::fmt;
use std::io;

/// Exception for the fiscal printer
#[derive(Debug)]
pub struct FiscalPrinterException {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl FiscalPrinterException {
    // Creating a new exception with a message
    pub fn new(message: &str) -> Self {
        FiscalPrinterException {
            message: message.to_string(),
            source: None,
        }
    }

    // Creating a new exception with a message and an underlying error
    pub fn with_source(message: &str, source: Box<dyn Error>) -> Self {
        FiscalPrinterException {
            message: message.to_string(),
            source: Some(source),
        }
    }
}

// Implementation of the trait for formatting the error message
impl fmt::Display for FiscalPrinterException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FiscalPrinterException: {}", self.message)
    }
}

// Implementation of the trait for getting the underlying error
impl Error for FiscalPrinterException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

// Example usage
fn main() -> Result<(), FiscalPrinterException> {
    // Generating an exception
    let error = FiscalPrinterException::new("An error occurred in the fiscal printer.");
    Err(error)
}
