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
//! here for how to use them.
//!
//!
//! # `encode_from_str()`
//!
//! Basic usage of a module's `encode_from_str()` function looks like this:
//!
//! ```
//! # use text_encoding::single_byte::ascii::encode_from_str;
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
//! This example is simplified, however, and ignores some important things
//! that you will want to handle in real code.  For example, error handling.
//!
//! ## Error Handling
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
//! # `decode_to_str()`
//!
//! The APIs for decoding are nearly identical in usage and behavior to
//! the APIs for encoding explained above.  A simple example:
//!
//! ```
//! # use text_encoding::single_byte::ascii::decode_to_str;
//! let bytes = "Some bytes of text data.".as_bytes();
//! let mut out_buffer = [0u8; 100];
//! let (decoded_text, _) = decode_to_str(bytes, &mut out_buffer).unwrap();
//! ```
//!
//! The main difference is that instead of passing in a `str` as input you
//! pass in a byte slice, and instead of the function returning a byte slice
//! it returns a `str`.  This makes sense, since the encode/decode functions
//! are inverse operations of each other.  But they otherwise operate
//! identically, including how they deal with stateful and streaming
//! encoding/decoding.
//!
//! It is worth noting that all of the decoders handle chunks correctly
//! _even when those chunks are split at otherwise invalid byte locations_.
//! For example, splitting utf16 text data in the middle of a code point is
//! perfectly valid and handled correctly when feeding chunks to the decoder.
//!
//! This robustness is important because otherwise client code would need to
//! know what a valid "split" is for every encoding used, which would push a
//! lot of encoding-specific logic into client code.  And that would somewhat
//! diminish the purpose of having a library like this in the first place.
//!
//! In a nutshell: you can blissfully throw bytes at the decoders however is
//! most convenient.
//!
//! ## Error Handling
//!
//! Error handling for decoding has the same general mechanics as the error
//! handling for encoding, but with more potential causes of errors.
//!
//! Unlike encoding, where the input text data is guaranteed to be valid,
//! decoding takes an arbitrary byte slice, which has no guarantees of being
//! valid text in the given encoding.  Therefore, a common cause of errors
//! when decoding is encountering invalid text data.
//!
//! It is tempting to think that--if valid--all input text data can be
//! unambiguously represented in utf8.  However, despite Unicode's lofty
//! goals of being able to represent all of the world's text, that is not
//! always the case.  The specifics of this are complex (see the "round-trip
//! conversion" section below), but it basically means there are two broad
//! causes of decode errors:
//!
//! - Invalid text data.
//! - Valid text data, but which we don't know how best to convert to Unicode.
//!
//! The `DecodeError` type therefore contains a `cause` field with an enum
//! indicating which of the two issues was encountered.
//!
//!
//! # Round-trip Conversions
//!
//! Round trip conversions between `str` and the various text encodings
//! are unfortunately not guaranteed to be lossless, even when no errors are
//! returned.
//!
//! A trivial case where it is obvious that lossy conversion has to take place
//! is converting from utf8 to Ascii and back.  Ascii only has 128
//! representable characters, whereas utf8 has well over 100,000.  To always
//! successfully convert between the two, some kind of lossy conversion has to
//! take place.  In this particular case, of course, the encoding API returns
//! an error if there is an unrepresentable character, so that the client
//! code can handle it however it sees fit.  However, this is not a reasonable
//! strategy in all cases.
//!
//! A good example is the ISO/IEC 2022 encoding, which can represent both
//! Chinese characters and Japanese kanji characters as distinct from each
//! other.  Unicode, however, represents Chinese and Japanese characters using
//! the same code points, not making any distinction between the two.  Thus,
//! converting between ISO/IEC 2022 and Unicode is necessarily lossy.  But
//! unlike with Ascii, if we returned an error for every ambiguous case,
//! almost every single character would be an error in typical use-cases for
//! for the encoding.  That would mean that client code would essentially
//! always need to manually encode/decode on its own in every case, at which
//! point this library wouldn't really be doing anything, and the client code
//! might as well write their own encode/decode functions instead.  Therefore,
//! in the case of ISO/IEC 2022, automatically doing a lossy conversion is
//! the more reasonable approach.
//!
//! If an encoding in this crate does silent lossy conversions, it will always
//! be documented in its module.  Otherwise you can depend on the conversion
//! functions either being lossless or returning an error.

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
    pub cause: DecodeErrorCause,
    pub error_range: (usize, usize),
    pub output_bytes_written: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DecodeErrorCause {
    /// Invalid text data was encountered.
    InvalidData,

    /// Valid text data for which a reasonable Unicode conversion is unknown.
    UnknownConversion,
}
