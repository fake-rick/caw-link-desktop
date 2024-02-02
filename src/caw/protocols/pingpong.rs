use bincode::config::{self, BigEndian, Configuration, Fixint};

use super::{
    code::{CmdCode, SystemCode},
    protocol::ProtocolHeader,
};
use crate::caw::devices::device::Device;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn ping(device: &mut Box<dyn Device + Send>) -> Result<()> {
    ProtocolHeader::write(device, CmdCode::System(SystemCode::Ping), 0)
}

pub fn pong(device: &mut Box<dyn Device + Send>, _: Option<&[u8]>) {
    println!("pong pong pong!!!!!!");
}
