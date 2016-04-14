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
