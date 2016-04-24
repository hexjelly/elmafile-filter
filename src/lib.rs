//! Library for reading and writing Elasto Mania files.

#![doc(html_root_url = "https://hexjelly.github.io/elma-rust/")]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(clippy)]

extern crate byteorder;

pub mod lev;
pub mod rec;

/// Shared position struct used in both sub-modules.
///
/// # Examples
/// ```
/// let vertex = elma::Position { x: 23.1928_f64, y: -199.200019_f64 };
/// ```
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

/// Converts the string-as-i32 times in top10 list to strings.
///
/// # Examples
/// Thanks to the genious data structure in Elma files, the best times in a level are represented
/// visually as a string, but stored as a i32. This function will convert the i32 time to a string
/// formatted as "00:00,00".
///
/// ```
/// let time: i32 = 2039;
/// let formatted = elma::time_format(time);
/// assert_eq!("00:20,39", formatted);
/// ```
pub fn time_format (time: i32) -> String {
    let time = time.to_string().into_bytes();
    let mut formatted = String::from("00:00,00").into_bytes();

    let mut n = 7;
    for byte in time.iter().rev() {
        formatted[n] = *byte;
        if n == 6 || n == 3 {
            n -= 2;
        } else if n > 0 {
            n -= 1;
        }
    }

    String::from_utf8(formatted).unwrap()
}
