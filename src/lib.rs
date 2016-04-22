//! Library for reading and writing Elasto Mania files.

#![doc(html_root_url = "https://hexjelly.github.io/elma-rust/")]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(clippy)]

extern crate byteorder;

pub mod lev;
pub mod rec;

/// Shared position struct.
#[derive(Debug, Default, PartialEq)]
pub struct Position<T> {
    /// X-position.
    pub x: T,
    /// Y-position.
    pub y: T
}

/// Trims trailing bytes after and including null byte.
pub fn trim_string (data: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    let bytes: Vec<u8> = data.into_iter()
                             .take_while(|&&d| d != 0)
                             .cloned()
                             .collect();

    String::from_utf8(bytes)
}

/// Converts times to various formats
pub fn time_format (time: i32) -> String {
    // TODO: figure out what to return etc.
    time.to_string()
}
