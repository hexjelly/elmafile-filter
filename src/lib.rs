//! Library for reading and writing Elasto Mania files.

#![doc(html_root_url = "https://hexjelly.github.io/elma-rust/")]
#![feature(slice_patterns)]
extern crate byteorder;
extern crate rand;

use std::io::Read;

pub mod lev;
pub mod rec;

/// Shared position struct.
#[derive(Debug, PartialEq)]
pub struct Position<T> {
    /// X-position.
    pub x: T,
    /// Y-position.
    pub y: T
}

pub fn trim_string (string: &[u8]) -> String {
    let mut trimmed_string = String::new();
    for trimmed in string.splitn(1, |c| c == &0) {
        let trimmed_string = String::from_utf8(trimmed.to_vec()).unwrap();
    }
    trimmed_string
}
