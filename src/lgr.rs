use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::fs;
use std::path::PathBuf;

use super::{
    utils::{string_null_pad, trim_string}, Clip, ElmaError,
};

// Magic arbitrary number to signify start of LGR file.
const LGR: i32 = 0x00_00_03_EA;
// Magic arbitrary number to signify end of LGR file.
const LGR_EOF: i32 = 0x0B_2E_05_E7;

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
}

/// LGR structure.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LGR {
    /// Path of LGR file.
    pub path: Option<PathBuf>,
    /// List of pictures.
    pub picture_list: Vec<Picture>,
    /// Picture data.
    pub picture_data: Vec<PictureData>,
}

/// LGR picture structure.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
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

/// LGR picture data structure.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct PictureData {
    /// Picture name.
    pub name: String,
    /// Picture data.
    pub data: Vec<u8>,
}

/// Picture types.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
        Self::default()
    }

    /// Loads a LGR from file.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::lgr::*;
    /// let lgr = LGR::load("default.lgr").unwrap();
    /// ```
    pub fn load<P: Into<PathBuf>>(path: P) -> Result<Self, ElmaError> {
        let path = path.into();
        let buffer = fs::read(path.as_path())?;
        let mut lgr = Self::parse_lgr(&buffer)?;
        lgr.path = Some(path);
        Ok(lgr)
    }

    /// Load a LGR from bytes.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::lgr::*;
    /// let buffer = &[0,1,2,3,4]; // pretend this is an actual lgr file.
    /// let lgr = LGR::from_bytes(buffer).unwrap();
    /// ```
    pub fn from_bytes<B: AsRef<[u8]>>(buffer: B) -> Result<Self, ElmaError> {
        Self::parse_lgr(buffer.as_ref())
    }

    fn parse_lgr(buffer: &[u8]) -> Result<Self, ElmaError> {
        let mut lgr = Self::new();

        let (version, mut buffer) = buffer.split_at(5);
        // there are no other LGR versions possible, so no need to store it (?)
        if version != b"LGR12" {
            return Err(ElmaError::InvalidLGRFile(LGRError::InvalidVersion(
                version.to_vec(),
            )));
        }

        let picture_len = buffer.read_u32::<LE>()? as usize;
        let expected_header = buffer.read_i32::<LE>()?;
        if expected_header != LGR {
            return Err(ElmaError::InvalidLGRFile(LGRError::InvalidHeader(
                expected_header,
            )));
        }

        // picture.lst section
        let list_len = buffer.read_u32::<LE>()? as usize;
        lgr.parse_list_data(&buffer, list_len)?;
        let (_, buffer) = buffer.split_at(26 * list_len);

        // pcx data
        let bytes_read = lgr.parse_picture_data(&buffer, picture_len)?;

        let (_, mut expected_eof) = buffer.split_at(bytes_read);

        let expected_eof = expected_eof.read_i32::<LE>()?;
        if expected_eof != LGR_EOF {
            return Err(ElmaError::EOFMismatch);
        }

        Ok(lgr)
    }

    fn parse_list_data(&mut self, buffer: &[u8], len: usize) -> Result<(), ElmaError> {
        let (names, buffer) = buffer.split_at(len * 10);
        let (mut picture_types, buffer) = buffer.split_at(len * 4);
        let (mut distances, buffer) = buffer.split_at(len * 4);
        let (mut clippings, buffer) = buffer.split_at(len * 4);
        let (mut transparencies, _) = buffer.split_at(len * 4);

        for n in 0..len {
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

            self.picture_list.push(Picture {
                name,
                picture_type,
                distance,
                clipping,
                transparency,
            });
        }
        Ok(())
    }

    fn parse_picture_data(&mut self, mut buffer: &[u8], len: usize) -> Result<usize, ElmaError> {
        let mut bytes_read = 0;
        // pcx data
        for _ in 0..len {
            let (name, remaining) = buffer.split_at(12);
            let name = trim_string(&name)?;
            let (_, remaining) = remaining.split_at(8);
            let (mut bytes_len, remaining) = remaining.split_at(4);
            let bytes_len = bytes_len.read_i32::<LE>()? as usize;
            let data = remaining[..bytes_len].to_vec();

            self.picture_data.push(PictureData { name, data });
            buffer = &buffer[24 + bytes_len..];
            bytes_read += 24 + bytes_len;
        }
        Ok(bytes_read)
    }

    /// Returns a Vec with bytes representing the LGR as a buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use elma::lgr::*;
    /// let lgr = LGR::new();
    /// let buffer = lgr.to_bytes().unwrap();
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>, ElmaError> {
        let mut bytes = vec![];
        bytes.extend_from_slice(b"LGR12");
        bytes.write_u32::<LE>(self.picture_data.len() as u32)?;
        bytes.write_i32::<LE>(LGR)?;
        bytes.extend_from_slice(&self.write_picture_list()?);
        bytes.extend_from_slice(&self.write_picture_data()?);
        bytes.write_i32::<LE>(LGR_EOF)?;

        Ok(bytes)
    }

    fn write_picture_list(&self) -> Result<Vec<u8>, ElmaError> {
        let mut bytes = vec![];
        bytes.write_u32::<LE>(self.picture_list.len() as u32)?;
        let mut names = vec![];
        let mut picture_types = vec![];
        let mut distances = vec![];
        let mut clippings = vec![];
        let mut transparencies = vec![];

        for picture in &self.picture_list {
            names.extend_from_slice(&string_null_pad(&picture.name, 10)?);
            picture_types.write_u32::<LE>(picture.picture_type as u32)?;
            distances.write_u32::<LE>(u32::from(picture.distance))?;
            clippings.write_u32::<LE>(picture.clipping as u32)?;
            transparencies.write_u32::<LE>(picture.transparency as u32)?;
        }

        bytes.extend_from_slice(&names);
        bytes.extend_from_slice(&picture_types);
        bytes.extend_from_slice(&distances);
        bytes.extend_from_slice(&clippings);
        bytes.extend_from_slice(&transparencies);

        Ok(bytes)
    }

    fn write_picture_data(&self) -> Result<Vec<u8>, ElmaError> {
        let mut bytes = vec![];

        for picture in &self.picture_data {
            bytes.extend_from_slice(&string_null_pad(&picture.name, 20)?);
            bytes.write_u32::<LE>(picture.data.len() as u32)?;
            bytes.extend_from_slice(&picture.data);
        }

        Ok(bytes)
    }

    /// Save the LGR to a file.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::lgr::*;
    /// let mut lgr = LGR::new();
    /// lgr.save("cool.lgr");
    /// ```
    pub fn save<P: Into<PathBuf>>(&mut self, path: P) -> Result<(), ElmaError> {
        let bytes = self.to_bytes()?;
        let path = path.into();
        fs::write(path.as_path(), &bytes)?;
        self.path = Some(path);
        Ok(())
    }
}
