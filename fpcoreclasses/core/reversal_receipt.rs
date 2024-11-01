use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Reversal Reason
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)] // Ensures that the enum is represented as a u8 in serialized form
pub enum ReversalReason {
    #[serde(rename = "operator-error")]
    OperatorError = 1,
    #[serde(rename = "refund")]
    Refund = 2,
    #[serde(rename = "tax-base-reduction")]
    TaxBaseReduction = 3,
    #[serde(rename = "taxbase-reduction")]
    OldCompatibleTaxBaseReduction = 3, // Must be the same as TaxBaseReduction
}

/// Represents one Receipt, which can be printed on a fiscal printer.
#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub unique_sale_number: String,
    pub items: Option<Vec<Item>>, // Assuming Item struct is defined elsewhere
    pub payments: Option<Vec<Payment>>, // Assuming Payment struct is defined elsewhere
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    // Define the structure for Item here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    // Define the structure for Payment here
}

/// Represents a reversal receipt.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReversalReceipt {
    pub receipt_number: String,
    pub receipt_date_time: chrono::NaiveDateTime, // Use chrono for date-time
    pub fiscal_memory_serial_number: String,
    #[serde(rename = "reason")]
    pub reason: ReversalReason,
    #[serde(flatten)]
    pub receipt: Receipt, // Incorporates Receipt properties

                          // Additional fields or methods can be added here
}

impl ReversalReceipt {
    /// Clones receipt details from a given Receipt instance.
    pub fn clone_receipt(&mut self, receipt: &Receipt) {
        if let Some(items) = &receipt.items {
            self.receipt.items = Some(items.clone());
        }
        if let Some(payments) = &receipt.payments {
            self.receipt.payments = Some(payments.clone());
        }
        self.receipt.unique_sale_number = receipt.unique_sale_number.clone();
    }
}
