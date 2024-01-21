use super::device::Device;
use serialport::SerialPortType;
use serialport::{SerialPort, SerialPortInfo};
use std::io::Read;
use std::io::Write;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static DEFAULT_TIMEOUT: u64 = 10;

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

pub struct Serial {
    driver: serialport::COMPort,
}

impl Serial {
    pub fn new(path: &str, baud_rate: u32) -> Result<Self> {
        let mut driver = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(DEFAULT_TIMEOUT))
            .open_native()?;
        Ok(Self { driver })
    }

    /// 搜索特定的串口设备
    ///
    /// 遍历串口设备，发送特定的数据并接口返回数据，通过返回的数据来匹配特定设备
    pub fn search_and_create<F>(baud_rate: u32, w_buf: &[u8], f: F) -> Result<Self>
    where
        F: Fn(&[u8]) -> Result<()>,
    {
        for port in Serial::ports()? {
            println!("{} {:?}", port.port_name, port.port_type);

            // 如果是USB串口则处理，其他串口设备不处理
            match port.port_type {
                SerialPortType::UsbPort(_) => (),
                _ => continue,
            }

            match Serial::new(&port.port_name[..], baud_rate).and_then(|mut serial| {
                println!("start write");
                let mut r_buf = vec![];
                match serial
                    .write(w_buf)
                    .and_then(|_| {
                        println!("write ok");
                        serial.read(&mut r_buf)
                    })
                    .and_then(|size| {
                        println!("read {} bytes", size);
                        f(&r_buf[..])
                    }) {
                    Err(e) => Err(e),
                    Ok(_) => Ok(serial),
                }
            }) {
                Ok(serial) => return Ok(serial),
                Err(e) => return Err(e),
            }
        }

        // 遍历串口设备未找到匹配设备则返回此错误
        Err(SerialError::DeviceNotExist.into())
    }

    pub fn ports() -> Result<Vec<SerialPortInfo>> {
        Ok(serialport::available_ports()?)
    }
}

impl Device for Serial {
    fn read(&mut self, r_buf: &mut Vec<u8>) -> Result<usize> {
        Ok(self.driver.read(r_buf)?)
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
