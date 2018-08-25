#![no_std]

//! A library for incrementally encoding/decoding between utf8 and various
//! text encodings.

pub mod big5_whatwg;
pub mod single_byte;
pub mod utf16_be;
pub mod utf16_le;
pub mod utf32_be;
pub mod utf32_le;
pub mod utf8;
mod utils;

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
