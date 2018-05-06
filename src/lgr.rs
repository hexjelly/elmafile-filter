extern crate combine;

use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use super::lev::Clip;

type LE = LittleEndian;

/// LGR structure.
pub struct Lgr {
    /// Filename.
    pub name: String,
    /// List of pictures.
    pub picture_list: Vec<Picture>,
    /// Picture data.
    pub picture_data: HashMap<String, Vec<u8>>,
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

/// Files that are mandatory for a valid LGR file.
pub const MANDATORY_FILES: [&'static str; 26] = [
    "q1body.pcx",
    "q1thigh.pcx",
    "q1leg.pcx",
    "q1bike.pcx",
    "q1wheel.pcx",
    "q1susp1.pcx",
    "q1susp2.pcx",
    "q1forarm.pcx",
    "q1up_arm.pcx",
    "q1head.pcx",
    "q2body.pcx",
    "q2thigh.pcx",
    "q2leg.pcx",
    "q2bike.pcx",
    "q2wheel.pcx",
    "q2susp1.pcx",
    "q2susp2.pcx",
    "q2forarm.pcx",
    "q2up_arm.pcx",
    "q2head.pcx",
    "qflag.pcx",
    "qkiller.pcx",
    "qexit.pcx",
    "qframe.pcx",
    "qcolors.pcx",
    "qfood1.pcx",
];
