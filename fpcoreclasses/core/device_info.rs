use crate::payment::PaymentType;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct DeviceInfo {
    /// Fiscal printer Uri
    pub uri: String,
    /// Fiscal printer serial number
    pub serial_number: String,
    /// Fiscal printer memory serial number
    pub fiscal_memory_serial_number: String,
    /// Manufacturer - Company or Trademark of Company that produces the fiscal device
    pub manufacturer: String,
    /// Model
    pub model: String,
    /// Optional. Firmware version.
    pub firmware_version: String,
    /// Maximum symbols for operator names, item names, department names allowed.
    pub item_text_max_length: i32,
    /// Maximum symbols for payment names allowed.
    pub comment_text_max_length: i32,
    /// Maximal operator password length allowed;
    pub operator_password_max_length: i32,
    /// Tax Number is Fiscal Subject Identification Number
    pub tax_identification_number: String,
    /// List of supported payment types by the device
    pub supported_payment_types: HashSet<PaymentType>,
    /// Expresses support of item types discount-amount and surcharge-amount by the device
    pub supports_sub_total_amount_modifiers: bool,
    /// Expresses support of payment terminal for current device model
    pub support_payment_terminal: bool,
    /// Expresses using of payment terminal for current device
    pub use_payment_terminal: bool,
}

impl DeviceInfo {
    pub fn new() -> Self {
        Self {
            uri: String::new(),
            serial_number: String::new(),
            fiscal_memory_serial_number: String::new(),
            manufacturer: String::new(),
            model: String::new(),
            firmware_version: String::new(),
            item_text_max_length: 0,
            comment_text_max_length: 0,
            operator_password_max_length: 0,
            tax_identification_number: String::new(),
            supported_payment_types: HashSet::new(),
            supports_sub_total_amount_modifiers: false,
            support_payment_terminal: false,
            use_payment_terminal: false,
        }
    }
}
