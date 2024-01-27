use std::fmt::Debug;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Device {
    fn get_id(&self) -> (u32, u32);
    fn write(&mut self, w_buf: &[u8]) -> Result<()>;
    fn read(&mut self, r_buf: &mut [u8]) -> Result<()>;
}

impl Debug for dyn Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
