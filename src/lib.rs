#![no_std]

//! A library for incrementally encoding/decoding between utf8 and various
//! text encodings.

mod ascii;
mod big5_whatwg;
mod iso_8859_1;
mod single_byte;
mod utf16_be;
mod utf16_le;
mod utf32_be;
mod utf32_le;
mod utf8;
mod utils;

use single_byte::{
    ibm866, iso_8859_10, iso_8859_13, iso_8859_14, iso_8859_15, iso_8859_16, iso_8859_2,
    iso_8859_3, iso_8859_4, iso_8859_5, iso_8859_6, iso_8859_7, iso_8859_8, iso_8859_9, koi8_r,
    koi8_u, macintosh, tis_620, windows_1250, windows_1251, windows_1252, windows_1253,
    windows_1254, windows_1255, windows_1256, windows_1257, windows_1258, windows_874,
    x_mac_cyrillic,
};

/// Encodes text from utf8 to a destination encoding.
pub fn encode_from_str<'a>(
    output_encoding: Encoding,
    input: &str,
    output: &'a mut [u8],
) -> EncodeResult<'a> {
    match output_encoding {
        Encoding::Utf8 => utf8::encode_from_str(input, output),
        Encoding::Utf16BE => utf16_be::encode_from_str(input, output),
        Encoding::Utf16LE => utf16_le::encode_from_str(input, output),
        Encoding::Utf32BE => utf32_be::encode_from_str(input, output),
        Encoding::Utf32LE => utf32_le::encode_from_str(input, output),
        Encoding::Big5_WHATWG => big5_whatwg::encode_from_str(input, output),
        Encoding::Ascii => ascii::encode_from_str(input, output),
        Encoding::Ibm866 => ibm866::encode_from_str(input, output),
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
        Encoding::Tis620 => tis_620::encode_from_str(input, output),
        Encoding::Iso8859_13 => iso_8859_13::encode_from_str(input, output),
        Encoding::Iso8859_14 => iso_8859_14::encode_from_str(input, output),
        Encoding::Iso8859_15 => iso_8859_15::encode_from_str(input, output),
        Encoding::Iso8859_16 => iso_8859_16::encode_from_str(input, output),
        Encoding::Koi8R => koi8_r::encode_from_str(input, output),
        Encoding::Koi8U => koi8_u::encode_from_str(input, output),
        Encoding::Macintosh => macintosh::encode_from_str(input, output),
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
        Encoding::MacCyrillic => x_mac_cyrillic::encode_from_str(input, output),
    }
}

/// Decodes text from a source encoding to utf8.
pub fn decode_to_str<'a>(
    input_encoding: Encoding,
    input: &[u8],
    output: &'a mut [u8],
) -> DecodeResult<'a> {
    match input_encoding {
        Encoding::Utf8 => utf8::decode_to_str(input, output),
        Encoding::Utf16BE => utf16_be::decode_to_str(input, output),
        Encoding::Utf16LE => utf16_le::decode_to_str(input, output),
        Encoding::Utf32BE => utf32_be::decode_to_str(input, output),
        Encoding::Utf32LE => utf32_le::decode_to_str(input, output),
        Encoding::Big5_WHATWG => big5_whatwg::decode_to_str(input, output),
        Encoding::Ascii => ascii::decode_to_str(input, output),
        Encoding::Ibm866 => ibm866::decode_to_str(input, output),
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
        Encoding::Tis620 => tis_620::decode_to_str(input, output),
        Encoding::Iso8859_13 => iso_8859_13::decode_to_str(input, output),
        Encoding::Iso8859_14 => iso_8859_14::decode_to_str(input, output),
        Encoding::Iso8859_15 => iso_8859_15::decode_to_str(input, output),
        Encoding::Iso8859_16 => iso_8859_16::decode_to_str(input, output),
        Encoding::Koi8R => koi8_r::decode_to_str(input, output),
        Encoding::Koi8U => koi8_u::decode_to_str(input, output),
        Encoding::Macintosh => macintosh::decode_to_str(input, output),
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
        Encoding::MacCyrillic => x_mac_cyrillic::decode_to_str(input, output),
    }
}

/// Describes a text encoding.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Encoding {
    Utf8,
    Utf16BE, // Big endian
    Utf16LE, // Little endian
    Utf32BE, // Big endian
    Utf32LE, // Little endian
    // ShiftJIS,
    // ShiftJIS_WHATWG,
    // EUC_JP,
    // Big5_2003,
    Big5_WHATWG,
    Ascii,       // US Ascii
    Ibm866,      // IBM 866
    Iso8859_1,   // ISO/IEC 8859-1, Latin1
    Iso8859_2,   // ISO/IEC 8859-2, Latin2
    Iso8859_3,   // ISO/IEC 8859-3, Latin3
    Iso8859_4,   // ISO/IEC 8859-4, Latin4
    Iso8859_5,   // ISO/IEC 8859-5
    Iso8859_6,   // ISO/IEC 8859-6
    Iso8859_7,   // ISO/IEC 8859-7
    Iso8859_8,   // ISO/IEC 8859-8
    Iso8859_9,   // ISO/IEC 8859-9
    Iso8859_10,  // ISO/IEC 8859-10
    Tis620,      // TIS-620, also called ISO/IEC 8859-11
    Iso8859_13,  // ISO/IEC 8859-13
    Iso8859_14,  // ISO/IEC 8859-14
    Iso8859_15,  // ISO/IEC 8859-15
    Iso8859_16,  // ISO/IEC 8859-16
    Koi8R,       // KOI8-R
    Koi8U,       // KOI8-U
    Macintosh,   // Macintosh
    Windows874,  // Windows code page 874
    Windows1250, // Windows code page 1250
    Windows1251, // Windows code page 1251
    Windows1252, // Windows code page 1252
    Windows1253, // Windows code page 1253
    Windows1254, // Windows code page 1254
    Windows1255, // Windows code page 1255
    Windows1256, // Windows code page 1256
    Windows1257, // Windows code page 1257
    Windows1258, // Windows code page 1258
    MacCyrillic, // x-max-cyrillic
}

/// Result type for encoding text from utf8 to a target encoding.
///
/// The Ok() variant provides the number of bytes consumed and a reference
/// to the valid encoded text data.
pub type EncodeResult<'a> = Result<(usize, &'a [u8]), EncodeError>;

/// Result type for decoding text from a target encoding to utf8.
///
/// The Ok() variant provides the number of bytes consumed and a reference
/// to the valid decoded text.
pub type DecodeResult<'a> = Result<(usize, &'a str), DecodeError>;

/// Represents an error when encoding from utf8 to some other format.
///
/// Since valid input utf8 is statically assumed, the only possible
/// error is encountering a char that is not representable in the target
/// encoding.
///
/// The problematic character, the byte index range of that character in the
/// input utf8, and the number of bytes already written to the output buffer
/// are provided.
///
/// It is guaranteed that all input leading up to the problem character has
/// already been encoded and written to the output buffer.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EncodeError {
    pub character: char,
    pub error_range: (usize, usize),
    pub output_bytes_written: usize,
}

/// Represents an error when decoding to utf8 from some other format.
///
/// All supported text encodings can be fully represented in utf8, and
/// therefore the only possible error is that we encounter bytes in the
/// input data that are invalid for the text encoding we're attempting
/// to decode from.
///
/// The byte index range of the invalid input data and the number of bytes
/// already encoded and written to the output buffer are provided.
///
/// It is guaranteed that all input leading up to the invalid data has
/// already been encoded and written to the output buffer.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DecodeError {
    pub error_range: (usize, usize),
    pub output_bytes_written: usize,
}
