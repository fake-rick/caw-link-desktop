use crate::ui::*;

use bincode::{
    config::{self},
    Decode, Encode,
};
use slint::*;

use crate::caw::devices::device::Device;

pub const BMS_INFO_SIZE: usize = 48;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct BMSInfo {
    state: u8,
    cell_voltage: [i32; 5],
    balance: [u8; 5],
    voltage: i32,
    current: i32,
    temperature: i32,
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
            temperature: 0,
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

pub fn bms_info_protocol(
    device: &mut Box<dyn Device + Send>,
    buf: Option<&[u8]>,
    ui: &Weak<AppWindow>,
) {
    if let Some(buf) = buf {
        if let Ok(bms_info) = BMSInfo::parse(buf) {
            let _ = ui.upgrade_in_event_loop(move |handle| {
                let service = handle.global::<BMSModelService>();
                let balance: Vec<i32> = bms_info.balance.iter().map(|&x| x as i32).collect();
                let cell_voltage: Vec<f32> = bms_info
                    .cell_voltage
                    .iter()
                    .map(|&x| x as f32 / 100.0)
                    .collect();
                service.set_bms_info(BMSInfoModel {
                    balance: VecModel::from_slice(balance.as_slice()),
                    cell_voltage: VecModel::from_slice(cell_voltage.as_slice()),
                    chg: bms_info.chg != 0,
                    current: bms_info.current as f32 / 100.0,
                    dsg: bms_info.dsg != 0,
                    soc: bms_info.soc as f32 / 100.0,
                    soh: bms_info.soh as f32 / 100.0,
                    state: VecModel::from_slice([0, 0, 0, 0, 0, 0, 0, 0].as_slice()),
                    temperature: bms_info.temperature as f32 / 100.0,
                    voltage: bms_info.voltage as f32 / 100.0,
                });
            });
        }

        // println!("{:?}", bms_info);
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
