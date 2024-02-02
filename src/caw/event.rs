use std::collections::HashMap;

use super::{devices::device::Device, protocols::code::CmdCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

type EventCallback = fn(&mut Box<dyn Device + Send>, Option<&[u8]>);

pub struct Event {
    cbs: HashMap<CmdCode, EventCallback>,
}

impl Event {
    pub fn new() -> Self {
        Event {
            cbs: HashMap::new(),
        }
    }

    pub fn register(mut self, cmd: CmdCode, cb: EventCallback) -> Self {
        self.cbs.insert(cmd, cb);
        self
    }

    pub fn call(&mut self, cmd: CmdCode, device: &mut Box<dyn Device + Send>, buf: Option<&[u8]>) {
        if let Some(cb) = self.cbs.get(&cmd) {
            cb(device, buf);
        }
    }
}
