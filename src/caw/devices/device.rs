type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Device {
    fn write(&mut self, w_buf: &[u8]) -> Result<()>;
    fn read(&mut self, r_buf: &mut [u8]) -> Result<()>;
}
