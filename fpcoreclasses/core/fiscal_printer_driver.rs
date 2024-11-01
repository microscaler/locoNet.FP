// core/fiscal_printer_driver.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::{IChannel, IFiscalPrinter, ServiceOptions};

// The FiscalPrinterDriver interface in Rust
pub trait FiscalPrinterDriver {
    // Method to get the name of the driver
    fn driver_name(&self) -> &str;

    /// Method to connect to the fiscal printer
    fn connect(
        &self,
        channel: Arc<dyn IChannel>,
        service_options: ServiceOptions,
        auto_detect: bool,
        options: Option<HashMap<String, String>>,
    ) -> Arc<dyn IFiscalPrinter>;
}

// Structure ExpiringCache for caching with expiration times
#[derive(Clone)]
pub struct ExpiringCache<K, V> {
    data: Arc<Mutex<HashMap<K, (V, Instant)>>>,
    expiration_duration: Duration,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> ExpiringCache<K, V> {
    pub fn new(expiration_duration: Duration) -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            expiration_duration,
        }
    }

    // Method to retrieve a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.expiration_duration {
                return Some(value.clone());
            } else {
                data.remove(key); // Remove the old value if it has expired
            }
        }
        None
    }

    // Method to set a value in the cache
    pub fn set(&self, key: K, value: V) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, (value, Instant::now()));
    }
}

// Creating an instance of the cache for use with FiscalPrinterDriver
lazy_static::lazy_static! {
    pub static ref CACHE: ExpiringCache<String, String> = ExpiringCache::new(Duration::from_secs(300));
}
