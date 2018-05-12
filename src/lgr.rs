use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use std::fs;
use std::path::Path;

use super::{constants, Clip, ElmaError, utils::{string_null_pad, trim_string}};

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
    pub fn load<P: AsRef<Path>>(file: P) -> Result<Self, ElmaError> {
        let buffer = fs::read(file)?;
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
        // there are no other LGR versions possible, so no need to store it (?)
        if version != b"LGR12" {
            return Err(ElmaError::InvalidLGRFile(LGRError::InvalidVersion(
                version.to_vec(),
            )));
        }

        let picture_len = buffer.read_u32::<LE>()? as usize;
        let expected_header = buffer.read_i32::<LE>()?;
        if expected_header != constants::LGR {
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
        if expected_eof != constants::LGR_EOF {
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

            self.picture_data.push(PictureData { name, data });
            buffer = &buffer[24 + bytes_len..];
            bytes_read += 24 + bytes_len;
        }
        Ok(bytes_read)
    }

    /// Returns a Vec with bytes representing the LGR as a buffer.
    pub fn to_bytes(&self) -> Result<Vec<u8>, ElmaError> {
        let mut bytes = vec![];
        bytes.extend_from_slice(b"LGR12");
        bytes.write_u32::<LE>(self.picture_data.len() as u32)?;
        bytes.write_i32::<LE>(constants::LGR)?;
        bytes.extend_from_slice(&self.write_picture_list()?);
        bytes.extend_from_slice(&self.write_picture_data()?);
        bytes.write_i32::<LE>(constants::LGR_EOF)?;

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

        for picture in self.picture_list.iter() {
            names.extend_from_slice(&string_null_pad(&picture.name, 10)?);
            picture_types.write_u32::<LE>(picture.picture_type as u32)?;
            distances.write_u32::<LE>(picture.distance as u32)?;
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
        let marker = &[0x95, 0x4C, 0x00, 0x98, 0x95, 0x4C, 0x00];

        for picture in self.picture_data.iter() {
            bytes.extend_from_slice(&string_null_pad(&picture.name, 13)?);
            bytes.extend_from_slice(marker);
            bytes.write_u32::<LE>(picture.data.len() as u32)?;
            bytes.extend_from_slice(&picture.data);
        }

        Ok(bytes)
    }

    /// Save the LGR to a file.
    pub fn save<P: AsRef<Path>>(&self, filename: P) -> Result<(), ElmaError> {
        let bytes = self.to_bytes()?;
        fs::write(filename, &bytes)?;

        Ok(())
    }
}
