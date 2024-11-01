// configuration/printer_config.rs test
#[derive(Debug, Default)]
pub struct PrinterConfig {
    pub uri: String,
}

#[derive(Debug, Default)]
pub struct PrinterConfigWithId {
    pub uri: String,
    pub id: String,
}

impl PrinterConfigWithId {
    pub fn new(uri: String, id: String) -> Self {
        Self { uri, id }
    }
}
