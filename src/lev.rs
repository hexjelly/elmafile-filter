/// Read and write Elasto Mania level files.

use std::io::{ Read, Write };
use std::fs::File;
use std::path::Path;
use byteorder::{ ByteOrder, ReadBytesExt, WriteBytesExt, LittleEndian };
use rand::random;
use super::{ Position, trim_string, string_null_pad, EOD, EOF, EMPTY_TOP10, ElmaError };

/// Game version.
#[derive(Debug, PartialEq)]
pub enum Version {
    Across,
    Elma
}

impl Default for Version {
    fn default() -> Version { Version::Elma }
}

/// Type of object.
#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Apple { gravity: Direction, animation: u8 },
    Exit,
    Killer,
    Player
}

impl Default for ObjectType {
    fn default() -> ObjectType { ObjectType::Apple { gravity: Direction::default(), animation: 1 } }
}

/// Apple direction object.
#[derive(Debug, PartialEq)]
pub enum Direction {
    Normal,
    Up,
    Down,
    Left,
    Right
}

impl Default for Direction {
    fn default() -> Direction { Direction::Normal }
}

/// Object struct. Every level requires one `ObjectType::Player` Object and at least one `ObjectType::Exit` Object.
#[derive(Debug, Default, PartialEq)]
pub struct Object {
    /// Position. See `Position` struct.
    pub position: Position<f64>,
    /// Type of Object, see `ObjectType`.
    pub object_type: ObjectType
}

/// Polygon struct.
#[derive(Debug, Default, PartialEq)]
pub struct Polygon {
    /// Grass polygon.
    pub grass: bool,
    /// Vector with all vertices, see `Position` struct.
    pub vertices: Vec<Position<f64>>
}

impl Polygon {
    pub fn new () -> Self {
        Polygon {
            grass: false,
            vertices: vec![]
        }
    }
}

/// Picture clipping.
#[derive(Debug, PartialEq)]
pub enum Clip {
    Unclipped,
    Ground,
    Sky
}

impl Default for Clip {
    fn default() -> Clip { Clip::Ground }
}

/// Picture struct.
#[derive(Debug, Default, PartialEq)]
pub struct Picture {
    /// Picture name.
    pub name: String,
    /// Texture name.
    pub texture: String,
    /// Mask name.
    pub mask: String,
    /// Position. See `Position` struct.
    pub position: Position<f64>,
    /// Z-distance
    pub distance: i32,
    /// Clipping.
    pub clip: Clip
}

/// Top10 list entry struct.
#[derive(Debug, Default, PartialEq)]
pub struct ListEntry {
    /// Player 1 name.
    pub name_1: String,
    /// Player 2 name.
    pub name_2: String,
    /// Time.
    pub time: i32
}

/// Level struct that contains all level information.
#[derive(Debug, PartialEq)]
pub struct Level {
    /// Raw binary data of a loaded or finalized constructed level.
    pub raw: Vec<u8>,
    /// Elma or Across level.
    pub version: Version,
    /// Random number that links level file to replay files.
    pub link: i32,
    /// Contains four integrity checks.
    pub integrity: [f64; 4],
    /// Level name.
    pub name: String,
    /// LGR file name.
    pub lgr: String,
    /// Ground texture name.
    pub ground: String,
    /// Sky texture name.
    pub sky: String,
    /// Vector with all polygons (See `Polygon`).
    pub polygons: Vec<Polygon>,
    /// Vector with all objects (See `Object`).
    pub objects: Vec<Object>,
    /// Vector with all pictures (See `Picture`).
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
    /// Returns a new `Level` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let level = elma::lev::Level::new();
    /// ```
    pub fn new () -> Self {
        Level {
            raw: vec![],
            version: Version::Elma,
            link: 0,
            integrity: [0f64; 4],
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

    /// Loads a level file and returns a `Level` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let level = elma::lev::Level::load("tests/test_1.lev").unwrap();
    /// ```
    pub fn load (filename: &str) -> Result<Self, ElmaError> {
        let path = Path::new(&filename);
        let mut level = Level::new();
        let mut file = try!(File::open(path));
        let mut buffer = vec![];
        try!(file.read_to_end(&mut buffer));
        level.raw = buffer;
        try!(level.parse_level());
        Ok(level)
    }

    /// Parses the raw binary data into `Level` struct fields.
    fn parse_level (&mut self) -> Result<(), ElmaError> {
        let remaining = self.raw.as_slice();

        // POT06 = Across, POT14 = Elma.
        let (version, remaining) = remaining.split_at(5);
        self.version = match version {
            [80, 79, 84, 49, 52] => Version::Elma,
            [80, 79, 84, 48, 54] => return Err(ElmaError::AcrossUnsupported),
            _ => return Err(ElmaError::InvalidLevelFile)
        };

        // Link.
        let (_, mut remaining) = remaining.split_at(2); // Never used
        self.link = try!(remaining.read_i32::<LittleEndian>());

        // Integrity checksums.
        for i in 0..4 {
            self.integrity[i] = try!(remaining.read_f64::<LittleEndian>());
        }

        // Level name.
        let (name, remaining) = remaining.split_at(51);
        self.name = try!(trim_string(name));
        // LGR name.
        let (lgr, remaining) = remaining.split_at(16);
        self.lgr = try!(trim_string(lgr));
        // Ground texture name.
        let (ground, remaining) = remaining.split_at(10);
        self.ground = try!(trim_string(ground));
        // Sky texture name.
        let (sky, mut remaining) = remaining.split_at(10);
        self.sky = try!(trim_string(sky));

        // Polygons.
        let poly_count = (try!(remaining.read_f64::<LittleEndian>()) - 0.4643643).round() as usize;
        let (polygons, read_bytes) = try!(self.parse_polygons(remaining, poly_count));
        self.polygons = polygons;
        let (_, mut remaining) = remaining.split_at(read_bytes);

        // Objects.
        let object_count = (try!(remaining.read_f64::<LittleEndian>()) - 0.4643643).round() as usize;
        let (object_data, mut remaining) = remaining.split_at(object_count*28);
        self.objects = try!(self.parse_objects(object_data, object_count));

        // Pictures.
        let picture_count = (try!(remaining.read_f64::<LittleEndian>()) - 0.2345672).round() as usize;
        let (picture_data, mut remaining) = remaining.split_at(picture_count*54);
        self.pictures = try!(self.parse_pictures(picture_data, picture_count));

        // EOD marker expected at this point.
        let expected = try!(remaining.read_i32::<LittleEndian>());
        if expected != EOD { return Err(ElmaError::EODMismatch) }

        // First decrypt the top10 blocks.
        let (top10, mut remaining) = remaining.split_at(688);
        let decrypted_top10_data = crypt_top10(top10);

        // Single-player list.
        let single = &decrypted_top10_data[0..344];
        self.top10_single = try!(parse_top10(single));

        // Multi-player list.
        let multi = &decrypted_top10_data[344..688];
        self.top10_multi = try!(parse_top10(multi));

        // EOF marker expected at this point.
        let expected = try!(remaining.read_i32::<LittleEndian>());
        if expected != EOF { return Err(ElmaError::EOFMismatch) }

        Ok(())
    }

    fn parse_polygons (&self, mut buffer: &[u8], n: usize) -> Result<(Vec<Polygon>, usize), ElmaError> {
        let mut polygons = vec![];
        let mut read_bytes = 0;
        for _ in 0..n {
            read_bytes += 8;
            let grass = try!(buffer.read_i32::<LittleEndian>()) > 0;
            let vertex_count = try!(buffer.read_i32::<LittleEndian>());
            let mut vertices: Vec<Position<f64>> = vec![];
            for _ in 0..vertex_count {
                read_bytes += 16;
                let x = try!(buffer.read_f64::<LittleEndian>());
                let y = try!(buffer.read_f64::<LittleEndian>());
                vertices.push(Position {
                    x: x,
                    y: y
                });
            }
            polygons.push(Polygon {
                grass: grass,
                vertices: vertices
            });
        }
        Ok((polygons, read_bytes))
    }

    fn parse_objects (&self, mut buffer: &[u8], n: usize) -> Result<Vec<Object>, ElmaError> {
        let mut objects = vec![];
        for _ in 0..n {
            let x = try!(buffer.read_f64::<LittleEndian>());
            let y = try!(buffer.read_f64::<LittleEndian>());
            let position = Position { x: x, y: y };
            let object_type = try!(buffer.read_i32::<LittleEndian>());
            let gravity = try!(buffer.read_i32::<LittleEndian>());
            let gravity_direction = match gravity {
                0 => Direction::Normal,
                1 => Direction::Up,
                2 => Direction::Down,
                3 => Direction::Left,
                4 => Direction::Right,
                _ => return Err(ElmaError::InvalidGravity)
            };
            let animation = (try!(buffer.read_i32::<LittleEndian>()) + 1) as u8;
            let object = match object_type {
                1 => ObjectType::Exit,
                2 => ObjectType::Apple { gravity: gravity_direction, animation: animation },
                3 => ObjectType::Killer,
                4 => ObjectType::Player,
                _ => return Err(ElmaError::InvalidObject)
            };

            objects.push(Object {
                position: position,
                object_type: object
            });
        }
        Ok(objects)
    }

    fn parse_pictures (&self, mut buffer: &[u8], n: usize) -> Result<Vec<Picture>, ElmaError> {
        let mut pictures = vec![];
        for _ in 0..n {
            let (name, temp_remaining) = buffer.split_at(10);
            let name = try!(trim_string(name));
            let (texture, temp_remaining) = temp_remaining.split_at(10);
            let texture = try!(trim_string(texture));
            let (mask, temp_remaining) = temp_remaining.split_at(10);
            let mask = try!(trim_string(mask));
            buffer = temp_remaining;
            let x = try!(buffer.read_f64::<LittleEndian>());
            let y = try!(buffer.read_f64::<LittleEndian>());
            let distance = try!(buffer.read_i32::<LittleEndian>());
            let clipping = try!(buffer.read_i32::<LittleEndian>());
            let clip = match clipping {
                0 => Clip::Unclipped,
                1 => Clip::Ground,
                2 => Clip::Sky,
                _ => return Err(ElmaError::InvalidClipping)
            };

            pictures.push(Picture {
                name: name,
                texture: texture,
                mask: mask,
                position: Position { x: x, y: y },
                distance: distance,
                clip: clip
            });
        }
        Ok(pictures)
    }

    /// Combines the `Level` struct fields to generate the raw binary data, and calculates
    /// integrity sums. Called automatically when using `get_raw`, `get_raw_with_top_10`, `save` or
    /// `save_with_top_10` methods, and is provided mainly for convinience if you need to use it
    /// manually.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut level = elma::lev::Level::load("tests/test_1.lev").unwrap();
    /// level.update(false);
    /// ```
    pub fn update (&mut self, top_10: bool) -> Result<(), ElmaError> {
        let mut bytes = vec![];

        // Level version.
        match self.version {
            Version::Elma => bytes.extend_from_slice(&[80, 79, 84, 49, 52]),
            Version::Across => return Err(ElmaError::AcrossUnsupported)
        };

        // Lower short of link.
        try!(bytes.write_i16::<LittleEndian>((self.link & 0xFFFF) as i16));
        // Link.
        try!(bytes.write_i32::<LittleEndian>(self.link));
        // Integrity checksums.
        self.calculate_integrity_sums(true);
        for sum in self.integrity.into_iter() {
            try!(bytes.write_f64::<LittleEndian>(*sum));
        }

        // Level name.
        bytes.extend_from_slice(&string_null_pad(&self.name, 51));
        // LGR name.
        bytes.extend_from_slice(&string_null_pad(&self.lgr, 16));
        // Ground name.
        bytes.extend_from_slice(&string_null_pad(&self.ground, 10));
        // Sky name.
        bytes.extend_from_slice(&string_null_pad(&self.sky, 10));

        // Number of polygons.
        try!(bytes.write_f64::<LittleEndian>(self.polygons.len() as f64 + 0.4643643_f64));
        // Polygons.
        bytes = try!(self.write_polygons(bytes));

        // Number of objects.
        try!(bytes.write_f64::<LittleEndian>(self.objects.len() as f64 + 0.4643643_f64));
        // Objects.
        bytes = try!(self.write_objects(bytes));

        // Number of pictures.
        try!(bytes.write_f64::<LittleEndian>(self.pictures.len() as f64 + 0.2345672_f64));
        // Pictures.
        bytes = try!(self.write_pictures(bytes));

        // EOD marker.
        try!(bytes.write_i32::<LittleEndian>(EOD));

        // Top10 lists.
        // TODO: figure out how to write this to be less idiotic.
        if top_10 {
            bytes = try!(self.write_top10(bytes));
        } else {
            bytes.extend_from_slice(&EMPTY_TOP10);
        }

        // EOF marker.
        try!(bytes.write_i32::<LittleEndian>(EOF));

        self.raw = bytes;
        Ok(())
    }

    fn write_polygons (&self, mut bytes: Vec<u8>) -> Result<Vec<u8>, ElmaError> {
        for poly in &self.polygons {
            // Grass poly.
            try!(bytes.write_i32::<LittleEndian>(if poly.grass { 1 } else { 0 }));
            // Number of vertices.
            try!(bytes.write_i32::<LittleEndian>(poly.vertices.len() as i32));
            // Vertices.
            for vertex in &poly.vertices {
                try!(bytes.write_f64::<LittleEndian>(vertex.x));
                try!(bytes.write_f64::<LittleEndian>(vertex.y));
            }
        }
        Ok(bytes)
    }

    fn write_objects (&self, mut bytes: Vec<u8>) -> Result<Vec<u8>, ElmaError> {
        for obj in &self.objects {
            // Position.
            try!(bytes.write_f64::<LittleEndian>(obj.position.x));
            try!(bytes.write_f64::<LittleEndian>(obj.position.y));
            // Object type.
            try!(bytes.write_i32::<LittleEndian>(match obj.object_type {
                ObjectType::Exit => 1,
                ObjectType::Apple { .. } => 2,
                ObjectType::Killer => 3,
                ObjectType::Player => 4
            }));
            // Apple gravity.
            try!(bytes.write_i32::<LittleEndian>(match obj.object_type {
                ObjectType::Apple { gravity: Direction::Up, .. } => 1,
                ObjectType::Apple { gravity: Direction::Down, .. } => 2,
                ObjectType::Apple { gravity: Direction::Left, .. } => 3,
                ObjectType::Apple { gravity: Direction::Right, .. } => 4,
                _ => 0
            }));
            // Apple animation.
            try!(bytes.write_i32::<LittleEndian>(match obj.object_type {
                ObjectType::Apple { animation: n, .. } => (n - 1) as i32,
                _ => 0
            }));
        }
        Ok(bytes)
    }

    fn write_pictures (&self, mut bytes: Vec<u8>) -> Result<Vec<u8>, ElmaError> {
        for pic in &self.pictures {
            // Picture name.
            bytes.extend_from_slice(&string_null_pad(&pic.name, 10));
            // Texture name.
            bytes.extend_from_slice(&string_null_pad(&pic.texture, 10));
            // Mask name.
            bytes.extend_from_slice(&string_null_pad(&pic.mask, 10));
            // Position.
            try!(bytes.write_f64::<LittleEndian>(pic.position.x));
            try!(bytes.write_f64::<LittleEndian>(pic.position.y));
            // Z-distance.
            try!(bytes.write_i32::<LittleEndian>(pic.distance));
            // Clipping.
            try!(bytes.write_i32::<LittleEndian>(match pic.clip {
                Clip::Unclipped => 0,
                Clip::Ground => 1,
                Clip::Sky => 2
            }));
        }
        Ok(bytes)
    }

    fn write_top10 (&self, mut bytes: Vec<u8>) -> Result<Vec<u8>, ElmaError> {
        let mut top10_bytes: Vec<u8> = vec![];

        // Single-player times.
        try!(top10_bytes.write_i32::<LittleEndian>(self.top10_single.len() as i32));
        let mut times = [0_i32;10];
        let mut names_1: Vec<u8> = vec![];
        let mut names_2: Vec<u8> = vec![];
        for (n, entry) in self.top10_single.iter().enumerate() {
            times[n] = entry.time;
            names_1.extend_from_slice(&string_null_pad(&entry.name_1, 15));
            names_2.extend_from_slice(&string_null_pad(&entry.name_2, 15));
        }
        // Pad with null bytes if less than 10 entries.
        for _ in 0..10 - self.top10_single.len() {
            names_1.extend_from_slice(&[0u8;15]);
            names_2.extend_from_slice(&[0u8;15]);
        }

        for time in &times {
            try!(top10_bytes.write_i32::<LittleEndian>(*time));
        }

        for _ in 0..10 - times.len() {
            try!(top10_bytes.write_i32::<LittleEndian>(0));
        }

        top10_bytes.extend_from_slice(&names_1);
        top10_bytes.extend_from_slice(&names_2);

        // Multi-player times.
        try!(top10_bytes.write_i32::<LittleEndian>(self.top10_multi.len() as i32));
        let mut times = [0_i32;10];
        let mut names_1: Vec<u8> = vec![];
        let mut names_2: Vec<u8> = vec![];
        for (n, entry) in self.top10_multi.iter().enumerate() {
            times[n] = entry.time;
            names_1.extend_from_slice(&string_null_pad(&entry.name_1, 15));
            names_2.extend_from_slice(&string_null_pad(&entry.name_2, 15));
        }
        // Pad with null bytes if less than 10 entries.
        for _ in 0..10 - self.top10_multi.len() {
            names_1.extend_from_slice(&[0u8;15]);
            names_2.extend_from_slice(&[0u8;15]);
        }

        for time in &times {
            try!(top10_bytes.write_i32::<LittleEndian>(*time));
        }

        for _ in 0..10 - times.len() {
            try!(top10_bytes.write_i32::<LittleEndian>(0));
        }

        top10_bytes.extend_from_slice(&names_1);
        top10_bytes.extend_from_slice(&names_2);

        // Encrypt the data before writing.
        bytes.extend_from_slice(&crypt_top10(&top10_bytes));
        Ok(bytes)
    }

    /// Check topology of level.
    // TODO: make this return a Result with problematic polygons/vertices.
    pub fn topology_check (&self) -> bool {
        // TODO: check max polygons
        // TODO: check max objects
        // TODO: check max pictures
        // TODO: check line segment overlaps
        // TODO: check if player and at least one exit object
        // TODO: check if head inside ground
        unimplemented!();
    }

    /// Calculate integrity sums for level.
    fn calculate_integrity_sums (&mut self, valid_topology: bool) {
        let mut pol_sum = 0_f64;
        let mut obj_sum = 0_f64;
        let mut pic_sum = 0_f64;

        for poly in &self.polygons {
            for vertex in &poly.vertices {
                pol_sum += vertex.x + vertex.y;
            }
        }

        for obj in &self.objects {
            let obj_type = match obj.object_type {
                ObjectType::Exit => 1,
                ObjectType::Apple { .. } => 2,
                ObjectType::Killer => 3,
                ObjectType::Player => 4
            };
            obj_sum += obj.position.x + obj.position.y + (obj_type as f64);
        }

        for pic in &self.pictures {
            pic_sum += pic.position.x + pic.position.y;
        }

        let sum = (pol_sum + obj_sum + pic_sum) * 3247.764325643;
        self.integrity[0] = sum;
        self.integrity[1] = (random::<u32>() % 5871) as f64 + 11877. - sum;
        if valid_topology {
            self.integrity[2] = (random::<u32>() % 5871) as f64 + 11877. - sum;
        } else {
            self.integrity[2] = (random::<u32>() % 4982) as f64 + 20961. - sum;
        }
        self.integrity[3] = (random::<u32>() % 6102) as f64 + 12112. - sum;
    }

    /// Converts all struct fields into raw binary form and returns the raw data.
    pub fn get_raw (&mut self, top10: bool) -> Result<Vec<u8>, ElmaError> {
        try!(self.update(top10));
        Ok(self.raw.clone())
    }

    /// Saves level as a file.
    pub fn save (&mut self, filename: &str, top10: bool) -> Result<(), ElmaError> {
        let path = Path::new(&filename);
        try!(self.update(top10));
        let mut file = try!(File::create(path));
        try!(file.write_all(&self.raw));
        Ok(())
    }
}

/// Decrypt and encrypt top10 list data. Same algorithm for both.
pub fn crypt_top10 (top10_data: &[u8]) -> Vec<u8> {
    let mut top10: Vec<u8> = Vec::with_capacity(688);
    top10.extend_from_slice(top10_data);

    // Some variable names to match the original c/asm code?
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
pub fn parse_top10 (top10: &[u8]) -> Result<Vec<ListEntry>, ElmaError> {
    let mut list: Vec<ListEntry> = vec![];
    let times = LittleEndian::read_i32(&top10[0..4]);
    for n in 0..times {
        let time_offset = (4 + n * 4) as usize;
        let time_end = time_offset + 4;
        let name_1_offset = (44 + n * 15) as usize;
        let name_1_end = name_1_offset + 15;
        let name_2_offset = (194 + n * 15) as usize;
        let name_2_end = name_2_offset + 15;

        let name_1 = &top10[name_1_offset..name_1_end];
        let name_2 = &top10[name_2_offset..name_2_end];
        let time = &top10[time_offset..time_end];
        list.push(ListEntry {
            time: LittleEndian::read_i32(time),
            name_1: try!(trim_string(name_1)),
            name_2: try!(trim_string(name_2))
        });
    }
    Ok(list)
}
