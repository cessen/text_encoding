// extern crate text_encoding;

// use text_encoding::{decode_to_str, encode_from_str, Encoding};

// const BIG5_DATA: &'static [u8] = include_bytes!("test_data/big5_in.txt");
// const UTF8_DATA: &'static [u8] = include_bytes!("test_data/big5_out.txt");

// #[test]
// fn big5_encode() {
//     let mut buf = vec![0u8; BIG5_DATA.len()];
//     let (_, big5) = encode_from_str(
//         Encoding::Big5_WHATWG,
//         std::str::from_utf8(UTF8_DATA).unwrap(),
//         &mut buf,
//     ).unwrap();

//     assert_eq!(BIG5_DATA, big5);
// }

// #[test]
// fn big5_decode() {
//     let mut buf = vec![0u8; UTF8_DATA.len()];
//     let (_, utf8) = decode_to_str(Encoding::Big5_WHATWG, BIG5_DATA, &mut buf).unwrap();

//     assert_eq!(UTF8_DATA, utf8.as_bytes());
// }
