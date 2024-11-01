// configuration/service_options.rs
use crate::device_info::DeviceInfo;
use crate::payment::PaymentType; // припускаємо, що PaymentType вже у файлі payment.rs
use crate::printer_config::PrinterConfig;
use crate::printer_properties::PrinterProperties;
use std::collections::HashMap;
use std::sync::{Arc, RwLock}; // припускаємо, що DeviceInfo вже у файлі device_info.rs

#[derive(Debug, Default)]
pub struct ServiceOptions {
    pub auto_detect: bool,
    pub server_id: String,
    pub printers: HashMap<String, PrinterConfig>,
    pub udp_beacon_port: i32,
    pub printers_properties: Arc<RwLock<HashMap<String, PrinterProperties>>>,
}

impl ServiceOptions {
    pub fn new() -> Self {
        Self {
            auto_detect: true,
            server_id: String::new(),
            printers: HashMap::new(),
            udp_beacon_port: 8001,
            printers_properties: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn remap_payment_types(&self, serial_number: &str, map: &mut HashMap<PaymentType, String>) {
        let printers_properties = self.printers_properties.read().unwrap();

        if let Some(printer_properties) = printers_properties.get(serial_number) {
            for pmt in PaymentType::iterator() {
                let serialized_key = serde_json::to_string(pmt)
                    .unwrap()
                    .trim_matches('"')
                    .to_string();
                if let Some(new_value) = printer_properties
                    .payment_type_mappings
                    .get(&serialized_key)
                {
                    if !new_value.is_empty() {
                        map.insert(*pmt, new_value.clone());
                    }
                }
            }
        }
    }

    pub fn reconfigure_printer_constants(&self, info: &mut DeviceInfo) {
        let printers_properties = self.printers_properties.read().unwrap();

        let printer_properties = printers_properties
            .entry(info.serial_number.clone())
            .or_insert_with(PrinterProperties::default);

        if let Some(comment_text_max_length) = printer_properties
            .printer_constants
            .get("commentTextMaxLength")
        {
            if let Ok(value) = comment_text_max_length.parse::<i32>() {
                if value > 0 {
                    info.comment_text_max_length = value;
                }
            }
        }

        if let Some(item_text_max_length) = printer_properties
            .printer_constants
            .get("itemTextMaxLength")
        {
            if let Ok(value) = item_text_max_length.parse::<i32>() {
                if value > 0 {
                    info.item_text_max_length = value;
                }
            }
        }
    }

    pub fn reconfigure_printer_options(&self, info: &mut DeviceInfo) {
        let printers_properties = self.printers_properties.read().unwrap();

        let printer_properties = printers_properties
            .entry(info.serial_number.clone())
            .or_insert_with(PrinterProperties::default);

        if let Some(support_payment_terminal) = printer_properties
            .printer_options
            .get("supportPaymentTerminal")
        {
            if let Ok(value) = support_payment_terminal.parse::<bool>() {
                info.support_payment_terminal = value;
            }
        } else {
            printer_properties.printer_options.insert(
                "supportPaymentTerminal".to_string(),
                info.support_payment_terminal.to_string(),
            );
        }

        if let Some(use_payment_terminal) =
            printer_properties.printer_options.get("usePaymentTerminal")
        {
            if let Ok(value) = use_payment_terminal.parse::<bool>() {
                info.use_payment_terminal = value;
            }
        } else {
            printer_properties.printer_options.insert(
                "usePaymentTerminal".to_string(),
                info.use_payment_terminal.to_string(),
            );
        }
    }
}
