extern crate elma;

use elma::utils::*;

#[test]
/// Supply some bogus utf-8 bytes.
fn trim_string_invalid_utf8() {
    assert_eq!(
        trim_string(&[222, 222, 222, 100, 211]).unwrap_err(),
        elma::ElmaError::StringFromUtf8(0)
    );
}

#[test]
/// Supply shorter padding than string length.
fn string_null_pad_length_error() {
    assert_eq!(
        string_null_pad("elma-rust", 5).unwrap_err(),
        elma::ElmaError::PaddingTooShort(-4)
    );
}

#[test]
/// Supply UTF-8 characters.
fn string_null_pad_utf8_error() {
    assert_eq!(
        string_null_pad("✗✗✗✗✗✗✗✗", 10).unwrap_err(),
        elma::ElmaError::NonASCII
    );
}
