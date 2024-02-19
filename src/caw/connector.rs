use std::{
    fmt::Debug,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

use crate::ui::*;

use slint::Weak;
use tokio::{runtime::Handle, task::JoinHandle};

use crate::caw::protocols::{
    code::{CmdCode, SystemCode},
    pingpong::ping,
};

use super::{
    devices::device::Device,
    event::Event,
    protocols::protocol::{self, ProtocolHeader},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub enum ConnectorError {
    Timeout,
    ParseFaild,
}

impl std::fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConnectorError::Timeout => {
                write!(f, "timeout")
            }
            ConnectorError::ParseFaild => {
                write!(f, "parse faild")
            }
        }
    }
}

impl std::error::Error for ConnectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConnectorError::Timeout => None,
            ConnectorError::ParseFaild => None,
        }
    }
}

type EventCallback = Box<dyn Fn(&[u8]) + Send>;

pub struct Connector {
    device: Arc<Mutex<Box<dyn Device + Send>>>,
    timeout: Arc<Mutex<Instant>>,
    event_task: Option<JoinHandle<()>>,
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
            timeout: Arc::new(Mutex::new(Instant::now())),
            event_task: None,
            running,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn check_timeout(&self) -> bool {
        if let Ok(timeout) = self.timeout.lock() {
            if timeout.elapsed().as_secs() > 10u64 {
                return true;
            }
        }
        false
    }

    pub fn get_device(&self) -> Arc<Mutex<Box<dyn Device + Send>>> {
        Arc::clone(&self.device)
    }

    pub fn event_loop(&mut self, mut event: Event, ui: Weak<AppWindow>) {
        println!("event_loop {:?}", Handle::try_current());
        let device = Arc::clone(&self.device);
        let event_running = Arc::clone(&self.running);
        let timeout = Arc::clone(&self.timeout);

        self.event_task = Some(tokio::task::spawn_blocking(move || -> () {
            let mut tmp_buf = [0; 1024];
            let mut buf = vec![];
            let mut index = 0usize;
            let mut ping_timer = Instant::now();

            tokio::spawn(async move {
                while event_running.load(Ordering::Relaxed) {
                    if let Ok(mut device) = device.lock() {
                        if ping_timer.elapsed().as_secs() > 3u64 {
                            ping_timer = Instant::now();
                            let ret = ping(&mut device);
                            println!("ping {:?} -> {:?}", device.get_id(), ret);
                        }
                        let _ = device
                            .read(&mut tmp_buf[index..])
                            .or_else(|e| {
                                if let Some(err) = e.downcast_ref::<io::Error>() {
                                    match err.kind() {
                                        io::ErrorKind::TimedOut => (),
                                        _ => {
                                            event_running.store(false, Ordering::Relaxed);
                                        }
                                    }
                                }
                                Err(e)
                            })
                            .map(|size| {
                                buf.append(&mut tmp_buf[..size].to_vec());
                                index += size;

                                loop {
                                    if buf.len() < protocol::HEADER_SIZE {
                                        break;
                                    }
                                    match ProtocolHeader::parse(&buf[..]) {
                                        Err(_) => break,
                                        Ok(header) => {
                                            if buf.len()
                                                < header.get_data_size() as usize
                                                    + protocol::HEADER_SIZE
                                            {
                                                break;
                                            }
                                            match header.get_cmd_code() {
                                                CmdCode::System(SystemCode::Pong) => {
                                                    println!("pong {:?}", device.get_id());
                                                    if let Ok(mut timeout) = timeout.lock() {
                                                        *timeout = Instant::now();
                                                    }
                                                }
                                                _ => {
                                                    event.call(
                                                        header.get_cmd_code(),
                                                        &mut device,
                                                        Some(&buf[protocol::HEADER_SIZE..]),
                                                        &ui,
                                                    );
                                                }
                                            }
                                            let protocol_size = header.get_data_size() as usize
                                                + protocol::HEADER_SIZE;
                                            buf.drain(0..protocol_size);
                                            index -= protocol_size;
                                        }
                                    }
                                }
                            });
                    }
                    tokio::time::sleep(tokio::time::Duration::ZERO).await;
                }
            });
        }));
    }
}
