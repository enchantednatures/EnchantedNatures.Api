use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub addr: [u8; 4],
    pub port: u16,
}

impl ApplicationSettings {
    fn new(addr: [u8; 4], port: u16) -> Self {
        Self { addr, port }
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self::new([127, 0, 0, 1], 6969)
    }
}
