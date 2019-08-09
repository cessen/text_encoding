//! Encoding/decoding functions for the WHATWG variant of Shift JIS.

use core;
use {DecodeError, DecodeResult, EncodeError, EncodeResult};

// Generated by build.rs  Contains ENCODE_TABLE and DECODE_TABLE.
include!("generated/whatwg/shiftjis_whatwg_tables.rs.inc");

pub fn encode_from_str<'a>(input: &str, out_buffer: &'a mut [u8]) -> EncodeResult<'a> {
    // Do the encode.
    let mut input_i = 0;
    let mut output_i = 0;
    for (offset, c) in input.char_indices() {
        let mut code = c as u32;
        if output_i >= out_buffer.len() {
            break;
        } else if code <= 128 {
            // Ascii
            out_buffer[output_i] = code as u8;
            output_i += 1;
            input_i = offset + 1;
        } else if code == 0xA5 {
            // Special case 1
            out_buffer[output_i] = 0x5C;
            output_i += 1;
            input_i = offset + 1;
        } else if code == 0x203E {
            // Special case 2
            out_buffer[output_i] = 0x7E;
            output_i += 1;
            input_i = offset + 1;
        } else if code >= 0xFF61 && code <= 0xFF9F {
            // Special case 3
            out_buffer[output_i] = (code - 0xFF61 + 0xA1) as u8;
            output_i += 1;
            input_i = offset + 1;
        } else {
            if (output_i + 1) < out_buffer.len() {
                let jis_bytes = if code >= 0xE000 && code <= 0xE757 {
                    // Special case 5
                    // Note: the WHATWG spec doesn't specify this, but it does it
                    // in reverse during decoding, and these simply don't map
                    // otherwise.  So we're just making it map back properly.
                    let jis_ptr = code - 0xE000 + 8836;
                    let lead = jis_ptr / 188;
                    let lead_offset = if lead < 0x1F { 0x81 } else { 0xC1 };
                    let trail = jis_ptr % 188;
                    let trail_offset = if trail < 0x3F { 0x40 } else { 0x41 };
                    [(lead + lead_offset) as u8, (trail + trail_offset) as u8]
                } else {
                    if code == 0x2212 {
                        // Special case 4
                        code = 0xFF0D;
                    }
                    let code = core::char::from_u32(code).unwrap();
                    if let Ok(ptr_i) = ENCODE_TABLE.binary_search_by_key(&code, |x| x.0) {
                        ENCODE_TABLE[ptr_i].1
                    } else {
                        return Err(EncodeError {
                            character: c,
                            error_range: (offset, offset + c.len_utf8()),
                            output_bytes_written: output_i,
                        });
                    }
                };

                out_buffer[output_i] = jis_bytes[0];
                out_buffer[output_i + 1] = jis_bytes[1];
                output_i += 2;
                input_i = offset + 1;
            } else {
                break;
            }
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
        } else if byte_1 == 128 {
            // Edge-case, close to ascii.
            if output_i + 1 >= out_buffer.len() {
                break;
            } else {
                out_buffer[output_i] = 0xC2;
                out_buffer[output_i + 1] = 0x80;
                output_i += 2;
                input_i += 1;
            }
        } else {
            // Get our decoded data, either from the table or by special handling.
            let (string, input_consumed) = if byte_1 >= 0xA1 && byte_1 <= 0xDF {
                // Special case 3 from encode function above.
                (
                    core::char::from_u32(byte_1 as u32 + 0xFF61 - 0xA1)
                        .unwrap()
                        .encode_utf8(&mut buf),
                    1,
                )
            } else if (byte_1 > 0x9F && byte_1 < 0xE0) || byte_1 > 0xFC {
                // Error: invalid leading byte.
                return Err(DecodeError {
                    error_range: (input_i, input_i + 1),
                    output_bytes_written: output_i,
                });
            } else if let Some(&byte_2) = itr.next() {
                if byte_2 < 0x40 || byte_2 == 0x7F || byte_2 > 0xFC {
                    // Error: invalid trailing byte.
                    // WHATWG dictates that if the second byte is ascii, it
                    // remains part of the stream, and thus (in our case) is
                    // not treated as part of the error.
                    return Err(DecodeError {
                        error_range: (input_i, input_i + if byte_2 <= 127 { 1 } else { 2 }),
                        output_bytes_written: output_i,
                    });
                }

                let jis_ptr = {
                    let lead_offset = if byte_1 < 0xA0 { 0x81 } else { 0xC1 };
                    let trail_offset = if byte_2 < 0x7F { 0x40 } else { 0x41 };
                    (byte_1 as u32 - lead_offset) * 188 + byte_2 as u32 - trail_offset
                };

                if jis_ptr >= 8836 && jis_ptr <= 10715 {
                    // Special case 5
                    (
                        core::char::from_u32(jis_ptr + 0xE000 - 8836)
                            .unwrap()
                            .encode_utf8(&mut buf),
                        2,
                    )
                } else if jis_ptr as usize >= DECODE_TABLE.len()
                    || DECODE_TABLE[jis_ptr as usize] == '�'
                {
                    // Error: correctly formed but undefined code.
                    // WHATWG dictates that if the second byte is ascii, it
                    // remains part of the stream, and thus (in our case) is
                    // not treated as part of the error.
                    return Err(DecodeError {
                        error_range: (input_i, input_i + if byte_2 <= 127 { 1 } else { 2 }),
                        output_bytes_written: output_i,
                    });
                } else {
                    // Encode codepoint to utf8.
                    (DECODE_TABLE[jis_ptr as usize].encode_utf8(&mut buf), 2)
                }
            } else {
                // No trailing byte available.  If it's not end-of-input, not a problem.
                // But if it is, then that's an error.
                if !is_end {
                    break;
                } else {
                    return Err(DecodeError {
                        error_range: (input_i, input_i + 1),
                        output_bytes_written: output_i,
                    });
                }
            };

            // Copy decoded data to output.
            if (output_i + string.len()) > out_buffer.len() {
                // Not enough space in output buffer.
                break;
            }
            out_buffer[output_i..(output_i + string.len())].copy_from_slice(string.as_bytes());

            // Update our counters.
            input_i += input_consumed;
            output_i += string.len();
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

    // Helper function.
    fn correct_encode(input: &str, expected_output: &[u8]) {
        let mut buf = [0u8; 256];
        assert_eq!(
            encode_from_str(input, &mut buf),
            Ok((expected_output, input.len())),
        );
    }

    // Adapted from tests in encoding_rs:
    // https://crates.io/crates/encoding_rs
    #[test]
    fn decode_01() {
        // Empty
        correct_decode(b"", &"");

        // ASCII
        correct_decode(b"\x61\x62", "\u{0061}\u{0062}");

        // Half-width
        correct_decode(b"\xA1", "\u{FF61}");
        correct_decode(b"\xDF", "\u{FF9F}");

        // EUDC
        correct_decode(b"\xF0\x40", "\u{E000}");
        correct_decode(b"\xF9\xFC", "\u{E757}");
        correct_decode(b"\xFA\x40", "\u{2170}");

        // JIS 0208
        correct_decode(b"\x81\x40", "\u{3000}");
        correct_decode(b"\xEE\xFC", "\u{FF02}");
        correct_decode(b"\xFA\x40", "\u{2170}");
        correct_decode(b"\xFC\x4B", "\u{9ED1}");
    }

    #[test]
    fn decode_02() {
        let data = b"\x8D\xA1\x93\xFA\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！"
        let mut buf = [0u8; 2];
        let (decoded, consumed_count) = decode_to_str(data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(decoded, "");
    }

    #[test]
    fn decode_03() {
        let data = b"\x8D\xA1\x93\xFA\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！"
        let mut buf = [0u8; 3];
        let (decoded, consumed_count) = decode_to_str(data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "今");
    }

    #[test]
    fn decode_04() {
        let data = b"\x8D\xA1\x93\xFA\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！"
        let mut buf = [0u8; 5];
        let (decoded, consumed_count) = decode_to_str(data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "今");
    }

    #[test]
    fn decode_05() {
        let data = b"\x8D\xA1\x81\x3F\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！" with an error on the second char (invalid sequence)
        let mut buf = [0u8; 3];
        let (decoded, consumed_count) = decode_to_str(data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 2);
        assert_eq!(decoded, "今");
    }

    #[test]
    fn decode_06() {
        let data = b"\x8D\xA1\x93\xFA\x61\xD6\x82\xCD\x62"; // "今日aﾖはb"
        let mut buf = [0u8; 14];
        let (encoded, consumed_count) = decode_to_str(data, &mut buf, true).unwrap();
        assert_eq!(consumed_count, 9);
        assert_eq!(encoded, "今日aﾖはb");
    }

    #[test]
    fn decode_07() {
        let data = b"\x8D\xA1\x93\xFA\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81"; // "今日はいいよ！" with last byte chopped off.
        let mut buf = [0u8; 64];
        let (decoded, consumed_count) = decode_to_str(data, &mut buf, false).unwrap();
        assert_eq!(consumed_count, 12);
        assert_eq!(decoded, "今日はいいよ");
    }

    // Adapted from tests in encoding_rs:
    // https://crates.io/crates/encoding_rs
    #[test]
    fn decode_error_01() {
        error_decode(b"\xA0", (0, 1), 0);
        error_decode(b"\xA0+", (0, 1), 0);
        error_decode(b"\xE0+", (0, 1), 0);

        error_decode(b"\xEF\xFC", (0, 2), 0);

        error_decode(b"\x81\x3F", (0, 1), 0);
        error_decode(b"\xEE\xFD", (0, 2), 0);
        error_decode(b"\xFA\x3F", (0, 1), 0);
        error_decode(b"\xFC\x4C", (0, 1), 0);
    }

    #[test]
    fn decode_error_02() {
        let data = b"\x81\x3F\x93\xFA\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！" with an error on the first char (invalid sequence)
        let mut buf = [0u8; 2];
        let error = decode_to_str(data, &mut buf, true);
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
        let data = b"\x8D\xA1\x81\x3F\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！" with an error on the second char (invalid sequence)
        let mut buf = [0u8; 4];
        // Scope to contain borrow.
        {
            let error = decode_to_str(data, &mut buf, true);
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
        let data = b"\x8D\xA1\x93\xFA\x82\xCD\x81\x3F\x82\xA2\x82\xE6\x81\x49"; // "今日はいいよ！" with an error on the fourth char (invalid sequence)
        let mut buf = [0u8; 64];
        // Scope to contain borrow.
        {
            let error = decode_to_str(data, &mut buf, true);
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
        let data = b"\x8D\xA1\x93\xFA\x82\xCD\x82\xA2\x82\xA2\x82\xE6\x81"; // "今日はいいよ！" with last byte chopped off.
        let mut buf = [0u8; 64];
        // Scope to contain borrow.
        {
            let error = decode_to_str(data, &mut buf, true);
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

    // Adapted from tests in encoding_rs:
    // https://crates.io/crates/encoding_rs
    #[test]
    fn encode_01() {
        // Empty
        correct_encode("", b"");

        // ASCII
        correct_encode("\u{0061}\u{0062}", b"\x61\x62");

        // Exceptional code points
        correct_encode("\u{0080}", b"\x80");
        correct_encode("\u{00A5}", b"\x5C");
        correct_encode("\u{203E}", b"\x7E");
        correct_encode("\u{2212}", b"\x81\x7C");

        // Half-width
        correct_encode("\u{FF61}", b"\xA1");
        correct_encode("\u{FF9F}", b"\xDF");

        // EUDC
        correct_encode("\u{E000}", b"\xF0\x40");
        correct_encode("\u{E757}", b"\xF9\xFC");

        // JIS 0208
        correct_encode("\u{3000}", b"\x81\x40");
        // TODO: these depend on excluding specific parts
        // of the table that contains duplicates, to force
        // using later entries.  Not sure if we want to do
        // that, even though it's specified in the WHATWG
        // spec.
        // correct_encode("\u{FF02}", b"\xFA\x57");
        // correct_encode("\u{2170}", b"\xFA\x40");
        // correct_encode("\u{9ED1}", b"\xFC\x4B");
    }

    #[test]
    fn encode_02() {
        let text = "今日はいいよ！";
        let mut buf = [0u8; 1];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 0);
        assert_eq!(encoded, &[]);
    }

    #[test]
    fn encode_03() {
        let text = "今日はいいよ！";
        let mut buf = [0u8; 2];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(encoded, b"\x8D\xA1");
    }

    #[test]
    fn encode_04() {
        let text = "今日はいいよ！";
        let mut buf = [0u8; 3];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 3);
        assert_eq!(encoded, b"\x8D\xA1");
    }

    #[test]
    fn encode_05() {
        let text = "今日aﾖはbいいよ！";
        let mut buf = [0u8; 9];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 14);
        assert_eq!(encoded, b"\x8D\xA1\x93\xFA\x61\xD6\x82\xCD\x62");
    }

    #[test]
    fn encode_06() {
        let text = "今日😺はいいよ！";
        let mut buf = [0u8; 4];
        let (encoded, consumed_count) = encode_from_str(text, &mut buf).unwrap();
        assert_eq!(consumed_count, 6);
        assert_eq!(encoded, b"\x8D\xA1\x93\xFA");
    }

    #[test]
    fn encode_error_01() {
        let text = "😺今日はいいよ！";
        let mut buf = [0u8; 64];
        assert_eq!(
            encode_from_str(text, &mut buf),
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
            encode_from_str(text, &mut buf),
            Err(EncodeError {
                character: '😺',
                error_range: (6, 10),
                output_bytes_written: 4,
            })
        );
        assert_eq!(&buf[..4], b"\x8D\xA1\x93\xFA");
    }
}
