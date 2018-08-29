#![no_std]

//! A library for encoding/decoding between `&str`'s and other text encodings.
//!
//! This crate is designed to be relatively low-level, but also simple.  Each
//! module in the crate represents a different text encoding, and contains
//! two functions:
//!
//! * `encode_from_str()`: converts a `&str` to the module's text encoding.
//! * `decode_to_str()`: converts text in the module's text encoding to a `&str`.
//!
//! These functions behave identically across all modules, and are therefore
//! undocumented in the individual modules.  Instead, see the documentation
//! below for how to use them.
//!
//! # Str -> Text Encoding
//!
//! TODO: explain how to use `encode_from_str()`.
//!
//! # Text Encoding -> Str
//!
//! TODO: explain how to use `decode_to_str()`.

pub mod big5_whatwg;
pub mod shiftjis_whatwg;
pub mod single_byte;
pub mod utf16_be;
pub mod utf16_le;
pub mod utf32_be;
pub mod utf32_le;
pub mod utf8;
mod utils;

/// Result type for encoding text from a `&str` to a target encoding.
///
/// The Ok() variant provides the encoded text data and the number of
/// bytes of input consumed.
pub type EncodeResult<'a> = Result<(&'a [u8], usize), EncodeError>;

/// Result type for decoding text from a source encoding to a `&str`.
///
/// The Ok() variant provides the decoded text and the number of bytes
/// of input consumed.
pub type DecodeResult<'a> = Result<(&'a str, usize), DecodeError>;

/// An error when encoding from a `&str` to some other format.
///
/// Since `&str`'s are always valid text, the only possible error is
/// encountering a char that is not representable in the target encoding.
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

/// An error when decoding from some other format to a `&str`.
///
/// The only possible error when decoding is encountering data in the input
/// that is invalid for the text encoding we're attempting to decode from.
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
