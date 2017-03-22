use std::io::{ Read, Write };
use std::fs::File;
use std::path::Path;
use byteorder::{ ByteOrder, ReadBytesExt, WriteBytesExt, LittleEndian };

use super::lev::Clip;

type LE = LittleEndian;

/// LGR structure.
pub struct Lgr {
    /// Filename.
    pub name: String,
    /// List of pictures.
    pub picture_list: Vec<Picture>,
    /// Picture data.
    pub picture_data: Vec<PictureData>
}

/// LGR picture structure.
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
pub struct PictureData {
    /// Picture name.
    pub name: String,
    /// Raw byte data.
    pub data: Vec<u8>,
}

/// Picture types.
pub enum PictureType {
    /// Normal picture.
    Normal,
    /// Texture.
    Texture,
    /// Mask for textures.
    Mask,
}

/// Transparency.
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
