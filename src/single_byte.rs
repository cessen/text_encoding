//! Single byte encodings.
//!
//! So far all of these are Ascii and extensions of Ascii.

use core;
use {DecodeError, DecodeErrorCause, DecodeResult, EncodeError, EncodeResult};

pub mod ascii {
    //! US ASCII.
    use super::*;

    pub fn encode_from_str<'a>(input: &str, out_buffer: &'a mut [u8]) -> EncodeResult<'a> {
        ascii_ext_encode_from_str(&[], input, out_buffer)
    }

    pub fn decode_to_str<'a>(input: &[u8], out_buffer: &'a mut [u8]) -> DecodeResult<'a> {
        ascii_ext_decode_to_str(&['�'; 128], input, out_buffer)
    }
}

pub mod ibm_866 {
    //! IBM code page 866.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/ibm-cp866_tables.rs.inc");
}

pub mod iso_8859_1 {
    //! ISO/IEC 8859-1, also known as Latin-1.
    //!
    //! This encoding maps 1-to-1 to the first 256 unicode scalar values.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-1_tables.rs.inc");
}

pub mod iso_8859_2 {
    //! ISO/IEC 8859-2, also known as Latin-2.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-2_tables.rs.inc");
}

pub mod iso_8859_3 {
    //! ISO/IEC 8859-3, also known as Latin-3.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-3_tables.rs.inc");
}

pub mod iso_8859_4 {
    //! ISO/IEC 8859-4, also known as Latin-4.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-4_tables.rs.inc");
}

pub mod iso_8859_5 {
    //! ISO/IEC 8859-5.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-5_tables.rs.inc");
}

pub mod iso_8859_6 {
    //! ISO/IEC 8859-6.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-6_tables.rs.inc");
}

pub mod iso_8859_7 {
    //! ISO/IEC 8859-7.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-7_tables.rs.inc");
}

pub mod iso_8859_8 {
    //! ISO/IEC 8859-8.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-8_tables.rs.inc");
}

pub mod iso_8859_9 {
    //! ISO/IEC 8859-9.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-9_tables.rs.inc");
}

pub mod iso_8859_10 {
    //! ISO/IEC 8859-10.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-10_tables.rs.inc");
}

pub mod iso_8859_11 {
    //! ISO/IEC 8859-11, also known as TIS-620.
    //!
    //! This is a common one-byte encoding for the Thai language.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-11_tables.rs.inc");
}

pub mod iso_8859_13 {
    //! ISO/IEC 8859-13.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-13_tables.rs.inc");
}

pub mod iso_8859_14 {
    //! ISO/IEC 8859-14.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-14_tables.rs.inc");
}

pub mod iso_8859_15 {
    //! ISO/IEC 8859-15.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-15_tables.rs.inc");
}

pub mod iso_8859_16 {
    //! ISO/IEC 8859-16.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/iso-8859-16_tables.rs.inc");
}

pub mod koi8_r {
    //! KOI8-R.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/koi8-r_tables.rs.inc");
}

pub mod koi8_u {
    //! KOI8-U.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/koi8-u_tables.rs.inc");
}

pub mod mac_roman {
    //! Mac OS Roman, also known as "macintosh".
    //!
    //! The standard text encoding used on classic Mac OS (i.e. prior to Mac OS X).

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/mac-roman_tables.rs.inc");
}

pub mod mac_cyrillic {
    //! Mac OS Cyrillic, also known as "x-mac-cyrillic".

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/mac-cyrillic_tables.rs.inc");
}

pub mod windows_874 {
    //! Windows code page 874.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp874_tables.rs.inc");
}

pub mod windows_1250 {
    //! Windows code page 1250.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1250_tables.rs.inc");
}

pub mod windows_1251 {
    //! Windows code page 1251.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1251_tables.rs.inc");
}

pub mod windows_1252 {
    //! Windows code page 1252.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1252_tables.rs.inc");
}

pub mod windows_1253 {
    //! Windows code page 1253.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1253_tables.rs.inc");
}

pub mod windows_1254 {
    //! Windows code page 1254.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1254_tables.rs.inc");
}

pub mod windows_1255 {
    //! Windows code page 1255.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1255_tables.rs.inc");
}

pub mod windows_1256 {
    //! Windows code page 1256.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1256_tables.rs.inc");
}

pub mod windows_1257 {
    //! Windows code page 1257.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1257_tables.rs.inc");
}

pub mod windows_1258 {
    //! Windows code page 1258.

    // Generated by:
    // `encoding_tables/single_byte/generate_ascii_ext_tables.py`.
    include!("generated/single_byte/windows-cp1258_tables.rs.inc");
}

/// This is shared among the single byte encoders that are strict extensions
/// of ascii.  It is shallowly wrapped in each of their modules.
#[inline]
fn ascii_ext_encode_from_str<'a>(
    table: &[(char, u8)],
    input: &str,
    output: &'a mut [u8],
) -> EncodeResult<'a> {
    // Do the encode.
    let mut input_i = 0;
    let mut output_i = 0;
    for (offset, c) in input.char_indices() {
        if output_i >= output.len() {
            break;
        }
        if c as u32 <= 127 {
            output[output_i] = c as u8;
            output_i += 1;
            input_i = offset + 1;
        } else if let Ok(i) = table.binary_search_by_key(&c, |x| x.0) {
            output[output_i] = table[i].1;
            output_i += 1;
            input_i = offset + 1;
        } else {
            return Err(EncodeError {
                character: c,
                error_range: (offset, offset + c.len_utf8()),
                output_bytes_written: output_i,
            });
        }
    }

    // Calculate how much of the input was consumed.
    if input_i > input.len() {
        input_i = input.len();
    } else {
        while !input.is_char_boundary(input_i) {
            input_i += 1;
        }
    }

    Ok((&output[..output_i], input_i))
}

/// This is shared among the single byte decoders that are strict extensions
/// of ascii.  It is shallowly wrapped in each of their modules.
#[inline]
fn ascii_ext_decode_to_str<'a>(
    table: &[char; 128],
    input: &[u8],
    output: &'a mut [u8],
) -> DecodeResult<'a> {
    let mut input_i = 0;
    let mut output_i = 0;
    for &byte in input.iter() {
        if byte < 0x80 {
            // 1-byte case
            if output_i >= output.len() {
                break;
            }
            output[output_i] = byte;
            input_i += 1;
            output_i += 1;
        } else {
            // Use lookup table.
            let code = table[byte as usize - 0x80];
            if code == '�' {
                // Error: undefined byte.
                return Err(DecodeError {
                    cause: DecodeErrorCause::InvalidData,
                    error_range: (input_i, input_i + 1),
                    output_bytes_written: output_i,
                });
            }
            // Encode to utf8
            let mut buf = [0u8; 4];
            let s = code.encode_utf8(&mut buf);
            if (output_i + s.len()) > output.len() {
                break;
            }
            output[output_i..(output_i + s.len())].copy_from_slice(s.as_bytes());
            input_i += 1;
            output_i += s.len();
        }
    }

    Ok((
        unsafe { core::str::from_utf8_unchecked(&output[..output_i]) },
        input_i,
    ))
}

//===========================================================================

// Most of the testing below is done with iso-8859-7, since it has a few
// undefined characters, allowing us to test handling of those.
#[cfg(test)]
mod tests {
    use super::{iso_8859_7::*, *};
    use {DecodeError, EncodeError};

    #[test]
    fn encode_01() {
        let text = "Hello world!";
        let mut buf = [0u8; 0];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn encode_02() {
        let text = "Hello world!";
        let mut buf = [0u8; 1];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 1);
        assert_eq!(encoded, "H".as_bytes());
    }

    #[test]
    fn encode_03() {
        let text = "Hello world!";
        let mut buf = [0u8; 2];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(encoded, "He".as_bytes());
    }

    #[test]
    fn encode_04() {
        let text = "Hello world!";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 12);
        assert_eq!(encoded, "Hello world!".as_bytes());
    }

    #[test]
    fn encode_05() {
        let text = "Hello world!こ";
        let mut buf = [0u8; 12];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 12);
        assert_eq!(encoded, "Hello world!".as_bytes());
    }

    #[test]
    fn decode_01() {
        let data = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!"
        let mut buf = [0u8; 0];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(decoded, "");
    }

    #[test]
    fn decode_02() {
        let data = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!"
        let mut buf = [0u8; 1];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf).unwrap();
        assert_eq!(consumed_count, 1);
        assert_eq!(decoded, "H");
    }

    #[test]
    fn decode_03() {
        let data = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!"
        let mut buf = [0u8; 2];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "He");
    }

    #[test]
    fn decode_04() {
        let data = [
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!"
        let mut buf = [0u8; 64];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf).unwrap();
        assert_eq!(consumed_count, 12);
        assert_eq!(decoded, "Hello world!");
    }

    #[test]
    fn decode_05() {
        let data = [
            0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE,
            0xCF, 0xD0, 0xD1, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9,
        ]; // "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"
        let mut buf = [0u8; 128];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf).unwrap();
        assert_eq!(consumed_count, 24);
        assert_eq!(decoded, "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ");
    }

    #[test]
    fn encode_error_01() {
        let text = "こello world!";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: 'こ',
                error_range: (0, 3),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn encode_error_02() {
        let text = "\u{00C0}ello world!";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: '\u{00C0}',
                error_range: (0, 2),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn encode_error_03() {
        let text = "Hこllo world!";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: 'こ',
                error_range: (1, 4),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn encode_error_04() {
        let text = "H\u{00C0}llo world!";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: '\u{00C0}',
                error_range: (1, 3),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn encode_error_05() {
        let text = "Heこlo world!";
        let mut buf = [0u8; 3];
        assert_eq!(
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: 'こ',
                error_range: (2, 5),
                output_bytes_written: 2,
            })
        );
    }

    #[test]
    fn encode_error_06() {
        let text = "He\u{00C0}lo world!";
        let mut buf = [0u8; 3];
        assert_eq!(
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: '\u{00C0}',
                error_range: (2, 4),
                output_bytes_written: 2,
            })
        );
    }

    #[test]
    fn decode_error_01() {
        let data = [
            0x48, 0xAE, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!" with an error on the second byte (undefined byte).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (1, 2),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn decode_error_02() {
        let data = [
            0x48, 0xD2, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!" with an error on the second byte (undefined byte).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (1, 2),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn decode_error_03() {
        let data = [
            0x48, 0xFF, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!" with an error on the second byte (undefined byte).
        let mut buf = [0u8; 64];
        let error = decode_to_str(&data, &mut buf);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (1, 2),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn ascii_encode_error_01() {
        let text = "\u{0080}ello world!";
        let mut buf = [0u8; 64];
        assert_eq!(
            ascii::encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: '\u{0080}',
                error_range: (0, 2),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn ascii_encode_error_02() {
        let text = "\u{00FF}ello world!";
        let mut buf = [0u8; 64];
        assert_eq!(
            ascii::encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: '\u{00FF}',
                error_range: (0, 2),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn ascii_decode_error_01() {
        let data = [
            0x48, 0x80, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!" with an error on the second byte (undefined byte).
        let mut buf = [0u8; 64];
        let error = ascii::decode_to_str(&data, &mut buf);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (1, 2),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn ascii_decode_error_02() {
        let data = [
            0x48, 0xFF, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21,
        ]; // "Hello world!" with an error on the second byte (undefined byte).
        let mut buf = [0u8; 64];
        let error = ascii::decode_to_str(&data, &mut buf);
        assert_eq!(
            error,
            Err(DecodeError {
                cause: DecodeErrorCause::InvalidData,
                error_range: (1, 2),
                output_bytes_written: 1,
            })
        );
    }

    #[test]
    fn encode_latin1_01() {
        let mut buf = [0u8; 64];
        // Make sure all the allowed code points encode correctly.
        for i in 0..=255u8 {
            let (encoded, consumed_count) =
                iso_8859_1::encode_from_str((i as char).encode_utf8(&mut [0u8; 4]), &mut buf)
                    .unwrap();
            assert_eq!(consumed_count, (i as char).len_utf8());
            assert_eq!(encoded, &[i]);
        }
    }

    #[test]
    fn decode_latin1_01() {
        let mut buf = [0u8; 64];
        // Make sure all the allowed code points encode correctly.
        for i in 0..=255u8 {
            let (encoded, consumed_count) = iso_8859_1::decode_to_str(&[i], &mut buf).unwrap();
            assert_eq!(consumed_count, 1);
            assert_eq!(encoded, (i as char).encode_utf8(&mut [0u8; 4]));
        }
    }
}
