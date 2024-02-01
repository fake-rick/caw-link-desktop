use std::io::Cursor;

use crate::caw::{
    devices::device::{self, Device},
    utils::crypto::crc8_slice_with_ccitt,
};

use super::code::{CmdCode, OtherCode};
use bincode::{
    config::{self, BigEndian, Configuration, Fixint},
    Decode, Encode,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const MAGIC: [u8; 4] = ['C' as u8, 'A' as u8, 'W' as u8, 'X' as u8];
const VERSION: u16 = 0x101;
pub const HEADER_SIZE: usize = 19;

#[derive(Debug, Clone)]
enum ProtocolError {
    ParseHeaderFailed,
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ProtocolError::ParseHeaderFailed => {
                write!(f, "parse header failed")
            }
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ProtocolError::ParseHeaderFailed => None,
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct ProtocolHeader {
    magic: [u8; 4],
    cmd_code: CmdCode,
    version: u16,
    data_size: u32,
    checksum: u8, // crc8
}

impl Default for ProtocolHeader {
    fn default() -> Self {
        Self {
            magic: MAGIC,
            cmd_code: CmdCode::Other(OtherCode::Unknown),
            version: VERSION,
            data_size: 0,
            checksum: 0,
        }
    }
}

impl ProtocolHeader {
    /// 设置指令
    pub fn set_cmd_code(mut self, code: CmdCode) -> Self {
        self.cmd_code = code;
        self
    }

    /// 设置数据体大小
    pub fn set_data_size(mut self, size: u32) -> Self {
        self.data_size = size;
        self
    }

    /// 设置CRC8
    pub fn set_checksum(mut self, buf: &[u8]) -> Self {
        self.checksum = crc8_slice_with_ccitt(buf);
        self
    }

    pub fn get_cmd_code(&self) -> CmdCode {
        self.cmd_code
    }

    pub fn get_data_size(&self) -> u32 {
        self.data_size
    }
}

impl ProtocolHeader {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        if buf.len() < HEADER_SIZE {
            return Err(ProtocolError::ParseHeaderFailed.into());
        }
        let config = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let (header, _): (ProtocolHeader, usize) =
            bincode::decode_from_slice(&buf[..HEADER_SIZE], config)?;
        Ok(header)
    }

    pub fn write(device: &mut Box<dyn Device + Send>, code: CmdCode, size: u32) -> Result<()> {
        let header = ProtocolHeader::default()
            .set_cmd_code(code)
            .set_data_size(size);
        let config = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let header_buf: Vec<u8> = bincode::encode_to_vec(&header, config)?;
        device.write(&header_buf[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caw::protocols::code::OtherCode;
    use bincode::config;

    #[test]
    fn default_test() {
        let p = ProtocolHeader::default();
        assert_eq!(p.magic, MAGIC);
        assert_eq!(p.cmd_code, CmdCode::Other(OtherCode::Unknown));
        assert_eq!(p.version, VERSION);
        assert_eq!(p.data_size, 0);
        assert_eq!(p.checksum, 0);
    }

    #[test]
    fn header_size_test() {
        let config = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let p = ProtocolHeader::default();
        let encode: Vec<u8> = bincode::encode_to_vec(&p, config).unwrap();
        assert_eq!(encode.len(), HEADER_SIZE);
    }
}
