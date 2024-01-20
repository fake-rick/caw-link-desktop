/// MainCode 主码
const MainCodeSystem: u32 = 0;
const MainCodeBMS: u32 = 1;
const MainCodeMotor: u32 = 2;

/// System 扩展码// 发现设备[w]
const DiscoverDevice: u32 = 0;
// 设备信息[R]
const DeviceInfo: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;
}
