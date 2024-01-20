use crate::caw::devices::device::Device;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait WriteProtocol {
    fn write<T: Device>(&self, device: T) -> Result<usize>;
}

pub trait ReadProtocol {
    fn read<T: Device>(&self, device: T) -> Result<usize>;
}

pub struct Protocol {
    device: Box<dyn Device>,
}

impl Protocol {
    pub fn new(device: Box<dyn Device>) -> Self {
        Self { device }
    }
}
