use super::devices::device::Device;

pub struct Connector {
    device: Box<dyn Device>,
}

impl Connector {
    pub fn new(device: Box<dyn Device>) -> Self {
        Self { device }
    }

    pub fn event_loop(&mut self) {
        loop {}
    }
}
