#![no_std]
#![allow(clippy::cast_lossless)]

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
//!
//! # Str -> Text Encoding
//!
//! Basic usage of a module's `encode_from_str()` function looks like this:
//!
//! ```
//! # use text_encoding::utf8::encode_from_str;
//! let text = "Hi, I'm some string.";
//! let mut out_buffer = [0u8; 100];
//! let (encoded_text, _) = encode_from_str(text, &mut out_buffer).unwrap();
//! ```
//!
//! The resulting encoded text is in `encoded_text`, which is an immutable
//! slice from the `out_buffer` that we passed in.
//!
//! The basic idea is that you provide the encoding function with both the
//! input text _and_ a buffer to write the encoded text to.  It then returns a
//! slice to just the part of the buffer containing the encoded text.
//!
//! ## Error Handling
//!
//! The above example is over-simplified, however.  It ignores some really
//! important things that you will want to handle in real code.  For example,
//! error handling.
//!
//! Error handling is both flexible and relatively straightforward.  Because
//! the `str` type in Rust is guaranteed to always be valid utf8, the only
//! possible error is when the output encoding is unable to represent some
//! part of the input utf8 text.  For example, if you're encoding to latin-1
//! but the utf8 text contains an emoji.
//!
//! When that happens, the encoding function will still encode all of the data
//! up to the unrepresentable text, and will then return an error with the
//! following information:
//!
//! - The un-encodable `char` in the input text.
//! - The byte index range of that char in the input text.
//! - The number of bytes written to the output buffer.  These always start at
//!   the beginning of the buffer, and are guaranteed to all be valid for the
//!   output encoding, so you can simply take a `&[..output_bytes_written]`
//!   slice of the output buffer to get the valid encoded data so far.
//!
//! This provides all of the information you need to handle the error however
//! you like.  You could choose to just panic (which is basically what our
//! first example does with `unwrap()`).  Or you could skip the problematic
//! `char` and continue encoding.  Or you could substitute another `char` in
//! place of the problematic `char`.  Really, it's up to you.
//!
//! ## Streaming Encoding
//!
//! The last main aspect of the encoding API has to do with a streaming style
//! of text encoding.  In our basic example, we simply created a buffer we
//! knew was large enough to contain all of the encoded text data.  But what
//! if your input text is huge, and you don't want to allocate a huge output
//! buffer?  Or what if you are receiving the text one chunk at a time, and
//! want to handle the chunks eagerly?
//!
//! That is what the second return parameter is for, which we ignored with a
//! `_` in our basic example.
//!
//! If the output buffer is too small to hold the entirety of the encoded
//! text, the encode function will write all of the data that does fit,
//! and then return the number of consumed _input_ bytes corresponding to the
//! returned buffer slice.  Also note that the returned buffer slice will
//! always itself be completely valid encoded text--the encoding function will
//! never, for example, write just part of a character.  Moreover, the number
//! of consumed input bytes will always be at a valid utf8 codepoint boundary.
//!
//! This API allows straightforward handling of text encoding in a streaming
//! fashion.  You can keep re-using the same buffer, and iteratively encode
//! more of the text, write it to disk, then encode more, then write that to
//! disk, etc.  This works both in the case that you have a too-small
//! output buffer but are repeatingly passing all of the remaining input
//! text, _and_ in the case where your ouput buffer is not your bottleneck and
//! instead you are receiving the input text data in chunks.
//!
//! Note that in the latter case--receiving the input text in chunks--you need
//! to be a little careful.  The encoding function may not consume all of a
//! given chunk.  In such cases, you will need to _append_ the next chunk to
//! unprocessed text, and pass that as the next chunk.
//!
//! ## Stateful Encoders
//!
//! Not all text encoding can be handled in a stateless way.  There are some
//! text encodings that require some kind of state to encode/decode
//! correctly.  In such cases, the `encode_from_str()` function will have one
//! or two additional parameters.
//!
//! The simplest variation is an encoding that simply needs to know when the
//! passed input is the last chunk of text in a stream.  In such cases,
//! the signature will include an `is_end` boolean parameter.
//!
//! If the passed input text contains _all_ of the remaining text, you should
//! simply pass `true` for that parameter.  However, when you are encoding
//! text one input-chunk at a time, you should pass `false` for all chunks
//! except the final one, which should be passed `true` to correctly finalize
//! the encoding.
//!
//! The other kind of state, which only a small number of encoders need, is
//! _internal_ state.  Unlike `is_end`, which must be managed by you, internal
//! state is  handled entirely by the encoder itself.  In such cases, the
//! encoder's module will have a `State` type which you must instantiate and
//! repeatedly pass to the encoding function's `state` parameter on every
//! call.
//!
//! Note that you must not re-use state for different pieces of text.  For an
//! entirely new piece of text, you must start again with a new state.
//! However, all of the `State` types are both very small and are `Copy`
//! types, so this is very cheap.
//!
//! Having a separate `State` type may seem like a strange API choice, since
//! the same semantics could be achieved with a struct with encode/decode
//! methods.  But the intent here is to provide a flexible, low-level, C-esk
//! API which can then be built on top of by other crates to provide
//! friendlier APIs.
//!
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
