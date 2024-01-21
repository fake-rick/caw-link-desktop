pub fn crc8_with_ccitt(crc: u8, data: u8) -> u8 {
    let mut data = crc ^ data;
    for _ in 0..8 {
        if (data & 0x80) != 0 {
            data <<= 1;
            data ^= 0x07;
        } else {
            data <<= 1;
        }
    }
    data
}

/// 计算字节切片的CRC8校验和
pub fn crc8_slice_with_ccitt(buf: &[u8]) -> u8 {
    let mut checksum = 0u8;
    for b in buf {
        checksum = crc8_with_ccitt(checksum, *b);
    }
    return checksum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc8_slice_with_ccitt_test() {
        let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = crc8_slice_with_ccitt(&data[..]);
        assert_eq!(result, 0x85);
    }
}
