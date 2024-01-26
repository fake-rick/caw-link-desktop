use super::device::Device;
use serialport::SerialPortType;
use serialport::{SerialPort, SerialPortInfo};
use std::io::Read;
use std::io::Write;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static DEFAULT_TIMEOUT: u64 = 1000;

#[derive(Debug, Clone)]
enum SerialError {
    DeviceNotExist,
    Unknown,
}

impl std::fmt::Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SerialError::DeviceNotExist => {
                write!(f, "device not exist")
            }
            SerialError::Unknown => {
                write!(f, "unknown")
            }
        }
    }
}

impl std::error::Error for SerialError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SerialError::DeviceNotExist => None,
            SerialError::Unknown => None,
        }
    }
}

#[derive(Debug)]
pub struct Serial {
    device_id: u32,
    type_id: u32,
    driver: Box<dyn SerialPort>,
}

impl Serial {
    pub fn new(path: &str, baud_rate: u32) -> Result<Self> {
        let driver = serialport::new(path, baud_rate)
            // .timeout(Duration::from_millis(DEFAULT_TIMEOUT))
            .open()?;
        Ok(Self {
            device_id: 0,
            type_id: 0,
            driver,
        })
    }

    /// 搜索特定的串口设备
    ///
    /// 遍历串口设备，发送特定的数据并接口返回数据，通过返回的数据来匹配特定设备
    pub fn search<F>(baud_rate: u32, w_buf: &[u8], f: F)
    where
        F: Fn(Self, &[u8]) -> Result<()>,
    {
        for port in Serial::ports().unwrap() {
            // 如果是USB串口则处理，其他串口设备不处理
            match port.port_type {
                SerialPortType::UsbPort(_) => (),
                _ => continue,
            }
            // println!("{} {:?}", port.port_name, port.port_type);
            let _ = Serial::new(&port.port_name[..], baud_rate).and_then(|mut serial| {
                let mut r_buf = [0u8; 12];
                serial
                    .write(w_buf)
                    .and_then(|_| serial.read(&mut r_buf[..]))
                    .and_then(|_| f(serial, &r_buf[..]))
            });
        }
    }

    pub fn ports() -> Result<Vec<SerialPortInfo>> {
        Ok(serialport::available_ports()?)
    }
}

impl Device for Serial {
    fn read(&mut self, r_buf: &mut [u8]) -> Result<()> {
        Ok(self.driver.read_exact(r_buf)?)
    }

    fn write(&mut self, w_buf: &[u8]) -> Result<()> {
        Ok(self.driver.write_all(w_buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_test() {
        Serial::ports().unwrap();
    }
}
