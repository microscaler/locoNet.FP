use serde::{Deserialize, Serialize};

/// Represents a raw request frame.
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestFrame {
    /// The raw request, including the command prefix.
    pub raw_request: String,
}

impl Default for RequestFrame {
    fn default() -> Self {
        RequestFrame {
            raw_request: String::new(), // Initialize with an empty string
        }
    }
}
