use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use byteorder::{LittleEndian as LE, WriteBytesExt};

use super::{BestTimes, ElmaError, constants::STATE, utils::{parse_top10, write_top10}};

/// State.dat struct
#[derive(Default, Debug, Clone, PartialEq)]
pub struct State {
    /// TODO: remove when other state.dat parsing done
    buffer: Vec<u8>,
    /// Best times lists.
    pub times: Vec<BestTimes>,
}

impl State {
    /// Create new state.dat
    pub fn new() -> Self {
        State {
            buffer: Vec::with_capacity(67910),
            times: Vec::with_capacity(540),
        }
    }

    /// Load state.dat file
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self, ElmaError> {
        let mut buffer = vec![];
        let mut file = File::open(filename)?;
        file.read_to_end(&mut buffer)?;
        State::parse(&buffer)
    }

    /// Load a state.dat file from bytes.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::state::*;
    /// let state = State::from_bytes(&[0,1,2]).unwrap();
    /// ```
    pub fn from_bytes<B: AsRef<[u8]>>(buffer: B) -> Result<Self, ElmaError> {
        State::parse(buffer.as_ref())
    }

    fn parse(buffer: &[u8]) -> Result<Self, ElmaError> {
        let mut state = State::new();
        state.buffer = buffer.to_vec();
        crypt_state(&mut state.buffer[4..]);
        for n in 1..91 {
            let offset_start = 4 + (688 * (n - 1));
            let offset_end = offset_start + 344;
            let level = BestTimes {
                single: parse_top10(&state.buffer[offset_start..offset_end])?,
                multi: parse_top10(&state.buffer[offset_start + 344..offset_end + 344])?,
            };
            state.times.push(level);
        }
        Ok(state)
    }

    fn serialize(&mut self) -> Result<Vec<u8>, ElmaError> {
        let mut buffer = vec![];
        buffer.write_i32::<LE>(STATE)?;

        for mut level in self.times.iter_mut() {
            // Order lists first.
            level.single.sort();
            level.multi.sort();
            let top10_bytes = write_top10(&level)?;
            buffer.extend_from_slice(&top10_bytes);
        }

        // TODO: fix when understand rest of state.dat
        let idiot = buffer.len();
        buffer.extend_from_slice(&self.buffer[idiot..]);

        Ok(buffer)
    }

    /// Save state.dat
    pub fn save<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), ElmaError> {
        let mut buffer = self.serialize()?;
        crypt_state(&mut buffer[4..]);
        let mut file = File::create(filename)?;
        file.write_all(&buffer)?;
        Ok(())
    }
}

fn crypt_state(buffer: &mut [u8]) {
    let mut ebp8: i16 = 0x17;
    let mut ebp10: i16 = 0x2636;

    for mut t in buffer.iter_mut() {
        *t ^= (ebp8 & 0xFF) as u8;
        ebp10 = ebp10.wrapping_add((ebp8.wrapping_rem(0xD3F)).wrapping_mul(0xD3F));
        ebp8 = ebp10.wrapping_mul(0x1F).wrapping_add(0xD3F);
    }
}
