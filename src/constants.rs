/// Diameter of player head.
pub const HEAD_DIAMETER: f64 = 0.476;
/// Radius of player head.
pub const HEAD_RADIUS: f64 = 0.238;
/// Diameter of objects (and wheels).
pub const OBJECT_DIAMETER: f64 = 0.8;
/// Radius of objects (and wheels).
pub const OBJECT_RADIUS: f64 = 0.4;
/// Magic arbitrary number signifying end-of-data in level file.
pub const EOD: i32 = 0x00_67_10_3A;
/// Magic arbitrary number signifying end-of-file in level file.
pub const EOF: i32 = 0x00_84_5D_52;
/// Magic arbitrary number to signify end of replay file.
pub const EOR: i32 = 0x00_49_2F_75;
/// Magic arbitrary number to signify start of state.dat file.
pub const STATE: i32 = 0x2E_78_40_DF;
/// Magic arbitrary number to signify start of LGR file.
pub const LGR: i32 = 0x00_00_03_EA;
/// Magic arbitrary number to signify end of LGR file.
pub const LGR_EOF: i32 = 0x0B_2E_05_E7;
/// Size of top10 data for a player.
pub const PLAYER_TOP10_SIZE: usize = 344;
/// Size of top10 data for a level.
pub const TOP10_SIZE: usize = PLAYER_TOP10_SIZE * 2;
