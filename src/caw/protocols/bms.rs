use bincode::{
    config::{self},
    Decode, Encode,
};

pub const BMS_INFO_SIZE: usize = 44;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct BMSInfo {
    state: u8,
    cell_voltage: [i32; 5],
    balance: [u8; 5],
    voltage: i32,
    current: i32,
    soc: i32,
    soh: i32,
    dsg: u8,
    chg: u8,
}

impl Default for BMSInfo {
    fn default() -> Self {
        Self {
            state: 0,
            cell_voltage: [0, 0, 0, 0, 0],
            balance: [0, 0, 0, 0, 0],
            voltage: 0,
            current: 0,
            soc: 0,
            soh: 0,
            dsg: 0,
            chg: 0,
        }
    }
}

impl BMSInfo {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let config = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let (info, _): (BMSInfo, usize) =
            bincode::decode_from_slice(&buf[..BMS_INFO_SIZE], config)?;
        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bms_info_size_test() {
        let config = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let p = BMSInfo::default();
        let encode: Vec<u8> = bincode::encode_to_vec(&p, config).unwrap();
        assert_eq!(encode.len(), BMS_INFO_SIZE);
    }
}
