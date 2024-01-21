use std::collections::HashMap;

use super::devices::device::Device;
use bytes::{BufMut, BytesMut};

pub struct Connector {
    device: Box<dyn Device>,
    buf: BytesMut,
}

impl Connector {
    pub fn new(device: Box<dyn Device>) -> Self {
        Self {
            device,
            buf: BytesMut::with_capacity(4096),
        }
    }

    pub fn event_loop(&mut self) {
        loop {
            let mut r_buf = vec![];
            match self.device.read(&mut r_buf) {
                Err(e) => println!("error: {:?}", e),
                Ok(_) => {
                    self.buf.put(&r_buf[..]);
                }
            }
        }
    }
}
