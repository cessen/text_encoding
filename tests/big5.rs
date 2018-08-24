extern crate text_encoding;

use text_encoding::{decode_to_str, encode_from_str, Encoding};

const WHATWG_DECODE_BIG5_DATA: &'static [u8] =
    include_bytes!("test_data/big5_whatwg_test_decode_in.txt");
const WHATWG_DECODE_UTF8_DATA: &'static [u8] =
    include_bytes!("test_data/big5_whatwg_test_decode_out.txt");
const WHATWG_ENCODE_UTF8_DATA: &'static [u8] =
    include_bytes!("test_data/big5_whatwg_test_encode_in.txt");
const WHATWG_ENCODE_BIG5_DATA: &'static [u8] =
    include_bytes!("test_data/big5_whatwg_test_encode_out.txt");

#[test]
fn big5_whatwg_encode() {
    let mut buf = vec![0u8; WHATWG_ENCODE_BIG5_DATA.len()];
    let (_, big5) = encode_from_str(
        Encoding::Big5_WHATWG,
        std::str::from_utf8(WHATWG_ENCODE_UTF8_DATA).unwrap(),
        &mut buf,
    ).unwrap();

    assert_eq!(WHATWG_ENCODE_BIG5_DATA, big5);
}

#[test]
fn big5_whatwg_decode() {
    let mut buf = vec![0u8; WHATWG_DECODE_UTF8_DATA.len()];
    let (_, utf8) =
        decode_to_str(Encoding::Big5_WHATWG, WHATWG_DECODE_BIG5_DATA, &mut buf).unwrap();

    assert_eq!(WHATWG_DECODE_UTF8_DATA, utf8.as_bytes());
}
