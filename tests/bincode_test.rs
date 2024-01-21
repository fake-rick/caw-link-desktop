use bincode::{config, Decode, Encode};

#[derive(Debug, Encode, Decode, PartialEq)]
struct VData {
    v0: u8,
    v1: u16,
    v2: u32,
    v3: u64,
}

#[derive(Debug, Encode, Decode, PartialEq)]
enum MainEnum {
    Sub1(Sub1Enum),
    Sub2(Sub2Enum),
}

#[derive(Debug, Encode, Decode, PartialEq)]
enum Sub1Enum {
    V0 = 0,
    V1,
}

#[derive(Debug, Encode, Decode, PartialEq)]
enum Sub2Enum {
    V0 = 0,
    V1,
    V2,
}

#[test]
fn test_bincode() {
    let config = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let vdata = VData {
        v0: 0,
        v1: 1,
        v2: 2,
        v3: 3,
    };
    let encode_v: Vec<u8> = bincode::encode_to_vec(&vdata, config).unwrap();
    let result_v = vec![0u8, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3];
    assert_eq!(encode_v.len(), 15);
    assert_eq!(encode_v, result_v);
    let (decode_v, _): (VData, usize) = bincode::decode_from_slice(&encode_v[..], config).unwrap();
    assert_eq!(decode_v, vdata);

    let edata = MainEnum::Sub2(Sub2Enum::V2);
    let encode_e: Vec<u8> = bincode::encode_to_vec(&edata, config).unwrap();
    let result_e = vec![0u8, 0, 0, 1, 0, 0, 0, 2];
    assert_eq!(encode_e.len(), 8);
    assert_eq!(encode_e, result_e);
    let (decode_e, _): (MainEnum, usize) =
        bincode::decode_from_slice(&encode_e[..], config).unwrap();
    assert_eq!(decode_e, edata);
}
