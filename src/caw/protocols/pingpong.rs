use bincode::config::{self, BigEndian, Configuration, Fixint};

use super::{
    code::{CmdCode, SystemCode},
    protocol::ProtocolHeader,
};
use crate::caw::devices::device::Device;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Pingpong {}

impl Pingpong {
    pub fn ping(
        device: &mut Box<dyn Device + Send>,
        config: &Configuration<BigEndian, Fixint>,
    ) -> Result<()> {
        let header = ProtocolHeader::default()
            .set_cmd_code(CmdCode::System(SystemCode::Ping))
            .set_data_size(0);
        let header_buf: Vec<u8> = bincode::encode_to_vec(&header, *config)?;
        device.write(&header_buf[..])
    }
}
