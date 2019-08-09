//! The WHATWG variant of BIG5.

use core;
use {DecodeError, DecodeResult, EncodeError, EncodeResult};

// Generated by build.rs  Contains ENCODE_TABLE and DECODE_TABLE.
include!("generated/whatwg/big5_whatwg_tables.rs.inc");

pub fn encode_from_str<'a>(
    input: &str,
    out_buffer: &'a mut [u8],
    is_end: bool,
) -> EncodeResult<'a> {
    // Do the encode.
    let mut input_i = 0;
    let mut output_i = 0;
    let mut itr = input.char_indices();
    while let Some((offset, c)) = itr.next() {
        let mut code = c as u32;
        if output_i >= out_buffer.len() {
            break;
        } else if code <= 127 {
            // Ascii
            out_buffer[output_i] = code as u8;
            output_i += 1;
            input_i = offset + 1;
        } else if let Ok(ptr_i) = ENCODE_TABLE.binary_search_by_key(&c, |x| x.0) {
            if (output_i + 1) < out_buffer.len() {
                // Handle graphemes.
                if code == 0xCA || code == 0xEA {
                    if let Some((offset2, c2)) = itr.clone().next() {
                        let code2 = c2 as u32;
                        if let Some(bytes) = map_grapheme(code, code2) {
                            itr.next();
                            out_buffer[output_i] = bytes[0];
                            out_buffer[output_i + 1] = bytes[1];
                            output_i += 2;
                            input_i = offset2 + 1;
                            continue;
                        }
                    } else if !is_end {
                        break;
                    }
                }

                // Common case, fetch from table.
                let big5_bytes = ENCODE_TABLE[ptr_i].1;
                out_buffer[output_i] = big5_bytes[0];
                out_buffer[output_i + 1] = big5_bytes[1];
                output_i += 2;
                input_i = offset + 1;
            } else {
                break;
            }
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

    Ok((&out_buffer[..output_i], input_i))
}

fn map_grapheme(a: u32, b: u32) -> Option<[u8; 2]> {
    match (a, b) {
        (0xCA, 0x304) => Some([0x88, 0x62]),
        (0xCA, 0x30C) => Some([0x88, 0x64]),
        (0xEA, 0x304) => Some([0x88, 0xA3]),
        (0xEA, 0x30C) => Some([0x88, 0xA5]),
        _ => None,
    }
}

pub fn decode_to_str<'a>(input: &[u8], out_buffer: &'a mut [u8], is_end: bool) -> DecodeResult<'a> {
    let mut input_i = 0;
    let mut output_i = 0;
    let mut buf = [0u8; 4]; // For encoding utf8 codepoints.

    // Loop through the input, getting a byte at a time.
    let mut itr = input.iter();
    while let Some(&byte_1) = itr.next() {
        if output_i >= out_buffer.len() {
            break;
        } else if byte_1 <= 127 {
            // Ascii
            out_buffer[output_i] = byte_1;
            output_i += 1;
            input_i += 1;
        } else if byte_1 == 0x80 || byte_1 == 0xFF {
            // Error: invalid leading byte.
            return Err(DecodeError {
                error_range: (input_i, input_i + 1),
                output_bytes_written: output_i,
            });
        } else if let Some(&byte_2) = itr.next() {
            if byte_2 < 0x40 || byte_2 > 0xFE || (byte_2 > 0x7E && byte_2 < 0xA1) {
                // Error: invalid trailing byte.
                // WHATWG dictates that if the second byte is ascii, it
                // remains part of the stream, and thus (in our case) is
                // not treated as part of the error.
                return Err(DecodeError {
                    error_range: (input_i, input_i + if byte_2 <= 127 { 1 } else { 2 }),
                    output_bytes_written: output_i,
                });
            }
            let big5_ptr = {
                let lead = (byte_1 as usize - 0x81) * 157;
                let trail = if byte_2 < 0x7f {
                    byte_2 as usize - 0x40
                } else {
                    byte_2 as usize - 0x62
                };
                lead + trail
            };

            // Get our decoded data, either from the table or by special handling.
            let string = if big5_ptr >= DECODE_TABLE.len() || DECODE_TABLE[big5_ptr] == '�' {
                match big5_ptr {
                    // Special handling for codes that map to graphemes.
                    1133 => "\u{00CA}\u{0304}",
                    1135 => "\u{00CA}\u{030C}",
                    1164 => "\u{00EA}\u{0304}",
                    1166 => "\u{00EA}\u{030C}",
                    _ => {
                        // Error: correctly formed but undefined code.
                        // WHATWG dictates that if the second byte is ascii, it
                        // remains part of the stream, and thus (in our case) is
                        // not treated as part of the error.
                        return Err(DecodeError {
                            error_range: (input_i, input_i + if byte_2 <= 127 { 1 } else { 2 }),
                            output_bytes_written: output_i,
                        });
                    }
                }
            } else {
                // Encode codepoint to utf8.
                DECODE_TABLE[big5_ptr].encode_utf8(&mut buf)
            };

            // Copy decoded data to output.
            if (output_i + string.len()) > out_buffer.len() {
                break;
            }
            out_buffer[output_i..(output_i + string.len())].copy_from_slice(string.as_bytes());

            // Update our counters.
            input_i += 2;
            output_i += string.len();
        } else {
            // No available trailing byte.  If not end-of-input, no problem.
            // Otherwise it's an error.
            if !is_end {
                break;
            } else {
                return Err(DecodeError {
                    error_range: (input_i, input_i + 1),
                    output_bytes_written: output_i,
                });
            }
        }
    }

    Ok((
        unsafe { core::str::from_utf8_unchecked(&out_buffer[..output_i]) },
        input_i,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function.
    fn correct_decode(input: &[u8], expected_output: &str) {
        let mut buf = [0u8; 256];
        assert_eq!(
            decode_to_str(input, &mut buf, true),
            Ok((expected_output, input.len())),
        );
    }

    // Helper function.
    fn error_decode(input: &[u8], error_range: (usize, usize), bytes_written: usize) {
        let mut buf = [0u8; 256];
        assert_eq!(
            decode_to_str(input, &mut buf, true),
            Err(DecodeError {
                error_range: error_range,
                output_bytes_written: bytes_written,
            }),
        );
    }

    // Adapted from tests in encoding_rs:
    // https://crates.io/crates/encoding_rs
    #[test]
    fn decode_01() {
        // ASCII
        correct_decode(&[0x61u8, 0x62u8], "\u{0061}\u{0062}");

        // Edge cases
        correct_decode(&[0x87u8, 0x40u8], "\u{43F0}");
        correct_decode(&[0xFEu8, 0xFEu8], "\u{79D4}");
        correct_decode(&[0xFEu8, 0xFDu8], "\u{2910D}");
        correct_decode(&[0x88u8, 0x62u8], "\u{00CA}\u{0304}");
        correct_decode(&[0x88u8, 0x64u8], "\u{00CA}\u{030C}");
        correct_decode(&[0x88u8, 0x66u8], "\u{00CA}");
        correct_decode(&[0x88u8, 0xA3u8], "\u{00EA}\u{0304}");
        correct_decode(&[0x88u8, 0xA5u8], "\u{00EA}\u{030C}");
        correct_decode(&[0x88u8, 0xA7u8], "\u{00EA}");
        correct_decode(&[0x99u8, 0xD4u8], "\u{8991}");
        correct_decode(&[0x99u8, 0xD5u8], "\u{27967}");
        correct_decode(&[0x99u8, 0xD6u8], "\u{8A29}");

        // Edge cases surrounded with ASCII
        correct_decode(
            &[0x61u8, 0x87u8, 0x40u8, 0x62u8],
            "\u{0061}\u{43F0}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0xFEu8, 0xFEu8, 0x62u8],
            "\u{0061}\u{79D4}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0xFEu8, 0xFDu8, 0x62u8],
            "\u{0061}\u{2910D}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x88u8, 0x62u8, 0x62u8],
            "\u{0061}\u{00CA}\u{0304}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x88u8, 0x64u8, 0x62u8],
            "\u{0061}\u{00CA}\u{030C}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x88u8, 0x66u8, 0x62u8],
            "\u{0061}\u{00CA}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x88u8, 0xA3u8, 0x62u8],
            "\u{0061}\u{00EA}\u{0304}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x88u8, 0xA5u8, 0x62u8],
            "\u{0061}\u{00EA}\u{030C}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x88u8, 0xA7u8, 0x62u8],
            "\u{0061}\u{00EA}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x99u8, 0xD4u8, 0x62u8],
            "\u{0061}\u{8991}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x99u8, 0xD5u8, 0x62u8],
            "\u{0061}\u{27967}\u{0062}",
        );
        correct_decode(
            &[0x61u8, 0x99u8, 0xD6u8, 0x62u8],
            "\u{0061}\u{8A29}\u{0062}",
        );
    }

    #[test]
    fn decode_02() {
        let data = [
            0xA4, 0xB5, 0xA4, 0xE9, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！"
        let mut buf = [0u8; 2];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(decoded, "");
    }

    #[test]
    fn decode_03() {
        let data = [
            0xA4, 0xB5, 0xA4, 0xE9, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！"
        let mut buf = [0u8; 3];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "今");
    }

    #[test]
    fn decode_04() {
        let data = [
            0xA4, 0xB5, 0xA4, 0xE9, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！"
        let mut buf = [0u8; 5];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "今");
    }

    #[test]
    fn decode_05() {
        let data = [
            0xA4, 0xB5, 0x80, 0x61, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！" with an error on the second char (invalid sequence)
        let mut buf = [0u8; 3];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "今");
    }

    #[test]
    fn decode_06() {
        let data = [0xA4, 0xB5, 0xA4, 0xE9, 0x61, 0xC7, 0x56, 0x62];
        let mut buf = [0u8; 11];
        let (encoded, consumed_count) = decode_to_str(&data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 8);
        assert_eq!(encoded, "今日aはb");
    }

    #[test]
    fn decode_07() {
        let data = [
            0xA4, 0xB5, 0xA4, 0xE9, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1,
        ]; // "今日はいいよ！" with last byte chopped off.
        let mut buf = [0u8; 64];
        let (decoded, consumed_count) = decode_to_str(&data, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 12);
        assert_eq!(decoded, "今日はいいよ");
    }

    #[test]
    fn decode_error_01() {
        error_decode(&[0x80u8, 0x61u8], (0, 1), 0); // Invalid sequence
        error_decode(&[0xFFu8, 0x61u8], (0, 1), 0); // Invalid sequence
        error_decode(&[0xFEu8, 0x39u8], (0, 1), 0); // Invalid sequence
        error_decode(&[0x87u8, 0x66u8], (0, 1), 0); // Undefined code
        error_decode(&[0x81u8, 0x40u8], (0, 1), 0); // Undefined code

        // Invalid sequence, second byte outside of ascii range.
        error_decode(&[0x81u8, 0xA0u8], (0, 2), 0);

        // Undefined code, second byte outside of ascii range.
        error_decode(&[0x81u8, 0xFEu8], (0, 2), 0);
    }

    #[test]
    fn decode_error_02() {
        let data = [
            0x80, 0x61, 0xA4, 0xE9, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！" with an error on the first char (invalid sequence)
        let mut buf = [0u8; 2];
        let error = decode_to_str(&data, &mut buf, true);
        assert_eq!(
            error,
            Err(DecodeError {
                error_range: (0, 1),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn decode_error_03() {
        let data = [
            0xA4, 0xB5, 0x80, 0x61, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！" with an error on the second char (invalid sequence)
        let mut buf = [0u8; 4];
        // Scope to contain borrow.
        {
            let error = decode_to_str(&data, &mut buf, true);
            assert_eq!(
                error,
                Err(DecodeError {
                    error_range: (2, 3),
                    output_bytes_written: 3,
                })
            );
        }
        assert_eq!(&buf[..3], b"\xE4\xBB\x8A");
    }

    #[test]
    fn decode_error_04() {
        let data = [
            0xA4, 0xB5, 0xA4, 0xE9, 0xC7, 0x56, 0x80, 0x61, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1, 0x49,
        ]; // "今日はいいよ！" with an error on the fourth char (invalid sequence)
        let mut buf = [0u8; 64];
        // Scope to contain borrow.
        {
            let error = decode_to_str(&data, &mut buf, true);
            assert_eq!(
                error,
                Err(DecodeError {
                    error_range: (6, 7),
                    output_bytes_written: 9,
                })
            );
        }
        assert_eq!(&buf[..9], b"\xE4\xBB\x8A\xE6\x97\xA5\xE3\x81\xAF");
    }

    #[test]
    fn decode_error_05() {
        let data = [
            0xA4, 0xB5, 0xA4, 0xE9, 0xC7, 0x56, 0xC6, 0xEA, 0xC6, 0xEA, 0xC7, 0x6F, 0xA1,
        ]; // "今日はいいよ！" with last byte chopped off.
        let mut buf = [0u8; 64];
        // Scope to contain borrow.
        {
            let error = decode_to_str(&data, &mut buf, true);
            assert_eq!(
                error,
                Err(DecodeError {
                    error_range: (12, 13),
                    output_bytes_written: 18,
                })
            );
        }
        assert_eq!(&buf[..18], "今日はいいよ".as_bytes());
    }

    #[test]
    fn encode_01() {
        let text = "今日はいいよ！";
        let mut buf = [0u8; 1];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn encode_02() {
        let text = "今日はいいよ！";
        let mut buf = [0u8; 2];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(encoded, &[0xA4, 0xB5]);
    }

    #[test]
    fn encode_03() {
        let text = "今日はいいよ！";
        let mut buf = [0u8; 3];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(encoded, &[0xA4, 0xB5]);
    }

    #[test]
    fn encode_04() {
        let text = "今日aはbいいよ！";
        let mut buf = [0u8; 8];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 11);
        assert_eq!(encoded, &[0xA4, 0xB5, 0xA4, 0xE9, 0x61, 0xC7, 0x56, 0x62]);
    }

    #[test]
    fn encode_05() {
        let text = "今日😺はいいよ！";
        let mut buf = [0u8; 4];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 6);
        assert_eq!(encoded, &[0xA4, 0xB5, 0xA4, 0xE9]);
    }

    #[test]
    fn encode_06() {
        let text = "\u{00CA}\u{0304}";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 4);
        assert_eq!(encoded, &[0x88, 0x62]);
    }

    #[test]
    fn encode_07() {
        let text = "\u{00CA}\u{0304}";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 4);
        assert_eq!(encoded, &[0x88, 0x62]);
    }

    #[test]
    fn encode_08() {
        let text = "\u{00CA}";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(encoded, &[0x88, 0x66]);
    }

    #[test]
    fn encode_09() {
        let text = "\u{00CA}";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn encode_10() {
        let text = "\u{00CA}よ";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 5);
        assert_eq!(encoded, &[0x88, 0x66, 0xc7, 0x6f]);
    }

    #[test]
    fn encode_11() {
        let text = "\u{00CA}よ";
        let mut buf = [0u8; 64];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 5);
        assert_eq!(encoded, &[0x88, 0x66, 0xc7, 0x6f]);
    }

    #[test]
    fn encode_error_01() {
        let text = "😺今日はいいよ！";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf, true),
            Err(EncodeError {
                character: '😺',
                error_range: (0, 4),
                output_bytes_written: 0,
            })
        );
    }

    #[test]
    fn encode_error_02() {
        let text = "今日😺はいいよ！";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf, true),
            Err(EncodeError {
                character: '😺',
                error_range: (6, 10),
                output_bytes_written: 4,
            })
        );
        assert_eq!(&buf[..4], &[0xA4, 0xB5, 0xA4, 0xE9]);
    }
}
