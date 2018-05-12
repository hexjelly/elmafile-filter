extern crate elma;

use elma::state::*;
use elma::{BestTimes, TimeEntry};
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
