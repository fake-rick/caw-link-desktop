use std::{
    fmt::Debug,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use tokio::{runtime::Handle, task::JoinHandle};

use super::{
    devices::device::Device,
    event::Event,
    protocols::protocol::{self, ProtocolHeader},
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
    task: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl Drop for Connector {
    fn drop(&mut self) {
        if let Ok(device) = self.device.lock() {
            println!("Connector drop: id:{:?}", device.get_id());
            self.running.store(false, Ordering::Relaxed);
        }
    }
}

impl Connector {
    pub fn new(device: Box<dyn Device + Send>) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        Self {
            device: Arc::new(Mutex::new(device)),
            timeout: Instant::now(),
            task: None,
            running,
        }
    }

    pub fn check_timeout(&self) -> bool {
        if self.timeout.elapsed().as_millis() > 6000u128 {
            return true;
        }
        false
    }

    pub fn event_loop(&mut self, mut event: Event) {
        println!("event_loop {:?}", Handle::try_current());
        let device = Arc::clone(&self.device);
        let running = Arc::clone(&self.running);
        self.task = Some(tokio::task::spawn_blocking(move || -> () {
            let mut tmp_buf = [0; 1024];
            let mut buf = vec![];
            let mut index = 0usize;

            while running.load(Ordering::Relaxed) {
                if let Ok(mut device) = device.lock() {
                    let _ = device
                        .read(&mut tmp_buf[index..])
                        .or_else(|e| {
                            if let Some(err) = e.downcast_ref::<io::Error>() {
                                match err.kind() {
                                    io::ErrorKind::TimedOut => (),
                                    _ => (),
                                }
                            }
                            Err(e)
                        })
                        .map(|size| {
                            buf.append(&mut tmp_buf[..size].to_vec());
                            index += size
                        })
                        .and_then(|_| ProtocolHeader::parse(&buf[..]))
                        .map(|header| {
                            if header.get_data_size() as usize + protocol::HEADER_SIZE >= buf.len()
                            {
                                println!("{:?}", header);
                                event.call(header.get_cmd_code(), &mut device, None);
                            }
                            header.get_data_size() as usize + protocol::HEADER_SIZE
                        })
                        .map(|size| {
                            buf.drain(0..size);
                            index -= size;
                        });
                }
            }
        }));
    }
}
