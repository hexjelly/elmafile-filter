//! Read and write Elasto Mania replay files.
use std::io::{ Read, Write };
use std::fs::File;
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use super::{ Position, trim_string };

// Magic arbitrary number to signify end of replay file.
const EOR: i32 = 0x00492F75;

/// One frame of replay.
#[derive(Debug, Default, PartialEq)]
pub struct Frame {
    /// Bike position?
    pub bike: Position<f32>,
    /// Left wheel position.
    pub left_wheel: Position<i16>,
    /// Right wheel position.
    pub right_wheel: Position<i16>,
    /// Head position.
    pub head: Position<i16>,
    /// Bike rotation. Range 0..10000.
    pub rotation: i16,
    /// Left wheel rotation. Range 0..255.
    pub left_wheel_rotation: u8,
    /// Right wheel rotation. Range 0..255.
    pub right_wheel_rotation: u8,
    /// Throttle.
    pub throttle: bool,
    /// Right direction. True = right, False = left.
    // TODO: consider making right field = direction and enum with right and left?
    pub right: bool,
    /// Spring sound effect volume.
    pub volume: i16
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            bike: Position { x: 0f32, y: 0f32 },
            left_wheel: Position { x: 0, y: 0 },
            right_wheel: Position { x: 0, y: 0 },
            head: Position { x: 0, y: 0 },
            rotation: 0,
            left_wheel_rotation: 0,
            right_wheel_rotation: 0,
            throttle: false,
            right: false,
            volume: 0
        }
    }
}


#[derive(Debug, Default, PartialEq)]
pub struct Event {
    /// Time of event.
    pub time: f64,
    /// Event type.
    // TODO: Make enum.
    pub event_type: [u32; 2]
}

impl Event {
    pub fn new() -> Self {
        Event {
            time: 0f64,
            event_type: [0, 0]
        }
    }
}

/// Replay struct
#[derive(Debug, PartialEq)]
pub struct Replay {
    /// Raw binary data.
    raw: Vec<u8>,
    /// Whether replay is multi-player or not.
    pub multi: bool,
    /// Whether replay is flag-tag or not.
    pub flag_tag: bool,
    /// Random number to link with level file.
    pub link: u32,
    /// Full level filename.
    pub level: String,
    /// Vector with Frame structs.
    pub frames: Vec<Frame>,
    /// Events.
    pub events: Vec<Event>
}

impl Default for Replay {
    fn default() -> Replay { Replay::new() }
}

impl Replay {
    /// Build a new Replay.
    ///
    /// # Examples
    ///
    /// ```
    /// let rec = elma::rec::Replay::new();
    /// ```
    pub fn new() -> Self {
        Replay {
            raw: vec![],
            multi: false,
            flag_tag: false,
            link: 0,
            level: String::new(),
            frames: vec![],
            events: vec![]
        }
    }

    /// Loads a replay file and returns a Replay struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let rec = elma::rec::Replay::load_replay("tests/test.rec");
    /// ```
    pub fn load_replay(filename: &str) -> Self {
        let mut replay = Replay::new();
        let mut file = File::open(filename).unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        replay.raw = buffer;
        replay.parse_replay();
        replay
    }

    pub fn parse_replay(&mut self) {
        // TODO: do that.
        let mut remaining = self.raw.as_slice();

        // Frame count.
        let frame_count = remaining.read_i32::<LittleEndian>().unwrap();

        // Some unused value, always 0x83.
        let (_, mut remaining) = remaining.split_at(4);

        // Multi-player replay.
        self.multi = remaining.read_i32::<LittleEndian>().unwrap() > 0;

        // Flag-tag replay.
        self.flag_tag = remaining.read_i32::<LittleEndian>().unwrap() > 0;

        // Level link.
        self.link = remaining.read_u32::<LittleEndian>().unwrap();

        // Level file name, including extension.
        let (level, remaining) = remaining.split_at(12);
        self.level = trim_string(level).unwrap();

        // Unknown, unused.
        let (_, mut remaining) = remaining.split_at(4);

        // Frames.
        for _ in 0..frame_count {
            // TODO: parse this
            let (_, temp_remaining) = remaining.split_at(27);
            remaining = temp_remaining;
        }

        // Events.
        let event_count = remaining.read_i32::<LittleEndian>().unwrap();
        for _ in 0..event_count {
            // TODO: parse this
            let (_, temp_remaining) = remaining.split_at(16);
            remaining = temp_remaining;
        }

        let expected = remaining.read_i32::<LittleEndian>().unwrap();
        if expected != EOR { panic!("EOR marker mismatch: x0{:x} != x0{:x}", expected, EOR); }

        // TODO: Add multi-player replay parsing.
    }
}
