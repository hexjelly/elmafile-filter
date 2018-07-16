extern crate elma;
extern crate nom;

use elma::state::*;
use elma::{BestTimes, TimeEntry};
use nom::simple_errors::Context::Code;
use nom::Err::Error;
use nom::Err::Incomplete;
use nom::ErrorKind::CondReduce;
use nom::Needed::Size;
use std::env;
use std::fs;

#[test]
/// Load state.dat, save it and compare without changes.
fn load_parse_save_state() {
    let orig_state = State::load("tests/assets/state/state.dat").unwrap();
    let mut dir = env::temp_dir();
    dir.push("saved.dat");
    let mut state = orig_state.clone();
    state.save(&dir).unwrap();

    let file_original = fs::read("tests/assets/state/state.dat").unwrap();
    let file_saved = fs::read(&dir).unwrap();

    let saved_state = State::load(&dir).unwrap();

    let mut expected_times = BestTimes::new();
    expected_times
        .single
        .push(TimeEntry::new(("proman", "proman"), 1465));
    expected_times
        .single
        .push(TimeEntry::new(("proman", "proman"), 1487));
    expected_times
        .multi
        .push(TimeEntry::new(("proman", "proman"), 1492));
    expected_times
        .multi
        .push(TimeEntry::new(("proman", "proman"), 1494));
    assert_eq!(expected_times, orig_state.times[0]);
    assert_eq!(PlayMode::Single, orig_state.play_mode);
    assert_eq!(false, orig_state.sound_enabled);
    assert_eq!(
        SoundOptimization::Compatibility,
        orig_state.sound_optimization
    );
    assert_eq!(true, orig_state.animated_menus);
    assert_eq!(VideoDetail::High, orig_state.video_detail);
    assert_eq!(true, orig_state.animated_objects);
    assert_eq!(false, orig_state.swap_bikes);
    &file_original
        .iter()
        .zip(&file_saved)
        .enumerate()
        .for_each(|(i, (o, s))| {
            if o != s {
                assert!(false, format!("State files differ at: {}", i))
            }
        });
    assert_eq!(&file_original, &file_saved);
    assert_eq!(&file_original.len(), &file_saved.len());
    assert_eq!(orig_state, saved_state);
}

#[test]
fn load_state_from_bytes() {
    let state = State::load("tests/assets/state/state.dat").unwrap();
    let buffer = fs::read("tests/assets/state/state.dat").unwrap();
    assert_eq!(state, State::from_bytes(&buffer).unwrap());
}

#[test]
fn default_state() {
    let state = State::load("tests/assets/state/state_default.dat").unwrap();
    assert_eq!(State::new(), state);
}

#[test]
fn null_pad_string() {
    assert_eq!(
        null_padded_string(b"Elma\0\0\0\0\0\0", 10),
        Ok((&[][..], "Elma"))
    );
    assert_eq!(
        null_padded_string(b"Elma\0\0\0\0\0\0\0\0", 10),
        Ok((&[0, 0][..], "Elma"))
    );
    assert_eq!(
        null_padded_string(b"\0\0\0\0\0\0\0\0\0\0", 10),
        Ok((&[][..], ""))
    );
    assert_eq!(
        null_padded_string(b"Elma\0\0\0\0\0", 10),
        Err(Incomplete(Size(6)))
    );
    assert_eq!(
        null_padded_string(b"\0\0\0\0\0\0\0\0\0", 10),
        Err(Incomplete(Size(10)))
    );
    assert_eq!(
        null_padded_string(b"ElastoMani", 10),
        Err(Incomplete(Size(1)))
    );
    assert_eq!(
        null_padded_string(b"ElastoMania", 10),
        Err(Incomplete(Size(1)))
    );
    assert_eq!(
        null_padded_string(b"ElastoMania\0", 10),
        Err(Error(Code(&[0][..], CondReduce)))
    );
}
