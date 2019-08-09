extern crate text_encoding;

use text_encoding::{single_byte::*, *};

fn main() {
    let text = "Hello world!";

    let mut buf = [0u8; 100];
    let utf16 = encode_from_str(Encoding::Utf16BE, text, &mut buf, true)
        .unwrap()
        .0;

    let mut buf_2 = [0u8; 100];
    let text_2 = decode_to_str(Encoding::Utf16BE, utf16, &mut buf_2, true)
        .unwrap()
        .0;

    assert_eq!(text, text_2);
}

/// Describes a text encoding.
#[derive(Debug, Copy, Clone)]
pub enum Encoding {
    Utf8,
    Utf16BE,
    Utf16LE,
    Utf32BE,
    Utf32LE,
    Big5WHATWG,     // BIG5, WHATWG variant
    ShiftJISWHATWG, // Shift JIS, WHATWG variant
    Ascii,          // US Ascii
    Ibm866,         // IBM 866
    Iso8859_1,      // ISO/IEC 8859-1, Latin1
    Iso8859_2,      // ISO/IEC 8859-2, Latin2
    Iso8859_3,      // ISO/IEC 8859-3, Latin3
    Iso8859_4,      // ISO/IEC 8859-4, Latin4
    Iso8859_5,      // ISO/IEC 8859-5
    Iso8859_6,      // ISO/IEC 8859-6
    Iso8859_7,      // ISO/IEC 8859-7
    Iso8859_8,      // ISO/IEC 8859-8
    Iso8859_9,      // ISO/IEC 8859-9
    Iso8859_10,     // ISO/IEC 8859-10
    Iso8859_11,     // ISO/IEC 8859-11, also called TIS-620
    Iso8859_13,     // ISO/IEC 8859-13
    Iso8859_14,     // ISO/IEC 8859-14
    Iso8859_15,     // ISO/IEC 8859-15
    Iso8859_16,     // ISO/IEC 8859-16
    Koi8R,          // KOI8-R
    Koi8U,          // KOI8-U
    Macintosh,      // Macintosh
    MacCyrillic,    // x-max-cyrillic
    Windows874,     // Windows code page 874
    Windows1250,    // Windows code page 1250
    Windows1251,    // Windows code page 1251
    Windows1252,    // Windows code page 1252
    Windows1253,    // Windows code page 1253
    Windows1254,    // Windows code page 1254
    Windows1255,    // Windows code page 1255
    Windows1256,    // Windows code page 1256
    Windows1257,    // Windows code page 1257
    Windows1258,    // Windows code page 1258
}

/// Encodes text from utf8 to a destination encoding.
pub fn encode_from_str<'a>(
    output_encoding: Encoding,
    input: &str,
    output: &'a mut [u8],
    is_end: bool,
) -> EncodeResult<'a> {
    match output_encoding {
        Encoding::Utf8 => utf8::encode_from_str(input, output),
        Encoding::Utf16BE => utf16_be::encode_from_str(input, output),
        Encoding::Utf16LE => utf16_le::encode_from_str(input, output),
        Encoding::Utf32BE => utf32_be::encode_from_str(input, output),
        Encoding::Utf32LE => utf32_le::encode_from_str(input, output),
        Encoding::Big5WHATWG => big5_whatwg::encode_from_str(input, output, is_end),
        Encoding::ShiftJISWHATWG => shiftjis_whatwg::encode_from_str(input, output),
        Encoding::Ascii => ascii::encode_from_str(input, output),
        Encoding::Ibm866 => ibm_866::encode_from_str(input, output),
        Encoding::Iso8859_1 => iso_8859_1::encode_from_str(input, output),
        Encoding::Iso8859_2 => iso_8859_2::encode_from_str(input, output),
        Encoding::Iso8859_3 => iso_8859_3::encode_from_str(input, output),
        Encoding::Iso8859_4 => iso_8859_4::encode_from_str(input, output),
        Encoding::Iso8859_5 => iso_8859_5::encode_from_str(input, output),
        Encoding::Iso8859_6 => iso_8859_6::encode_from_str(input, output),
        Encoding::Iso8859_7 => iso_8859_7::encode_from_str(input, output),
        Encoding::Iso8859_8 => iso_8859_8::encode_from_str(input, output),
        Encoding::Iso8859_9 => iso_8859_9::encode_from_str(input, output),
        Encoding::Iso8859_10 => iso_8859_10::encode_from_str(input, output),
        Encoding::Iso8859_11 => iso_8859_11::encode_from_str(input, output),
        Encoding::Iso8859_13 => iso_8859_13::encode_from_str(input, output),
        Encoding::Iso8859_14 => iso_8859_14::encode_from_str(input, output),
        Encoding::Iso8859_15 => iso_8859_15::encode_from_str(input, output),
        Encoding::Iso8859_16 => iso_8859_16::encode_from_str(input, output),
        Encoding::Koi8R => koi8_r::encode_from_str(input, output),
        Encoding::Koi8U => koi8_u::encode_from_str(input, output),
        Encoding::Macintosh => mac_roman::encode_from_str(input, output),
        Encoding::MacCyrillic => mac_cyrillic::encode_from_str(input, output),
        Encoding::Windows874 => windows_874::encode_from_str(input, output),
        Encoding::Windows1250 => windows_1250::encode_from_str(input, output),
        Encoding::Windows1251 => windows_1251::encode_from_str(input, output),
        Encoding::Windows1252 => windows_1252::encode_from_str(input, output),
        Encoding::Windows1253 => windows_1253::encode_from_str(input, output),
        Encoding::Windows1254 => windows_1254::encode_from_str(input, output),
        Encoding::Windows1255 => windows_1255::encode_from_str(input, output),
        Encoding::Windows1256 => windows_1256::encode_from_str(input, output),
        Encoding::Windows1257 => windows_1257::encode_from_str(input, output),
        Encoding::Windows1258 => windows_1258::encode_from_str(input, output),
    }
}

/// Decodes text from a source encoding to utf8.
pub fn decode_to_str<'a>(
    input_encoding: Encoding,
    input: &[u8],
    output: &'a mut [u8],
    is_end: bool,
) -> DecodeResult<'a> {
    match input_encoding {
        Encoding::Utf8 => utf8::decode_to_str(input, output, is_end),
        Encoding::Utf16BE => utf16_be::decode_to_str(input, output, is_end),
        Encoding::Utf16LE => utf16_le::decode_to_str(input, output, is_end),
        Encoding::Utf32BE => utf32_be::decode_to_str(input, output, is_end),
        Encoding::Utf32LE => utf32_le::decode_to_str(input, output, is_end),
        Encoding::Big5WHATWG => big5_whatwg::decode_to_str(input, output, is_end),
        Encoding::ShiftJISWHATWG => shiftjis_whatwg::decode_to_str(input, output, is_end),
        Encoding::Ascii => ascii::decode_to_str(input, output),
        Encoding::Ibm866 => ibm_866::decode_to_str(input, output),
        Encoding::Iso8859_1 => iso_8859_1::decode_to_str(input, output),
        Encoding::Iso8859_2 => iso_8859_2::decode_to_str(input, output),
        Encoding::Iso8859_3 => iso_8859_3::decode_to_str(input, output),
        Encoding::Iso8859_4 => iso_8859_4::decode_to_str(input, output),
        Encoding::Iso8859_5 => iso_8859_5::decode_to_str(input, output),
        Encoding::Iso8859_6 => iso_8859_6::decode_to_str(input, output),
        Encoding::Iso8859_7 => iso_8859_7::decode_to_str(input, output),
        Encoding::Iso8859_8 => iso_8859_8::decode_to_str(input, output),
        Encoding::Iso8859_9 => iso_8859_9::decode_to_str(input, output),
        Encoding::Iso8859_10 => iso_8859_10::decode_to_str(input, output),
        Encoding::Iso8859_11 => iso_8859_11::decode_to_str(input, output),
        Encoding::Iso8859_13 => iso_8859_13::decode_to_str(input, output),
        Encoding::Iso8859_14 => iso_8859_14::decode_to_str(input, output),
        Encoding::Iso8859_15 => iso_8859_15::decode_to_str(input, output),
        Encoding::Iso8859_16 => iso_8859_16::decode_to_str(input, output),
        Encoding::Koi8R => koi8_r::decode_to_str(input, output),
        Encoding::Koi8U => koi8_u::decode_to_str(input, output),
        Encoding::Macintosh => mac_roman::decode_to_str(input, output),
        Encoding::MacCyrillic => mac_cyrillic::decode_to_str(input, output),
        Encoding::Windows874 => windows_874::decode_to_str(input, output),
        Encoding::Windows1250 => windows_1250::decode_to_str(input, output),
        Encoding::Windows1251 => windows_1251::decode_to_str(input, output),
        Encoding::Windows1252 => windows_1252::decode_to_str(input, output),
        Encoding::Windows1253 => windows_1253::decode_to_str(input, output),
        Encoding::Windows1254 => windows_1254::decode_to_str(input, output),
        Encoding::Windows1255 => windows_1255::decode_to_str(input, output),
        Encoding::Windows1256 => windows_1256::decode_to_str(input, output),
        Encoding::Windows1257 => windows_1257::decode_to_str(input, output),
        Encoding::Windows1258 => windows_1258::decode_to_str(input, output),
    }
}
