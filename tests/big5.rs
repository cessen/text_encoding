extern crate text_encoding;

use text_encoding::big5_whatwg::{decode_to_str, encode_from_str};

const WHATWG_DECODE_BIG5_DATA: &'static [u8] =
    include_bytes!("test_data/whatwg/big5_whatwg_test_decode_in.txt");
const WHATWG_DECODE_UTF8_DATA: &'static [u8] =
    include_bytes!("test_data/whatwg/big5_whatwg_test_decode_out.txt");
const WHATWG_ENCODE_UTF8_DATA: &'static [u8] =
    include_bytes!("test_data/whatwg/big5_whatwg_test_encode_in.txt");
const WHATWG_ENCODE_BIG5_DATA: &'static [u8] =
    include_bytes!("test_data/whatwg/big5_whatwg_test_encode_out.txt");

#[test]
fn big5_whatwg_encode() {
    let mut buf = vec![0u8; WHATWG_ENCODE_BIG5_DATA.len()];
    let (big5, _) = encode_from_str(
        std::str::from_utf8(WHATWG_ENCODE_UTF8_DATA).unwrap(),
        &mut buf,
        true,
    ).unwrap();

    assert_eq!(WHATWG_ENCODE_BIG5_DATA, big5);
}

#[test]
fn big5_whatwg_decode() {
    let mut buf = vec![0u8; WHATWG_DECODE_UTF8_DATA.len()];
    let (utf8, _) = decode_to_str(WHATWG_DECODE_BIG5_DATA, &mut buf, true).unwrap();

    assert_eq!(WHATWG_DECODE_UTF8_DATA, utf8.as_bytes());
}
