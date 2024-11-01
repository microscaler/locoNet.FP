#[derive(Debug, Default)]
pub struct ServerVariables {
    pub version: String,
    pub server_id: String,
    pub auto_detect: bool,
    pub udp_beacon_port: u16,
}

impl ServerVariables {
    pub fn new() -> Self {
        ServerVariables {
            version: String::new(),
            server_id: String::new(),
            auto_detect: true,
            udp_beacon_port: 8001,
        }
    }
}
