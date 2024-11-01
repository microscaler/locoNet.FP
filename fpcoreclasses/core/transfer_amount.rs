use serde::{Deserialize, Serialize};

/// Represents the credentials needed for transfers (assumed to be a struct in your code)
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    // Add fields relevant to Credentials
    // e.g., username: String, password: String, etc.
}

/// Represents the amount to be transferred
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAmount {
    #[serde(rename = "amount")]
    pub amount: f64, // Using f64 for decimal representation
}

impl TransferAmount {
    /// Creates a new TransferAmount with a specified amount
    pub fn new(amount: f64) -> Self {
        Self { amount }
    }
}
