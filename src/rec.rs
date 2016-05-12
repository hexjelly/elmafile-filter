/// Read and write Elasto Mania replay files.

use std::io::{ Read, Write };
use std::fs::File;
use std::path::Path;
use rand::random;
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use super::{ Position, trim_string, string_null_pad, EOR, ElmaError };

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
    /// Returns a new Frame struct with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// let frame = elma::rec::Frame::new();
    /// ```
    pub fn new() -> Self {
        Frame { bike: Position { x: 0_f32, y: 0_f32 },
                left_wheel: Position { x: 0, y: 0 },
                right_wheel: Position { x: 0, y: 0 },
                head: Position { x: 0, y: 0 },
                rotation: 0,
                left_wheel_rotation: 0,
                right_wheel_rotation: 0,
                throttle: false,
                right: false,
                volume: 0 }
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
    /// Returns a new Event struct with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// let event = elma::rec::Event::new();
    /// ```
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

impl Replay {
    /// Return a new Replay struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let rec = elma::rec::Replay::new();
    /// ```
    pub fn new() -> Self {
        Replay { raw: vec![],
                 multi: false,
                 flag_tag: false,
                 link: 0,
                 level: String::new(),
                 frames: vec![],
                 events: vec![],
                 frames_2: vec![],
                 events_2: vec![] }
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

    fn write_rec (&self, multi: bool) -> Result<Vec<u8>, ElmaError> {
        let mut bytes: Vec<u8> = vec![];

        // Number of frames.
        if multi {
            try!(bytes.write_i32::<LittleEndian>(self.frames_2.len() as i32));
        } else {
            try!(bytes.write_i32::<LittleEndian>(self.frames.len() as i32));
        }
        // Garbage value.
        try!(bytes.write_i32::<LittleEndian>(0x83_i32));
        // Multi-player replay or not.
        try!(bytes.write_i32::<LittleEndian>(match self.multi {
            true => 1_i32,
            false => 0_i32
        }));
        // Flag-tag replay or not.
        try!(bytes.write_i32::<LittleEndian>(match self.flag_tag {
            true => 1_i32,
            false => 0_i32
        }));
        // Link.
        try!(bytes.write_u32::<LittleEndian>(self.link));
        // Level name.
        bytes.extend_from_slice(&string_null_pad(&self.level, 12));
        // Garbage value.
        try!(bytes.write_i32::<LittleEndian>(0x00_i32));

        // Frames and events.
        if multi {
            bytes.extend_from_slice(&try!(write_frames(&self.frames_2)));
            bytes.extend_from_slice(&try!(write_events(&self.events_2)));
        } else {
            bytes.extend_from_slice(&try!(write_frames(&self.frames)));
            bytes.extend_from_slice(&try!(write_events(&self.events)));
        }

        // EOR marker.
        try!(bytes.write_i32::<LittleEndian>(EOR));

        Ok(bytes)
    }

    /// Save replay as a file.
    pub fn save (&self, filename: &str) -> Result<(), ElmaError> {
        let path = Path::new(&filename);
        let mut bytes = try!(self.write_rec(false));
        if self.multi {
            bytes.extend_from_slice(&try!(self.write_rec(true)));
        }
        let mut file = try!(File::create(path));
        try!(file.write_all(&bytes));
        Ok(())
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
        // Right wheel X and Y.
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

    for _ in 0..event_count {
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
            4 => EventType::Ground { alternative: true },
            5 => EventType::Turn,
            6 => EventType::VoltRight,
            7 => EventType::VoltLeft,
            _ => return Err(ElmaError::InvalidEvent(event))
        };

        events.push(Event {
            time: time,
            event_type: event_type
        });
    }

    Ok(events)
}

/// Function for writing frame data.
fn write_frames (frame_data: &[Frame]) -> Result<Vec<u8>, ElmaError> {
    let mut bytes = vec![];

    let mut bike_x = vec![];
    let mut bike_y = vec![];
    let mut left_x = vec![];
    let mut left_y = vec![];
    let mut right_x = vec![];
    let mut right_y = vec![];
    let mut head_x = vec![];
    let mut head_y = vec![];
    let mut rotation = vec![];
    let mut left_rotation = vec![];
    let mut right_rotation = vec![];
    let mut data = vec![];
    let mut volume = vec![];

    for frame in frame_data {
        try!(bike_x.write_f32::<LittleEndian>(frame.bike.x));
        try!(bike_y.write_f32::<LittleEndian>(frame.bike.y));

        try!(left_x.write_i16::<LittleEndian>(frame.left_wheel.x));
        try!(left_y.write_i16::<LittleEndian>(frame.left_wheel.y));

        try!(right_x.write_i16::<LittleEndian>(frame.right_wheel.x));
        try!(right_y.write_i16::<LittleEndian>(frame.right_wheel.y));

        try!(head_x.write_i16::<LittleEndian>(frame.head.x));
        try!(head_y.write_i16::<LittleEndian>(frame.head.y));

        try!(rotation.write_i16::<LittleEndian>(frame.rotation));
        try!(left_rotation.write_u8(frame.left_wheel_rotation));
        try!(right_rotation.write_u8(frame.right_wheel_rotation));

        let mut data_temp = random::<u8>();
        if frame.throttle {
            data_temp |= 1;
        } else {
            data_temp &= !1;
        }
        if frame.right {
            data_temp |= 1 << 1;
        } else {
            data_temp &= !(1 << 1);
        }
        try!(data.write_u8(data_temp));

        try!(volume.write_i16::<LittleEndian>(frame.volume));
    }

    bytes.extend_from_slice(&bike_x);
    bytes.extend_from_slice(&bike_y);
    bytes.extend_from_slice(&left_x);
    bytes.extend_from_slice(&left_y);
    bytes.extend_from_slice(&right_x);
    bytes.extend_from_slice(&right_y);
    bytes.extend_from_slice(&head_x);
    bytes.extend_from_slice(&head_y);
    bytes.extend_from_slice(&rotation);
    bytes.extend_from_slice(&left_rotation);
    bytes.extend_from_slice(&right_rotation);
    bytes.extend_from_slice(&data);
    bytes.extend_from_slice(&volume);

    Ok(bytes)
}

/// Function for writing event data.
fn write_events (event_data: &[Event]) -> Result<Vec<u8>, ElmaError> {
    let mut bytes = vec![];

    // Number of events.
    try!(bytes.write_i32::<LittleEndian>(event_data.len() as i32));

    for event in event_data {
        try!(bytes.write_f64::<LittleEndian>(event.time));
        match event.event_type {
            EventType::Touch { index: info } => { try!(bytes.write_u32::<LittleEndian>(info as u32));
                                                  try!(bytes.write_u32::<LittleEndian>(0 as u32)); },
            EventType::Ground { alternative: false } => { try!(bytes.write_u32::<LittleEndian>(131071 as u32));
                                                          try!(bytes.write_u32::<LittleEndian>(1050605825 as u32)); },
            EventType::Ground { alternative: true } => { try!(bytes.write_u32::<LittleEndian>(327679 as u32));
                                                          try!(bytes.write_u32::<LittleEndian>(1065185444 as u32)); },
            EventType::Turn => { try!(bytes.write_u32::<LittleEndian>(393215 as u32));
                                 try!(bytes.write_u32::<LittleEndian>(1065185444 as u32)); },
            EventType::VoltRight => { try!(bytes.write_u32::<LittleEndian>(458751 as u32));
                                      try!(bytes.write_u32::<LittleEndian>(1065185444 as u32)); },
            EventType::VoltLeft => { try!(bytes.write_u32::<LittleEndian>(524287 as u32));
                                      try!(bytes.write_u32::<LittleEndian>(1065185444 as u32)); }
        }

    }

    Ok(bytes)
}
