//! Read and write Elasto Mania replay files.
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use byteorder::{ ReadBytesExt, LittleEndian };
use super::{ Position, trim_string, EOR, ElmaError };

/// One frame of replay.
#[derive(Debug, Default, PartialEq)]
pub struct Frame {
    /// Bike position.
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
    /// Returns a new Frame struct with zero-filled values.
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = elma::rec::Frame::new();
    /// ```
    pub fn new() -> Self {
        Frame {
            bike: Position { x: 0_f32, y: 0_f32 },
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
/// Replay events.
pub struct Event {
    /// Time of event.
    pub time: f64,
    /// Event type.
    pub event_type: EventType
}

#[derive(Debug, PartialEq)]
/// Type of event.
pub enum EventType {
    /// Apple or flower touch, with index of object.
    Touch { index: i16 },
    Turn,
    VoltRight,
    VoltLeft,
    /// Ground touch, for sound effects. Two types; if alternative is true, uses the second type.
    Ground { alternative: bool }
}

impl Default for EventType {
    fn default() -> EventType { EventType::Touch { index: 0 } }
}

impl Event {
    pub fn new() -> Self {
        Event {
            time: 0_f64,
            event_type: EventType::default()
        }
    }
}

/// Replay struct
#[derive(Debug, PartialEq)]
pub struct Replay {
    /// Raw binary data.
    pub raw: Vec<u8>,
    /// Whether replay is multi-player or not.
    pub multi: bool,
    /// Whether replay is flag-tag or not.
    pub flag_tag: bool,
    /// Random number to link with level file.
    pub link: u32,
    /// Full level filename.
    pub level: String,
    /// Player one frames.
    pub frames: Vec<Frame>,
    /// Player one events.
    pub events: Vec<Event>,
    /// Player two frames.
    pub frames_2: Vec<Frame>,
    /// Player two events.
    pub events_2: Vec<Event>
}

impl Default for Replay {
    fn default() -> Replay { Replay::new() }
}

impl Replay {
    /// Return a new Replay struct.
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
            events: vec![],
            frames_2: vec![],
            events_2: vec![]
        }
    }

    /// Loads a replay file and returns a Replay struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let rec = elma::rec::Replay::load("tests/test_1.rec").unwrap();
    /// ```
    pub fn load(filename: &str) -> Result<Self, ElmaError> {
        let path = Path::new(&filename);
        let mut replay = Replay::new();
        let mut file = try!(File::open(path));
        let mut buffer = vec![];
        try!(file.read_to_end(&mut buffer));
        replay.raw = buffer;
        try!(replay.parse_replay());
        Ok(replay)
    }

    /// Parses the raw binary data into Replay struct fields.
    fn parse_replay(&mut self) -> Result<(), ElmaError> {
        let mut remaining = self.raw.as_slice();

        // Frame count.
        let frame_count = try!(remaining.read_i32::<LittleEndian>());
        // Some unused value, always 0x83.
        let (_, mut remaining) = remaining.split_at(4);
        // Multi-player replay.
        self.multi = try!(remaining.read_i32::<LittleEndian>()) > 0;
        // Flag-tag replay.
        self.flag_tag = try!(remaining.read_i32::<LittleEndian>()) > 0;
        // Level link.
        self.link = try!(remaining.read_u32::<LittleEndian>());
        // Level file name, including extension.
        let (level, remaining) = remaining.split_at(12);
        self.level = try!(trim_string(level));
        // Unknown, unused.
        let (_, remaining) = remaining.split_at(4);
        // Frames.
        self.frames = try!(parse_frames(remaining, frame_count));
        let (_, mut remaining) = remaining.split_at(27*frame_count as usize);
        // Events.
        let event_count = try!(remaining.read_i32::<LittleEndian>());
        self.events = try!(parse_events(remaining, event_count));
        let (_, mut remaining) = remaining.split_at(16*event_count as usize);
        // End of replay marker.
        let expected = try!(remaining.read_i32::<LittleEndian>());
        if expected != EOR { return Err(ElmaError::EORMismatch); }

        // If multi-rec, parse frame and events, while skipping other fields?
        if self.multi {
            // Frame count.
            let frame_count = try!(remaining.read_i32::<LittleEndian>());
            // Skip other fields.
            let (_, remaining) = remaining.split_at(32);
            // Frames.
            self.frames_2 = try!(parse_frames(remaining, frame_count));
            let (_, mut remaining) = remaining.split_at(27*frame_count as usize);
            // Events.
            let event_count = try!(remaining.read_i32::<LittleEndian>());
            self.events_2 = try!(parse_events(remaining, event_count));
            let (_, mut remaining) = remaining.split_at(16*event_count as usize);
            // End of replay marker.
            let expected = try!(remaining.read_i32::<LittleEndian>());
            if expected != EOR { return Err(ElmaError::EORMismatch); }
        }
        Ok(())
    }

    /// Save replay as a file.
    pub fn save (&self, _filename: &str) {
        unimplemented!();
    }
}

/// Function for parsing frame data from either single-player or multi-player replays.
fn parse_frames (frame_data: &[u8], frame_count: i32) -> Result<Vec<Frame>, ElmaError> {
    let mut frames: Vec<Frame> = vec![];

    let (mut bike_x, remaining) = frame_data.split_at((frame_count*4) as usize);
    let (mut bike_y, remaining) = remaining.split_at((frame_count*4) as usize);
    let (mut left_x, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut left_y, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut right_x, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut right_y, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut head_x, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut head_y, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut rotation, remaining) = remaining.split_at((frame_count*2) as usize);
    let (mut left_rotation, remaining) = remaining.split_at((frame_count) as usize);
    let (mut right_rotation, remaining) = remaining.split_at((frame_count) as usize);
    let (mut data, remaining) = remaining.split_at((frame_count) as usize);
    let (mut volume, _) = remaining.split_at((frame_count*2) as usize);

    for _ in 0..frame_count {
        // Bike X and Y.
        let x = try!(bike_x.read_f32::<LittleEndian>());
        let y = try!(bike_y.read_f32::<LittleEndian>());
        let bike = Position { x: x, y: y };
        // Left wheel X and Y.
        let x = try!(left_x.read_i16::<LittleEndian>());
        let y = try!(left_y.read_i16::<LittleEndian>());
        let left_wheel = Position { x: x, y: y };
        // Left wheel X and Y.
        let x = try!(right_x.read_i16::<LittleEndian>());
        let y = try!(right_y.read_i16::<LittleEndian>());
        let right_wheel = Position { x: x, y: y };
        // Head X and Y.
        let x = try!(head_x.read_i16::<LittleEndian>());
        let y = try!(head_y.read_i16::<LittleEndian>());
        let head = Position { x: x, y: y };
        // Rotations.
        let rotation = try!(rotation.read_i16::<LittleEndian>());
        let left_wheel_rotation = try!(left_rotation.read_u8());
        let right_wheel_rotation = try!(right_rotation.read_u8());
        // Throttle and turn right.
        let data = try!(data.read_u8());
        let throttle = data & 1 != 0;
        let right = data & (1 << 1) != 0;
        // Sound effect volume.
        let volume = try!(volume.read_i16::<LittleEndian>());

        frames.push(Frame {
            bike: bike,
            left_wheel: left_wheel,
            right_wheel: right_wheel,
            head: head,
            rotation: rotation,
            left_wheel_rotation: left_wheel_rotation,
            right_wheel_rotation: right_wheel_rotation,
            throttle: throttle,
            right: right,
            volume: volume
        });
    }

    Ok(frames)
}

/// Function for parsing event data from either single-player or multi-player replays.
fn parse_events (mut event_data: &[u8], event_count: i32) -> Result<Vec<Event>, ElmaError> {
    let mut events: Vec<Event> = vec![];

    for n in 0..event_count {
        // Event time
        let time = try!(event_data.read_f64::<LittleEndian>());
        // Event details
        let info = try!(event_data.read_i16::<LittleEndian>());
        let event = try!(event_data.read_u8());
        // Unknown values
        let _ = try!(event_data.read_u8());
        let _ = try!(event_data.read_f32::<LittleEndian>());
        let event_type = match event {
            0 => EventType::Touch { index: info },
            1 => EventType::Ground { alternative: false },
            5 => EventType::Turn,
            4 => EventType::Ground { alternative: true },
            6 => EventType::VoltRight,
            7 => EventType::VoltLeft,
            _ => return Err(ElmaError::InvalidEvent(n))
        };

        events.push(Event {
            time: time,
            event_type: event_type
        });
    }

    Ok(events)
}
