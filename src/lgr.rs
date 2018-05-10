use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use byteorder::{ByteOrder, LittleEndian as LE, ReadBytesExt, WriteBytesExt};

use super::{Clip, ElmaError};

/// LGR structure.
#[derive(Default, Clone, Eq, PartialEq)]
pub struct LGR {
    /// LGR/filename.
    pub name: String,
    /// List of pictures.
    pub picture_list: Vec<Picture>,
    /// Picture data.
    pub picture_data: Vec<PictureData>,
}

/// LGR picture structure.
#[derive(Default, Clone, Eq, PartialEq)]
pub struct Picture {
    /// Picture name.
    pub name: String,
    /// Picture type.
    pub picture_type: PictureType,
    /// Default distance, 1-999.
    pub distance: u16,
    /// Default clipping.
    pub clipping: Clip,
    /// Transparency.
    pub transparency: Transparency,
}

/// Picture data.
#[derive(Default, Clone, Eq, PartialEq)]
pub struct PictureData {
    /// Picture name.
    pub name: String,
    /// Raw byte data.
    pub data: Vec<u8>,
}

/// Picture types.
#[derive(Clone, Eq, PartialEq)]
pub enum PictureType {
    /// Normal picture.
    Normal,
    /// Texture.
    Texture,
    /// Mask for textures.
    Mask,
}

impl Default for PictureType {
    fn default() -> Self {
        PictureType::Normal
    }
}

/// Transparency.
#[derive(Clone, Eq, PartialEq)]
pub enum Transparency {
    /// No transparency. Only valid for ´Mask´ picture types.
    Solid,
    /// Palette index 0 is transparent color.
    Palette,
    /// Top left pixel is transparent color.
    TopLeft,
    /// Top right pixel is transparent color.
    TopRight,
    /// Bottom left pixel is transparent color.
    BottomLeft,
    /// Bottom right pixel is transparent color.
    BottomRight,
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
        let mut file = File::open(file)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        Self::parse_lgr(&buffer)
    }

    fn parse_lgr(buffer: &[u8]) -> Result<Self, ElmaError> {
        let mut lgr = Self::new();

        match &buffer[..5] {
            b"LGR12" => {}
            _ => return Err(ElmaError::InvalidLGRFile),
        };

        Ok(lgr)
    }
}
