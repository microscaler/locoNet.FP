use log::{error, info};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const DEFAULT_PORT: u16 = 9100;

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

pub struct TcpTransport {
    opened_channels: HashMap<String, Arc<Mutex<dyn IChannel>>>,
}

impl TcpTransport {
    pub fn new() -> Self {
        Self {
            opened_channels: HashMap::new(),
        }
    }

    fn parse_address(&self, address: &str) -> (String, u16) {
        let parts: Vec<&str> = address.split(':').collect();
        if parts.len() == 1 {
            (address.to_string(), DEFAULT_PORT)
        } else {
            let port = parts
                .get(1)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(DEFAULT_PORT);
            (parts[0].to_string(), port)
        }
    }
}

impl Transport for TcpTransport {
    fn open_channel(&mut self, address: &str) -> Arc<Mutex<dyn IChannel>> {
        if let Some(channel) = self.opened_channels.get(address) {
            return channel.clone();
        } else {
            let (host_name, port) = self.parse_address(address);
            let channel = Arc::new(Mutex::new(TcpChannel::new(&host_name, port)));
            self.opened_channels
                .insert(address.to_string(), channel.clone());
            return channel;
        }
    }

    fn drop_channel(&mut self, channel: Arc<Mutex<dyn IChannel>>) {
        channel.lock().unwrap().close();
    }

    fn get_available_addresses(&self) -> Vec<(String, String)> {
        // This function is a placeholder. In a real application, you would
        // implement logic to retrieve available TCP addresses.
        vec![]
    }
}

pub struct TcpChannel {
    stream: TcpStream,
    host_name: String,
    port: u16,
}

impl TcpChannel {
    pub fn new(host_name: &str, port: u16) -> Self {
        let addr = format!("{}:{}", host_name, port);
        let stream = TcpStream::connect(&addr).expect("Failed to connect");
        info!("Connected to {}", addr);
        Self {
            stream,
            host_name: host_name.to_string(),
            port,
        }
    }
}

impl IChannel for TcpChannel {
    fn open(&mut self) {
        // In Rust, we usually open a TCP channel in the constructor.
        // Repeated open calls are not necessary unless you close the connection.
    }

    fn close(&mut self) {
        self.stream
            .shutdown(std::net::Shutdown::Both)
            .expect("Failed to close stream");
        info!("Closed connection to {}:{}", self.host_name, self.port);
    }

    fn read(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; 1024]; // Example buffer size
        let bytes_read = self.stream.read(&mut buffer)?;
        buffer.truncate(bytes_read); // Resize buffer to number of bytes read
        Ok(buffer)
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_all(data)?;
        Ok(())
    }
}

fn main() {
    // Example usage
    env_logger::init(); // Initialize logging
    let mut tcp_transport = TcpTransport::new();

    let channel = tcp_transport.open_channel("127.0.0.1:9100");
    let mut channel_lock = channel.lock().unwrap();

    let data_to_write = b"Hello, Server!";
    channel_lock
        .write(data_to_write)
        .expect("Failed to write data");

    let received_data = channel_lock.read().expect("Failed to read data");
    println!("Received: {:?}", String::from_utf8_lossy(&received_data));
}
