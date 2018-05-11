use byteorder::{LittleEndian as LE, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::fs::read;
use std::io::{Read, Write};
use std::path::Path;

use super::{constants, Clip, ElmaError, utils::trim_string};

/// LGR related errors.
#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum LGRError {
    /// Error parsing version.
    InvalidVersion(Vec<u8>),
    /// Invalid header.
    InvalidHeader(i32),
    /// Invalid clipping.
    InvalidClip(u32),
    /// Invalid transparency.
    InvalidTransparency(u32),
    /// Error parsing PictureType.
    InvalidPictureType(u32),
    /// Invalid PCX data.
    InvalidPCXData((String, Vec<u8>, Vec<u8>)),
}

/// LGR structure.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LGR {
    /// LGR/filename.
    pub name: String,
    /// List of pictures.
    pub picture_list: HashMap<String, Picture>,
    /// Picture data.
    pub picture_data: HashMap<String, Vec<u8>>,
}

/// LGR picture structure.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Picture {
    /// Picture type.
    pub picture_type: PictureType,
    /// Default distance, 1-999.
    pub distance: u16,
    /// Default clipping.
    pub clipping: Clip,
    /// Transparency.
    pub transparency: Transparency,
}

/// Picture types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PictureType {
    /// Normal picture.
    Normal = 100,
    /// Texture.
    Texture = 101,
    /// Mask for textures.
    Mask = 102,
}

impl Default for PictureType {
    fn default() -> Self {
        PictureType::Normal
    }
}

/// Transparency.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Transparency {
    /// No transparency. Only valid for ´Mask´ picture types.
    Solid = 10,
    /// Palette index 0 is transparent color.
    Palette = 11,
    /// Top left pixel is transparent color.
    TopLeft = 12,
    /// Top right pixel is transparent color.
    TopRight = 13,
    /// Bottom left pixel is transparent color.
    BottomLeft = 14,
    /// Bottom right pixel is transparent color.
    BottomRight = 15,
}

impl Default for Transparency {
    fn default() -> Self {
        Transparency::TopLeft
    }
}

impl LGR {
    /// Creates a new LGR.
    pub fn new() -> Self {
        LGR::default()
    }

    /// Loads a LGR from file.
    pub fn load<P: AsRef<Path>>(file: P) -> Result<Self, ElmaError> {
        let buffer = read(file)?;
        Self::parse_lgr(&buffer)
    }

    /// Load a LGR from bytes.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::lgr::*;
    /// let lgr = LGR::from_bytes(&[0,1,2]).unwrap();
    /// ```
    pub fn from_bytes<B: AsRef<[u8]>>(buffer: B) -> Result<Self, ElmaError> {
        Self::parse_lgr(buffer.as_ref())
    }

    fn parse_lgr(buffer: &[u8]) -> Result<Self, ElmaError> {
        let mut lgr = Self::new();

        let (version, mut buffer) = buffer.split_at(5);
        match version {
            b"LGR12" => {}
            e => {
                return Err(ElmaError::InvalidLGRFile(LGRError::InvalidVersion(
                    e.to_vec(),
                )))
            }
        };

        let picture_len = buffer.read_u32::<LE>()? as usize;
        let expected_header = buffer.read_i32::<LE>()?;
        if expected_header != constants::LGR {
            return Err(ElmaError::InvalidLGRFile(LGRError::InvalidHeader(
                expected_header,
            )));
        }

        // picture.lst section
        let list_len = buffer.read_u32::<LE>()? as usize;

        let (names, buffer) = buffer.split_at(list_len * 10);
        let (mut picture_types, buffer) = buffer.split_at(list_len * 4);
        let (mut distances, buffer) = buffer.split_at(list_len * 4);
        let (mut clippings, buffer) = buffer.split_at(list_len * 4);
        let (mut transparencies, buffer) = buffer.split_at(list_len * 4);

        for n in 0..list_len {
            let name = trim_string(&names[10 * n..(10 * n) + 10])?;
            let picture_type = match picture_types.read_u32::<LE>()? {
                100 => PictureType::Normal,
                101 => PictureType::Texture,
                102 => PictureType::Mask,
                e => return Err(ElmaError::InvalidLGRFile(LGRError::InvalidPictureType(e))),
            };
            let distance = distances.read_u32::<LE>()? as u16;
            let clipping = match clippings.read_u32::<LE>()? {
                0 => Clip::Unclipped,
                1 => Clip::Ground,
                2 => Clip::Sky,
                e => return Err(ElmaError::InvalidLGRFile(LGRError::InvalidClip(e))),
            };
            let transparency = match transparencies.read_u32::<LE>()? {
                10 => Transparency::Solid,
                11 => Transparency::Palette,
                12 => Transparency::TopLeft,
                13 => Transparency::TopRight,
                14 => Transparency::BottomLeft,
                15 => Transparency::BottomRight,
                e => return Err(ElmaError::InvalidLGRFile(LGRError::InvalidTransparency(e))),
            };

            lgr.picture_list.insert(
                name,
                Picture {
                    picture_type,
                    distance,
                    clipping,
                    transparency,
                },
            );
        }

        // pcx data
        let (picture_data, bytes_read) = Self::parse_picture_data(&buffer, picture_len)?;
        lgr.picture_data = picture_data;

        let (_, mut expected_eof) = buffer.split_at(bytes_read);

        let expected_eof = expected_eof.read_i32::<LE>()?;
        if expected_eof != constants::LGR_EOF {
            println!("{:x} != {:x}", expected_eof, constants::LGR_EOF);
            return Err(ElmaError::EOFMismatch);
        }

        Ok(lgr)
    }

    fn parse_picture_data(
        mut buffer: &[u8],
        len: usize,
    ) -> Result<(HashMap<String, Vec<u8>>, usize), ElmaError> {
        let mut pictures = HashMap::new();
        let mut bytes_read = 0;
        // pcx data
        for _ in 0..len {
            let (name, remaining) = buffer.split_at(13);
            let name = trim_string(&name)?;
            let expected = &[0x95, 0x4C, 0x00, 0x98, 0x95, 0x4C, 0x00];
            let (buf_val, remaining) = remaining.split_at(7);
            if expected != buf_val {
                let e = (name, expected.to_vec(), buf_val.to_vec());
                return Err(ElmaError::InvalidLGRFile(LGRError::InvalidPCXData(e)));
            }
            let (mut bytes_len, remaining) = remaining.split_at(4);
            let bytes_len = bytes_len.read_i32::<LE>()? as usize;
            let data = remaining[..bytes_len].to_vec();

            pictures.insert(name, data);
            buffer = &buffer[24 + bytes_len..];
            bytes_read += 24 + bytes_len;
        }
        Ok((pictures, bytes_read))
    }
}
