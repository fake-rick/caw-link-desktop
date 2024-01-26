use std::io::Cursor;

use super::code::{CmdCode, OtherCode};
use bincode::{Decode, Encode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static MAGIC: [u8; 4] = ['C' as u8, 'A' as u8, 'W' as u8, 'X' as u8];
static VERSION: u16 = 0x101;
static HEADER_SIZE: usize = 19;

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
    pub fn set_cmd_code(&mut self, code: CmdCode) {
        self.cmd_code = code;
    }

    /// 设置数据体大小
    pub fn set_data_size(&mut self, size: u32) {
        self.data_size = size;
    }
}

impl ProtocolHeader {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        if buf.len() <= HEADER_SIZE {
            return Err(ProtocolError::ParseHeaderFailed.into());
        }

        return Err(ProtocolError::ParseHeaderFailed.into());
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
