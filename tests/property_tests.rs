#[macro_use]
extern crate proptest;
extern crate text_encoding;

use proptest::collection::vec;
use proptest::test_runner::Config;
use text_encoding::{single_byte::*, *};

proptest! {
    #![proptest_config(Config::with_cases(512))]

    #[test]
    fn pt_utf8_roundtrip(ref text in "\\PC*\\PC*\\PC*") {
        let mut buf = [0u8; 32];
        let mut utf8_encoded: Vec<u8> = Vec::new();
        let mut utf8 = String::new();

        // Encode to utf8
        let mut tmp = &text[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = utf8::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8_encoded.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        // Decode back from utf8
        let mut tmp = &utf8_encoded[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = utf8::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        assert_eq!(&text[..], &utf8[..]);
        assert_eq!(text.as_bytes(), &utf8_encoded[..]);
        assert_eq!(utf8.as_bytes(), &utf8_encoded[..]);
    }

    #[test]
    fn pt_utf8_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = utf8::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_utf8_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode.  Should always succeed, but the important thing
        // is that it should never panic.
        let mut buf = vec![0u8; 4096];
        let _ = utf8::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_utf16be_roundtrip(ref text in "\\PC*\\PC*\\PC*") {
        let mut buf = [0u8; 32];
        let mut utf16: Vec<u8> = Vec::new();
        let mut utf8 = String::new();

        // Encode to utf16 big endian
        let mut tmp = &text[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = utf16_be::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf16.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        // Decode back from utf16 big endian
        let mut tmp = &utf16[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = utf16_be::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        assert_eq!(&text[..], &utf8[..]);
    }

    #[test]
    fn pt_utf16be_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = utf16_be::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_utf16be_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode.  Should always succeed, but the important thing
        // is that it should never panic.
        let mut buf = vec![0u8; 4096];
        let _ = utf16_be::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_utf16le_roundtrip(ref text in "\\PC*\\PC*\\PC*") {
        let mut buf = [0u8; 32];
        let mut utf16: Vec<u8> = Vec::new();
        let mut utf8 = String::new();

        // Encode to utf16 little endian
        let mut tmp = &text[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = utf16_le::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf16.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        // Decode back from utf16 big endian
        let mut tmp = &utf16[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = utf16_le::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        assert_eq!(&text[..], &utf8[..]);
    }

    #[test]
    fn pt_utf16le_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = utf16_le::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_utf16le_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode.  Should always succeed, but the important thing
        // is that it should never panic.
        let mut buf = vec![0u8; 4096];
        let _ = utf16_le::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_utf32be_roundtrip(ref text in "\\PC*\\PC*\\PC*") {
        let mut buf = [0u8; 32];
        let mut utf32: Vec<u8> = Vec::new();
        let mut utf8 = String::new();

        // Encode to utf32 big endian
        let mut tmp = &text[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = utf32_be::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf32.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        // Decode back from utf32 big endian
        let mut tmp = &utf32[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = utf32_be::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        assert_eq!(&text[..], &utf8[..]);
    }

    #[test]
    fn pt_utf32be_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = utf32_be::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_utf32be_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode.  Should always succeed, but the important thing
        // is that it should never panic.
        let mut buf = vec![0u8; 4096];
        let _ = utf32_be::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_utf32le_roundtrip(ref text in "\\PC*\\PC*\\PC*") {
        let mut buf = [0u8; 32];
        let mut utf32: Vec<u8> = Vec::new();
        let mut utf8 = String::new();

        // Encode to utf32 little endian
        let mut tmp = &text[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = utf32_le::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf32.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        // Decode back from utf32 little endian
        let mut tmp = &utf32[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = utf32_le::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        assert_eq!(&text[..], &utf8[..]);
    }

    #[test]
    fn pt_utf32le_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = utf32_le::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_utf32le_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode.  Should always succeed, but the important thing
        // is that it should never panic.
        let mut buf = vec![0u8; 4096];
        let _ = utf32_le::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_ascii_roundtrip(ref data in vec(0u8..=127, 0..1000)) {
        let mut buf = [0u8; 32];
        let mut utf8 = String::new();
        let mut ascii: Vec<u8> = Vec::new();

        // Decode from ascii to utf8
        let mut tmp = &data[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = ascii::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        // Encode to from utf8 back to ascii
        let mut tmp = &utf8[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = ascii::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                ascii.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        assert_eq!(&data[..], &ascii[..]);
    }

    #[test]
    fn pt_ascii_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = ascii::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_ascii_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = ascii::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_latin1_roundtrip(ref data in vec(0u8..=255, 0..1000)) {
        let mut buf = [0u8; 32];
        let mut utf8 = String::new();
        let mut latin1: Vec<u8> = Vec::new();

        // Decode from latin1 to utf8
        let mut tmp = &data[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = iso_8859_1::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        // Encode to from utf8 back to latin1
        let mut tmp = &utf8[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = iso_8859_1::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                latin1.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        assert_eq!(&data[..], &latin1[..]);
    }

    #[test]
    fn pt_latin1_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode.  Should always succeed with latin1, but the
        // important thing here is that it doesn't panic.
        let mut buf = vec![0u8; 4096];
        let _ = iso_8859_1::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_latin1_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = iso_8859_1::encode_from_str(text, &mut buf);
    }

    // The iso-8859-7 tests are representative of all single-byte encodings
    // (except latin1) since they're all generated and share their code.
    #[test]
    fn pt_iso_8859_7_roundtrip(mut data in vec(0u8..=255, 0..1000)) {
        let mut buf = [0u8; 32];
        let mut utf8 = String::new();
        let mut iso8859_7: Vec<u8> = Vec::new();

        // Eliminate undefined bytes in input.
        for b in data.iter_mut() {
            if *b == 0xAE || *b == 0xD2 || *b == 0xFF {
                *b = 0;
            }
        }

        // Decode from iso-8859-7 to utf8
        let mut tmp = &data[..];
        while !tmp.is_empty() {
            if let Ok((decoded, n)) = iso_8859_7::decode_to_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                utf8.extend(decoded.chars());
            } else {
                panic!("Error when decoding.");
            }
        }

        // Encode to from utf8 back to iso-8859-7
        let mut tmp = &utf8[..];
        while !tmp.is_empty() {
            if let Ok((encoded, n)) = iso_8859_7::encode_from_str(tmp, &mut buf) {
                tmp = &tmp[n..];
                iso8859_7.extend_from_slice(encoded);
            } else {
                panic!("Error when encoding.");
            }
        }

        assert_eq!(&data[..], &iso8859_7[..]);
    }

    #[test]
    fn pt_iso_8859_7_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = iso_8859_7::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_iso_8859_7_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = iso_8859_7::encode_from_str(text, &mut buf);
    }

    #[test]
    fn pt_big5_whatwg_decode_random_bytes(ref data in vec(0u8..=255, 0..512)) {
        // Attempt to decode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = big5_whatwg::decode_to_str(data, &mut buf);
    }

    #[test]
    fn pt_big5_whatwg_encode_random_text(ref text in "\\PC*\\PC*\\PC*") {
        // Attempt to encode, but probably fail.  The important thing is that
        // it should never panic, only return errors.
        let mut buf = vec![0u8; 4096];
        let _ = big5_whatwg::encode_from_str(text, &mut buf);
    }
}
