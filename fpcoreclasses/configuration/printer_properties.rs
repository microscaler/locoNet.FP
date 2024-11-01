// configuration/printer_properties.rs
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PrinterProperties {
    pub payment_type_mappings: HashMap<String, String>,
    pub printer_constants: HashMap<String, String>,
    pub printer_options: HashMap<String, String>,
}
