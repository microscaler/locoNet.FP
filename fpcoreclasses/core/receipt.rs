use serde::{Deserialize, Serialize};
use serde_json::Value; // Include this for handling potential JSON serialization
use std::collections::HashMap;

/// Represents the credentials for the receipt.
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    /// Operator Name or Operator ID.
    pub operator: String,

    /// Operator Password.
    pub operator_password: String,
}

/// Represents one line in a receipt.
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_code: i32,
    pub item_type: ItemType,
    pub text: String,
    pub tax_group: TaxGroup,
    pub department: i32,
    pub quantity: f64,
    pub unit_price: f64,
    pub amount: f64,
    pub price_modifier_value: f64,
    pub price_modifier_type: PriceModifierType,
}

/// Represents the payment details for a receipt.
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub payment_type: String,
    pub amount: f64,
}

/// Represents one Receipt, which can be printed on a fiscal printer.
#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    /// The unique sale number is a fiscally controlled number.
    pub unique_sale_number: String,

    /// The line items of the receipt.
    #[serde(rename = "items")]
    pub items: Option<Vec<Item>>, // Using Option<Vec<Item>> to allow for nullability

    /// The payments of the receipt.
    /// The total amount should match the total amount of the line items.
    #[serde(rename = "payments")]
    pub payments: Option<Vec<Payment>>, // Using Option<Vec<Payment>> to allow for nullability
}

impl Default for Receipt {
    fn default() -> Self {
        Receipt {
            unique_sale_number: String::new(),
            items: Some(Vec::new()),
            payments: Some(Vec::new()),
        }
    }
}
