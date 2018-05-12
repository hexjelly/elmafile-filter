use byteorder::{WriteBytesExt, LE};
use std::fs;
use std::path::Path;

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
    ///
    /// **WARNING**: currently will not crate a valid state.dat file until further parsing is finished.
    fn new() -> Self {
        State {
            buffer: Vec::with_capacity(67910),
            times: Vec::with_capacity(540),
        }
    }

    /// Load a state.dat file.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::state::*;
    /// let state = State::load("state.dat").unwrap();
    /// ```
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self, ElmaError> {
        let buffer = fs::read(filename)?;
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

    /// Returns state.dat as a stream of bytes.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::state::*;
    /// let mut state = State::load("state.dat").unwrap();
    /// let buffer = state.to_bytes().unwrap();
    /// ```
    pub fn to_bytes(&mut self) -> Result<Vec<u8>, ElmaError> {
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
        crypt_state(&mut buffer[4..]);

        Ok(buffer)
    }

    /// Save state.dat
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use elma::state::*;
    /// let mut state = State::load("state.dat").unwrap();
    /// state.save("newstate.dat").unwrap();
    /// ```
    pub fn save<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), ElmaError> {
        let buffer = self.to_bytes()?;
        fs::write(filename, &buffer)?;

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
