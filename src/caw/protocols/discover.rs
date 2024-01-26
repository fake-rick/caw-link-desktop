use bincode::{config, Decode, Encode};

pub const DISCOVER_MAGIC: [u8; 4] = [0xff, 0xff, 0xff, 0x00];
pub const DEVICE_MAGIC: [u8; 4] = [0xff, 0xff, 0xff, 0x01];

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
enum TypeId {
    BMS = 0,
    Motor,
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Discover {
    device_id: u32,
    type_id: TypeId,
}

impl Discover {
    pub fn check_device_magic(buf: &[u8]) -> Result<()> {
        // println!("{:?}", buf);
        if buf != DEVICE_MAGIC {
            return Err(DiscoverError::ParseMagicFailed.into());
        }
        Ok(())
    }

    pub fn parse(buf: &[u8]) -> Result<Self> {
        println!("{:?}", buf);
        let config = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let (v, _): (Discover, usize) = bincode::decode_from_slice(buf, config)?;
        Ok(v)
    }

    pub fn get_id(&self) -> (u32, u32) {
        (self.device_id, self.type_id as u32)
    }
}

#[derive(Debug, Clone)]
pub enum DiscoverError {
    ParseMagicFailed,
}

impl std::fmt::Display for DiscoverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DiscoverError::ParseMagicFailed => {
                write!(f, "parse magic failed")
            }
        }
    }
}

impl std::error::Error for DiscoverError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DiscoverError::ParseMagicFailed => None,
        }
    }
}

#[cfg(test)]
mod tests {}
