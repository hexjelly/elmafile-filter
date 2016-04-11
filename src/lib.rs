//! Library for reading and writing Elasto Mania files.

#![doc(html_root_url = "https://hexjelly.github.io/elma-rust/")]
#![feature(slice_patterns)]
extern crate byteorder;
extern crate rand;

use std::io::Read;
use std::ffi::CString;

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

/// Read n bytes function.
fn read_n<R> (reader: R, bytes: u64) -> Vec<u8>
    where R: Read {
        let mut buffer = vec![];
        let mut chunk = reader.take(bytes);
        chunk.read_to_end(&mut buffer).unwrap();
        buffer
}

/// Read `CString` data from byte vector and stop at null byte.
fn cstring_read (buffer: Vec<u8>) -> CString {
    CString::new(buffer.iter()
                       .cloned()
                       .take_while(|x| *x != 0)
                       .collect::<Vec<u8>>()
                ).unwrap()
}
