use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    io,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::{Duration, Instant},
};

use bincode::config::{self, BigEndian, Configuration, Fixint};
use tokio::{runtime::Handle, sync::mpsc, task::JoinHandle};

use super::{
    devices::device::{self, Device},
    protocols::{
        code::{CmdCode, SystemCode},
        pingpong::Pingpong,
        protocol::{self, ProtocolHeader},
    },
};

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

type EventCallback = Box<dyn Fn(&[u8]) + Send>;

pub struct Connector {
    device: Arc<Mutex<Box<dyn Device + Send>>>,
    timeout: Instant,
    config: Configuration<BigEndian, Fixint>,
    task: Option<JoinHandle<()>>,
}

impl Drop for Connector {
    fn drop(&mut self) {
        if let Ok(device) = self.device.lock() {
            println!("Connector drop: id:{:?}", device.get_id());
        }
    }
}

impl Connector {
    pub fn new(device: Box<dyn Device + Send>) -> Self {
        Self {
            device: Arc::new(Mutex::new(device)),
            timeout: Instant::now(),
            config: config::standard()
                .with_fixed_int_encoding()
                .with_big_endian(),
            task: None,
        }
    }

    pub fn check_timeout(&self) -> bool {
        if self.timeout.elapsed().as_millis() > 6000u128 {
            return true;
        }
        false
    }

    pub fn event_loop(&mut self) {
        println!("event_loop {:?}", Handle::try_current());
        let device = Arc::clone(&self.device);
        tokio::task::spawn_blocking(move || {
            let mut tmp_buf = [0; 1024];
            let mut buf = vec![];
            let mut index = 0usize;
            loop {
                if let Ok(mut device) = device.lock() {
                    let _ = device
                        .read(&mut tmp_buf[index..])
                        .map(|size| {
                            buf.append(&mut tmp_buf[..size].to_vec());
                            index += size
                        })
                        .and_then(|_| ProtocolHeader::parse(&buf[..]))
                        .map(|header| {
                            if header.get_data_size() as usize + protocol::HEADER_SIZE >= buf.len()
                            {
                                println!("{:?}", header);
                            }
                            header.get_data_size() as usize + protocol::HEADER_SIZE
                        })
                        .map(|size| {
                            buf.drain(0..size);
                            index -= size;
                        });
                }
            }
        });
    }
}
