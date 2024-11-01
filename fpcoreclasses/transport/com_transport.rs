use log::{error, info};
use serialport::{new, SerialPort};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const DEFAULT_BAUD_RATE: u32 = 115200;
const DEFAULT_TIMEOUT: u64 = 800; // milliseconds
const DEFAULT_TIMEOUT_TO_CLOSE: u64 = 3000; // milliseconds

pub trait IChannel {
    fn open(&mut self);
    fn close(&mut self);
    fn read(&mut self) -> io::Result<Vec<u8>>;
    fn write(&mut self, data: &[u8]) -> io::Result<()>;
}

pub trait Transport {
    fn open_channel(&mut self, address: &str) -> Arc<Mutex<dyn IChannel>>;
    fn drop_channel(&mut self, channel: Arc<Mutex<dyn IChannel>>);
    fn get_available_addresses(&self) -> Vec<(String, String)>;
}

pub struct ComTransport {
    opened_channels: HashMap<String, Arc<Mutex<dyn IChannel>>>,
}

impl ComTransport {
    pub fn new() -> Self {
        Self {
            opened_channels: HashMap::new(),
        }
    }

    fn parse_address(&self, address: &str) -> (String, u32) {
        let parts: Vec<&str> = address.split(':').collect();
        if parts.len() == 1 {
            (address.to_string(), DEFAULT_BAUD_RATE)
        } else {
            let baud_rate = parts
                .get(1)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(DEFAULT_BAUD_RATE);
            (parts[0].to_string(), baud_rate)
        }
    }
}

impl Transport for ComTransport {
    fn open_channel(&mut self, address: &str) -> Arc<Mutex<dyn IChannel>> {
        if let Some(channel) = self.opened_channels.get(address) {
            return channel.clone();
        }
        let (com_port, baud_rate) = self.parse_address(address);
        let channel = Arc::new(Mutex::new(ComChannel::new(&com_port, baud_rate)));
        self.opened_channels
            .insert(com_port.clone(), channel.clone());
        channel
    }

    fn drop_channel(&mut self, channel: Arc<Mutex<dyn IChannel>>) {
        channel.lock().unwrap().close();
    }

    fn get_available_addresses(&self) -> Vec<(String, String)> {
        serialport::available_ports()
            .unwrap()
            .iter()
            .map(|port| (port.port_name.clone(), port.port_name.clone()))
            .collect()
    }
}

pub struct ComChannel {
    port_name: String,
    baud_rate: u32,
    serial_port: Option<Box<dyn SerialPort>>,
    last_active: Instant,
}

impl ComChannel {
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        Self {
            port_name: port_name.to_string(),
            baud_rate,
            serial_port: None,
            last_active: Instant::now(),
        }
    }

    fn open_serial_port(&mut self) -> io::Result<()> {
        self.serial_port = Some(new(&self.port_name, self.baud_rate)?);
        info!("Opening the COM port {}", self.port_name);
        Ok(())
    }

    fn close_serial_port(&mut self) {
        if let Some(port) = self.serial_port.take() {
            info!("Closing the COM port {}", self.port_name);
        }
    }

    fn idle_timeout(&self) -> bool {
        self.last_active.elapsed() > Duration::from_millis(DEFAULT_TIMEOUT_TO_CLOSE)
    }
}

impl IChannel for ComChannel {
    fn open(&mut self) {
        if self.serial_port.is_none() {
            self.open_serial_port().expect("Failed to open serial port");
        }
        self.last_active = Instant::now();
    }

    fn close(&mut self) {
        self.close_serial_port();
    }

    fn read(&mut self) -> io::Result<Vec<u8>> {
        self.open(); // Ensure channel is open
        let mut buffer = vec![0; 1024]; // Example buffer size
        let bytes_read = self.serial_port.as_mut().unwrap().read(&mut buffer)?;
        buffer.truncate(bytes_read); // Resize buffer to number of bytes read
        self.last_active = Instant::now();
        Ok(buffer)
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.open(); // Ensure channel is open
        self.serial_port.as_mut().unwrap().write_all(data)?;
        self.last_active = Instant::now();
        Ok(())
    }
}
