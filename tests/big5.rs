extern crate text_encoding;

use text_encoding::{decode_to_str, encode_from_str, Encoding};

const DECODE_BIG5_DATA: &'static [u8] = include_bytes!("test_data/big5_whatwg_test_decode_1.txt");
const DECODE_UTF8_DATA: &'static [u8] = include_bytes!("test_data/big5_whatwg_test_decode_2.txt");
const ENCODE_UTF8_DATA: &'static [u8] = include_bytes!("test_data/big5_whatwg_test_encode_1.txt");
const ENCODE_BIG5_DATA: &'static [u8] = include_bytes!("test_data/big5_whatwg_test_encode_2.txt");

#[test]
fn big5_encode() {
    let mut buf = vec![0u8; ENCODE_BIG5_DATA.len()];
    let (_, big5) = encode_from_str(
        Encoding::Big5_WHATWG,
        std::str::from_utf8(ENCODE_UTF8_DATA).unwrap(),
        &mut buf,
    ).unwrap();

    assert_eq!(ENCODE_BIG5_DATA, big5);
}

#[test]
fn big5_decode() {
    let mut buf = vec![0u8; DECODE_UTF8_DATA.len()];
    let (_, utf8) = decode_to_str(Encoding::Big5_WHATWG, DECODE_BIG5_DATA, &mut buf).unwrap();

    assert_eq!(DECODE_UTF8_DATA, utf8.as_bytes());
}
