use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum CmdCode {
    Other(OtherCode),
    System(SystemCode),
    BMS(BMSCode),
    Motor(MotorCode),
}

/// 其他指令
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum OtherCode {
    Unknown = 0,
}

/// 系统指令
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum SystemCode {}

/// 电源管理系统指令
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum BMSCode {}

/// 电机指令
#[derive(Encode, Decode, PartialEq, Debug)]
pub enum MotorCode {}
