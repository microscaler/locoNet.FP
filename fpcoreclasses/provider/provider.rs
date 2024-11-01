use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use regex::Regex;
use tokio::task;
use log::{info, error};
use std::time::Duration;

#[derive(Debug)]
pub struct ServiceOptions;

pub trait FiscalPrinterDriver {
    fn driver_name(&self) -> &str;
    fn connect(&self, channel: &Channel, service_options: &ServiceOptions, auto_detect: bool, options: Option<&HashMap<String, String>>) -> Box<dyn IFiscalPrinter>;
}

pub trait Transport {
    fn transport_name(&self) -> &str;
    fn get_available_addresses(&self) -> Vec<(String, String)>;
    fn open_channel(&self, address: &str) -> Channel;
    fn drop_channel(&self, channel: &Channel);
}

pub trait IFiscalPrinter {
    fn device_info(&self) -> &DeviceInfo;
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub uri: String,
}

#[derive(Debug)]
pub struct Channel {
    descriptor: String,
}

impl Channel {
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }
}

pub struct Provider {
    service_options: ServiceOptions,
    protocols: HashMap<String, (Box<dyn FiscalPrinterDriver>, Box<dyn Transport>)>,
}

impl Provider {
    pub fn new(service_options: ServiceOptions) -> Self {
        Self {
            service_options,
            protocols: HashMap::new(),
        }
    }

    /// Adds the specified protocol to the provider.
    pub fn register<T: FiscalPrinterDriver + 'static, U: Transport + 'static>(&mut self, driver: T, transport: U) {
        let key = format!("{}.{}", driver.driver_name(), transport.transport_name());
        self.protocols.insert(key, (Box::new(driver), Box::new(transport)));
    }

    pub async fn detect_printer_async(&self, channel: &Channel, transport: &dyn Transport, drivers: &[Box<dyn FiscalPrinterDriver>]) -> Option<Box<dyn IFiscalPrinter>> {
        let mut printer: Option<Box<dyn IFiscalPrinter>> = None;
        let uri_base = format!("{}.{}://{}", drivers[0].driver_name(), transport.transport_name(), channel.descriptor());

        for (i, driver) in drivers.iter().enumerate() {
            info!("Probing ({}/{}): {}...", i + 1, drivers.len(), uri_base);
            let result = task::spawn_blocking(move || {
                driver.connect(channel, &self.service_options, true, None)
            }).await;

            match result {
                Ok(connected_printer) => {
                    printer = Some(connected_printer);
                    info!("Successfully detected {}.", uri_base);
                    break; // Exit the loop on successful detection
                }
                Err(err) => {
                    error!("Error occurred while connecting: {}", err);
                }
            }
        }
        
        printer
    }

    /// Returns the available fiscal printers.
    pub async fn detect_available_printers(&self) -> HashMap<String, Box<dyn IFiscalPrinter>> {
        let mut found_printers = HashMap::new();
        let mut tasks = Vec::new();

        for (key, (driver, transport)) in &self.protocols {
            let addresses = transport.get_available_addresses();
            for (address, _) in addresses {
                let channel = transport.open_channel(&address);
                let drivers = vec![driver.clone()];
                let task = self.detect_printer_async(&channel, transport.as_ref(), &drivers);
                tasks.push(task);
            }
        }

        for task in tasks {
            if let Some(printer) = task.await.unwrap() {
                found_printers.insert(printer.device_info().uri.clone(), printer);
            }
        }

        found_printers
    }

    /// Connects to the fiscal printer with the specified device URI.
    pub fn connect(&self, device_uri: &str, options: Option<HashMap<String, String>>, auto_detect: bool) -> Result<Box<dyn IFiscalPrinter>, String> {
        let uri_pattern = Regex::new(r"^(?P<protocol>.+)://(?P<address>.+)$").unwrap();
        let captures = uri_pattern.captures(device_uri).ok_or(format!("'{}' is not recognized as valid device URI (protocol://address)", device_uri))?;

        let protocol = &captures["protocol"];
        let address = &captures["address"];

        let (driver, transport) = self.protocols.get(protocol).ok_or(format!("Unknown protocol '{}'.", protocol))?;
        let channel = transport.open_channel(address);
        let printer = driver.connect(&channel, &self.service_options, auto_detect, options.as_ref());
        
        let uri = format!("{}.{}://{}", driver.driver_name(), transport.transport_name(), channel.descriptor());
        printer.device_info().uri = uri;

        Ok(printer)
    }
}