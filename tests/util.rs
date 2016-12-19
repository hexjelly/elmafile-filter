extern crate elma;

use elma::{ time_format, trim_string, string_null_pad };

#[test]
/// Supply some bogus utf-8 bytes.
fn trim_string_invalid_utf8 () {
    assert_eq!(trim_string(&[222, 222, 222, 100, 211]).unwrap_err(), elma::ElmaError::StringFromUtf8(0));
}

#[test]
/// Supply shorter padding than string length.
fn string_null_pad_length_error () {
    assert_eq!(string_null_pad("elma-rust", 5).unwrap_err(), elma::ElmaError::PaddingTooShort(-4));
}

#[test]
/// Supply UTF-8 characters.
fn string_null_pad_utf8_error () {
    assert_eq!(string_null_pad("✗✗✗✗✗✗✗✗", 10).unwrap_err(), elma::ElmaError::NonASCII);
}

#[test]
fn correct_time_format () {
    assert_eq!("11:48,01", time_format(114801).unwrap());
    assert_eq!("01:00,21", time_format(10021).unwrap());
    assert_eq!("01:00,99", time_format(10099).unwrap());
    assert_eq!("59:00,99", time_format(590099).unwrap());
    assert_eq!("00:10,00", time_format(1000).unwrap());
    assert_eq!("10:00,00", time_format(100000).unwrap());
    assert_eq!("00:00,00", time_format(0).unwrap());
    assert_eq!("59:59,99", time_format(1922039).unwrap());
}

#[test]
/// Supply "60" as seconds, should generate error.
fn invalid_time_format_1 () {
    assert_eq!(time_format(16039_i32).unwrap_err(), elma::ElmaError::InvalidTimeFormat);
}

#[test]
/// Supply "60" as minutes, should generate error.
fn invalid_time_format_2 () {
    assert_eq!(time_format(601039_i32).unwrap_err(), elma::ElmaError::InvalidTimeFormat);
}
