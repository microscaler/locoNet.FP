use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    #[serde(rename = "")]
    Unspecified,
    Cash,
    Check,
    Coupons,
    ExtCoupons,
    Packaging,
    InternalUsage,
    Damage,
    Card,
    Bank,
    Reserved1,
    Reserved2,
    #[serde(rename = "change")]
    Change,
}

impl Default for PaymentType {
    fn default() -> Self {
        PaymentType::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Payment {
    /// The type of the payment.
    #[serde(rename = "payment_type")]
    pub payment_type: PaymentType,
    /// The amount of the payment.
    #[serde(rename = "amount")]
    pub amount: f64,
}

impl Payment {
    pub fn new(payment_type: PaymentType, amount: f64) -> Self {
        Self {
            payment_type,
            amount,
        }
    }
}
