use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize}; // For handling DateTime

/// Represents information returned after printing a receipt.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptInfo {
    /// The receipt number.
    pub receipt_number: String,

    /// The receipt date and time.
    pub receipt_date_time: DateTime<Utc>, // Using chrono for date and time

    /// The receipt amount.
    pub receipt_amount: f64, // Using f64 to represent decimal values

    /// The fiscal memory number.
    pub fiscal_memory_serial_number: String,
}

impl Default for ReceiptInfo {
    fn default() -> Self {
        ReceiptInfo {
            receipt_number: String::new(),
            receipt_date_time: Utc::now(), // Initialize with current time
            receipt_amount: 0.0,
            fiscal_memory_serial_number: String::new(),
        }
    }
}
