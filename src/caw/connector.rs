use std::time::{Duration, Instant};

use super::devices::device::Device;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub enum ConnectorError {
    Timeout,
}

impl std::fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConnectorError::Timeout => {
                write!(f, "timeout")
            }
        }
    }
}

impl std::error::Error for ConnectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConnectorError::Timeout => None,
        }
    }
}

pub struct Connector {
    device: Box<dyn Device + Send>,
    timeout: Instant,
}

impl Connector {
    pub fn new(device: Box<dyn Device + Send>) -> Self {
        Self {
            device,
            timeout: Instant::now(),
        }
    }

    pub fn check_timeout(&self) -> bool {
        if self.timeout.elapsed().as_millis() > 6000u128 {
            return true;
        }
        false
    }

    pub async fn event_loop(&mut self) {
        loop {}
    }
}
