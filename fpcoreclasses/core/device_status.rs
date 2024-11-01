// core/device_status.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Enumeration of StatusMessageType
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum StatusMessageType {
    #[serde(rename = "")]
    Unknown,
    Reserved,
    Info,
    Warning,
    Error,
}

impl Default for StatusMessageType {
    fn default() -> Self {
        StatusMessageType::Unknown
    }
}

// Structure StatusMessage
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StatusMessage {
    #[serde(default)]
    pub message_type: StatusMessageType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    pub text: String,
}

// Main class DeviceStatus
#[derive(Debug, Default)]
pub struct DeviceStatus {
    pub ok: bool,
    pub messages: Vec<StatusMessage>,
}

impl DeviceStatus {
    pub fn new() -> Self {
        Self {
            ok: true,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, status_message: StatusMessage) {
        match status_message.message_type {
            StatusMessageType::Unknown => {
                panic!("status message type cannot be unknown");
            }
            StatusMessageType::Reserved => {
                // Skip messages of type Reserved
                return;
            }
            StatusMessageType::Error => {
                self.ok = false;
            }
            _ => {}
        }

        if status_message.code.as_deref() == Some("") {
            self.messages.push(StatusMessage {
                code: None,
                ..status_message
            });
        } else {
            self.messages.push(status_message);
        }
    }

    pub fn add_info(&mut self, text: String) {
        self.add_message(StatusMessage {
            message_type: StatusMessageType::Info,
            text,
            ..Default::default()
        });
    }

    pub fn add_error(&mut self, code: String, text: String) {
        self.add_message(StatusMessage {
            message_type: StatusMessageType::Error,
            code: Some(code),
            text,
        });
    }

    pub fn add_warning(&mut self, code: String, text: String) {
        self.add_message(StatusMessage {
            message_type: StatusMessageType::Warning,
            code: Some(code),
            text,
        });
    }
}

// DeviceStatus with the DeviceDateTime field
#[derive(Debug)]
pub struct DeviceStatusWithDateTime {
    pub device_date_time: NaiveDateTime,
    pub device_status: DeviceStatus,
}

impl DeviceStatusWithDateTime {
    pub fn new(device_status: DeviceStatus, device_date_time: NaiveDateTime) -> Self {
        Self {
            device_status,
            device_date_time,
        }
    }
}

// DeviceStatus with the RawResponse field
#[derive(Debug)]
pub struct DeviceStatusWithRawResponse {
    pub raw_response: String,
    pub device_status: DeviceStatus,
}

impl DeviceStatusWithRawResponse {
    pub fn new(device_status: DeviceStatus, raw_response: String) -> Self {
        Self {
            device_status,
            raw_response,
        }
    }
}

// DeviceStatus with the Amount field
#[derive(Debug)]
pub struct DeviceStatusWithCashAmount {
    pub amount: f64,
    pub device_status: DeviceStatus,
}

impl DeviceStatusWithCashAmount {
    pub fn new(device_status: DeviceStatus, amount: f64) -> Self {
        Self {
            device_status,
            amount,
        }
    }
}

// DeviceStatus with the ReceiptInfo
#[derive(Debug)]
pub struct DeviceStatusWithReceiptInfo {
    pub receipt_number: String,
    pub receipt_date_time: NaiveDateTime,
    pub receipt_amount: f64,
    pub fiscal_memory_serial_number: String,
    pub device_status: DeviceStatus,
}

// Structure ReceiptInfo
#[derive(Debug)]
pub struct ReceiptInfo {
    pub receipt_number: String,
    pub receipt_date_time: NaiveDateTime,
    pub receipt_amount: f64,
    pub fiscal_memory_serial_number: String,
}

impl DeviceStatusWithReceiptInfo {
    pub fn new(device_status: DeviceStatus, info: ReceiptInfo) -> Self {
        Self {
            device_status,
            receipt_number: info.receipt_number,
            receipt_date_time: info.receipt_date_time,
            receipt_amount: info.receipt_amount,
            fiscal_memory_serial_number: info.fiscal_memory_serial_number,
        }
    }
}
