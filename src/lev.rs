//! Read and write Elasto Mania level files.

use std::io::{ Read };
use std::fs::File;
use byteorder::{ ByteOrder, ReadBytesExt, LittleEndian };
use super::{ Position, trim_string };

// Magic arbitrary number; signifies end-of-data. Followed by Top10 list(s).
const EOD: i32 = 0x0067103A;
// Magic arbitrary number; signifies end-of-file.
const EOF: i32 = 0x00845D52;

/// Game version.
#[derive(Debug, PartialEq)]
pub enum Version {
    Across,
    Elma
}

/// Type of object.
#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Apple,
    Exit,
    Killer,
    Player
}

/// Object struct. Every level requires one `ObjectType::Player` Object and at least one `ObjectType::Exit` Object.
#[derive(Debug, PartialEq)]
pub struct Object {
    /// Position. See `Position` struct.
    pub position: Position<f64>,
    /// Type of Object, see `ObjectType`.
    pub object_type: ObjectType,
    /// Applies to `ObjectType::Apple` only.
    ///
    /// 0 = normal
    /// 1 = gravity up
    /// 2 = gravity down
    /// 3 = gravity left
    /// 4 = gravity right
    // TODO: enum with gravity
    pub gravity: i32,
    /// Applies to `ObjectType::Apple` only. Valid values are 1 to 9.
    pub animation: i32
}

/// Polygon struct.
#[derive(Debug, Default, PartialEq)]
pub struct Polygon {
    /// Grass polygon.
    pub grass: bool,
    /// Vector with all vertices, see Position struct.
    pub vertices: Vec<Position<f64>>
}

impl Polygon {
    pub fn new () -> Polygon {
        Polygon {
            grass: false,
            vertices: vec![]
        }
    }
}

/// Picture struct.
#[derive(Debug, PartialEq)]
pub struct Picture {
    /// Picture name.
    pub name: String,
    /// Texture name.
    pub texture: String,
    /// Mask name.
    pub mask: String,
    /// Position. See Position struct.
    pub position: Position<f64>,
    /// Z-distance
    pub distance: i32,
    /// Clipping.
    ///
    /// 0 = unclipped
    /// 1 = ground
    /// 2 = sky
    // TODO: make enum
    pub clip: i32
}

/// Top10 list entry struct.
#[derive(Debug)]
pub struct ListEntry {
    /// Player 1 name.
    pub name_1: String,
    /// Player 2 name.
    pub name_2: String,
    /// Time.
    pub time: i32
}

/// Level struct that contains all level information.
pub struct Level {
    /// Elma or Across level.
    pub version: Version,
    /// Raw binary data of a loaded or finalized constructed level.
    raw: Vec<u8>,
    /// Random number that links level file to replay files.
    pub link: i32,
    /// Contains four integrity checks (See calculate_integrity_sums()).
    pub integrity: [f64; 4],
    /// Level name.
    pub name: String,
    /// LGR file name.
    pub lgr: String,
    /// Ground texture name.
    pub ground: String,
    /// Sky texture name.
    pub sky: String,
    /// Vector with all polygons (See Polygon).
    pub polygons: Vec<Polygon>,
    /// Vector with all objects (See Object).
    pub objects: Vec<Object>,
    /// Vector with all pictures (See Picture).
    pub pictures: Vec<Picture>,
    /// Vector of Top10 single-player names and times.
    pub top10_single: Vec<ListEntry>,
    /// Vector of Top10 multi-player names and times.
    pub top10_multi: Vec<ListEntry>
}

impl Default for Level {
    fn default() -> Level { Level::new() }
}

impl Level {
    /// Returns a new Level struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let level = elma::lev::Level::new();
    /// ```
    pub fn new () -> Level {
        Level {
            version: Version::Elma,
            raw: vec![],
            link: 0,
            integrity: [0.0f64; 4],
            name: String::new(),
            lgr: String::from("default"),
            ground: String::from("ground"),
            sky: String::from("sky"),
            polygons: vec![],
            objects: vec![],
            pictures: vec![],
            top10_single: vec![],
            top10_multi: vec![]
        }
    }

    /// Loads a level file and returns a Level struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let level = elma::lev::Level::load_level("tests/test.lev");
    /// ```
    pub fn load_level (filename: &str) -> Level {
        let mut level = Level::new();
        let mut file = File::open(filename).unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        level.raw = buffer;
        level.parse_level();
        level
    }

    /// Parses the raw binary data into Level struct fields.
    fn parse_level (&mut self) {
        let rem = self.raw.as_mut_slice();

        // POT06 = Across, POT14 = Elma.
        let (version, rem) = rem.split_at_mut(5);
        self.version = match version {
            [80, 79, 84, 49, 52] => Version::Elma,
            [80, 79, 84, 48, 54] => Version::Across,
            _ => panic!("Not a valid level file.")
        };

        // Link.
        let (_, mut rem) = rem.split_at_mut(2); // Never used
        self.link = (&*rem).read_i32::<LittleEndian>().unwrap();

        // Integrity checksums.
        for i in 0..4 {
            self.integrity[i] = (&*rem).read_f64::<LittleEndian>().unwrap();
        }

        // Level name.
        let (name, rem) = rem.split_at_mut(51);
        self.name = trim_string(name).unwrap();
        // LGR name.
        let (lgr, rem) = rem.split_at_mut(16);
        self.lgr = trim_string(lgr).unwrap();
        // Ground texture name.
        let (ground, rem) = rem.split_at_mut(10);
        self.ground = trim_string(ground).unwrap();
        // Sky texture name.
        let (sky, mut rem) = rem.split_at_mut(10);
        self.sky = trim_string(sky).unwrap();

        // Polygons.
        let poly_count = ((&*rem).read_f64::<LittleEndian>().unwrap() - 0.4643643).round() as usize;
        for _ in 0..poly_count {
            let grass = (&*rem).read_i32::<LittleEndian>().unwrap() > 0;
            let vertex_count = (&*rem).read_i32::<LittleEndian>().unwrap();
            let mut vertices: Vec<Position<f64>> = vec![];
            for _ in 0..vertex_count {
                let x = (&*rem).read_f64::<LittleEndian>().unwrap();
                let y = (&*rem).read_f64::<LittleEndian>().unwrap();
                vertices.push(Position {
                    x: x,
                    y: y
                });
            }
            self.polygons.push(Polygon {
                grass: grass,
                vertices: vertices
            });
        }

        // Objects.
        let object_count = ((&*rem).read_f64::<LittleEndian>().unwrap() - 0.4643643).round() as usize;
        for _ in 0..object_count {
            let x = (&*rem).read_f64::<LittleEndian>().unwrap();
            let y = (&*rem).read_f64::<LittleEndian>().unwrap();
            let position = Position { x: x, y: y };
            let object_type = match (&*rem).read_i32::<LittleEndian>().unwrap() {
                1 => ObjectType::Exit,
                2 => ObjectType::Apple,
                3 => ObjectType::Killer,
                4 => ObjectType::Player,
                _ => panic!("Not a valid object type")
            };
            let gravity = (&*rem).read_i32::<LittleEndian>().unwrap();
            let animation = (&*rem).read_i32::<LittleEndian>().unwrap() + 1;

            self.objects.push(Object {
                position: position,
                object_type: object_type,
                gravity: gravity,
                animation: animation
            });
        }

        // Pictures.
        let picture_count = ((&*rem).read_f64::<LittleEndian>().unwrap() - 0.2345672).round() as usize;
        for _ in 0..picture_count {
            let (name, temp_rem) = rem.split_at_mut(10);
            let name = trim_string(name).unwrap();
            let (texture, temp_rem) = temp_rem.split_at_mut(10);
            let texture = trim_string(texture).unwrap();
            let (mask, temp_rem) = temp_rem.split_at_mut(10);
            rem = temp_rem;
            let mask = trim_string(mask).unwrap();
            let x = (&*rem).read_f64::<LittleEndian>().unwrap();
            let y = (&*rem).read_f64::<LittleEndian>().unwrap();
            let distance = (&*rem).read_i32::<LittleEndian>().unwrap();
            let clip = (&*rem).read_i32::<LittleEndian>().unwrap();

            self.pictures.push(Picture {
                name: name,
                texture: texture,
                mask: mask,
                position: Position { x: x, y: y },
                distance: distance,
                clip: clip
            });
        }

        // EOD marker expected at this point.
        let expected = (&*rem).read_i32::<LittleEndian>().unwrap();
        if expected != EOD { panic!("EOD marker mismatch: x0{:x} != x0{:x}", expected, EOD); }

        // First decrypt the top10 blocks.
        let (top10, rem) = rem.split_at_mut(688);
        let decrypted_top10_data = crypt_top10(top10);

        // Single-player list.
        let single = &decrypted_top10_data[0..344];
        self.top10_single = parse_top10(single);

        // Multi-player list.
        let multi = &decrypted_top10_data[344..688];
        self.top10_multi = parse_top10(multi);

        // EOF marker expected at this point.
        let expected = (&*rem).read_i32::<LittleEndian>().unwrap();
        if expected != EOF { panic!("EOF marker mismatch: x0{:x} != x0{:x}", expected, EOF); }
    }

    /// Combines the Level struct fields to generate the raw binary data,
    /// and calculate integrity sums.
    fn update (&self) {
        // TODO: convert
        self.calculate_integrity_sums();
    }

    fn calculate_integrity_sums (&self) {
        // TODO: do that
    }

    /// Converts all struct fields into raw binary form and returns it.
    pub fn get_raw (self) -> Vec<u8> {
        self.update();
        self.raw
    }

    /// Saves level as a file.
    pub fn save_lev (self, filename: &str) {
        self.update();
        // let file = File::create(&filename).unwrap();
        // TODO: write stuff.
    }
}

/// Decrypt and encrypt top10 list data. Same algorithm for both.
pub fn crypt_top10 (top10: &mut [u8]) -> &[u8] {
    // Who knows
    let mut ebp8: i16 = 0x15;
    let mut ebp10: i16 = 0x2637;

    for mut t in top10.iter_mut().take(688) {
        *t ^= (ebp8 & 0xFF) as u8;
        ebp10 = ebp10.wrapping_add((ebp8.wrapping_rem(0xD3D)).wrapping_mul(0xD3D));
        ebp8 = ebp10.wrapping_mul(0x1F).wrapping_add(0xD3D);
    }

    top10
}

/// Parse top10 lists and return a vector of `ListEntry`s
pub fn parse_top10 (top10: &[u8]) -> Vec<ListEntry> {
    let mut list: Vec<ListEntry> = vec![];
    let times = LittleEndian::read_i32(&top10[0..4]);
    for n in 0..times {
        let time_offset = (4 + n * 4) as usize;
        let time_end = time_offset + 4;
        let name_1_offset = (44 + n * 15) as usize;
        let name_1_end = name_1_offset + 15;
        let name_2_offset = (194 + n * 15) as usize;
        let name_2_end = name_2_offset + 15;
        // All of this pains me even though I don't understand it...
        let name_1 = &top10[name_1_offset..name_1_end];
        let name_2 = &top10[name_2_offset..name_2_end];
        list.push(ListEntry {
            time: LittleEndian::read_i32(&top10[time_offset..time_end]),
            name_1: trim_string(name_1).unwrap(),
            name_2: trim_string(name_2).unwrap()
        });
    }
    list
}
