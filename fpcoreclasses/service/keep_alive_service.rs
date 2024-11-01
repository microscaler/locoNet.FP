use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::{self, Interval};
use tokio::task;
use log::{info, error};

pub trait IServiceController {
    fn is_ready(&self) -> bool;
    fn printers(&self) -> Vec<Box<dyn IFiscalPrinter>>;
}

pub trait IFiscalPrinter {
    fn check_status(&self);
}

pub struct KeepAliveService {
    context: Arc<dyn IServiceController + Send + Sync>,
    interval: Interval,
    is_running: bool,
}

impl KeepAliveService {
    pub fn new(context: Arc<dyn IServiceController + Send + Sync>) -> Self {
        let interval = time::interval(Duration::from_secs(120));
        Self {
            context,
            interval,
            is_running: false,
        }
    }

    pub async fn start(&mut self) {
        if self.is_running {
            return;
        }
        self.is_running = true;

        info!("Keep Alive Background Service is started.");
        
        loop {
            self.interval.tick().await;

            if !self.is_running {
                break;
            }

            self.keep_alive_with_get_status().await;
        }
    }

    async fn keep_alive_with_get_status(&self) {
        info!("Keep Alive Background Service running...");
        let printers = self.context.printers();
        
        if self.context.is_ready() {
            for printer in printers {
                printer.check_status();
            }
        }
        
        info!("Keep Alive Background Service done.");
    }

    pub async fn stop(&mut self) {
        self.is_running = false;
        info!("Keep Alive Background Service is stopped.");
    }
}

impl Drop for KeepAliveService {
    fn drop(&mut self) {
        self.stop();
    }
}