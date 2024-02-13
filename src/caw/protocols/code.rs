use bincode::{Decode, Encode};

#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum CmdCode {
    Other(OtherCode),
    System(SystemCode),
    BMS(BMSCode),
    Motor(MotorCode),
}

/// 其他指令
#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum OtherCode {
    Unknown = 0,
}

/// 系统指令
#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum SystemCode {
    Ping = 0,
    Pong,
    Log,
}

/// 电源管理系统指令
#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum BMSCode {
    Info = 0,
}

/// 电机指令
#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum MotorCode {}
