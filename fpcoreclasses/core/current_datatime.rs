// src/current_date_time.rs

use crate::credentials::Credentials;
use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Default)]
pub struct CurrentDateTime {
    /// Credentials from the base struct.
    pub credentials: Credentials,

    /// Date and time of the device.
    pub device_date_time: NaiveDateTime,
}

impl CurrentDateTime {
    pub fn new(credentials: Credentials, device_date_time: NaiveDateTime) -> Self {
        Self {
            credentials,
            device_date_time,
        }
    }
}

impl Default for CurrentDateTime {
    fn default() -> Self {
        Self {
            credentials: Credentials::default(),
            device_date_time: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}
