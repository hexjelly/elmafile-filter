extern crate elma;

use elma::lgr::*;
use std::fs;

#[test]
// Probably redundant, but maybe some new fields are added in the future. I don't know.
// Doesn't hurt or impact anything.
fn correctly_loads_lgr_1() {
    let lgr = LGR::load("tests/assets/lgr/Default.lgr");
    assert!(lgr.is_ok());
}

#[test]
fn correctly_loads_lgr_2() {
    let lgr = LGR::load("tests/assets/lgr/Across.lgr");
    assert!(lgr.is_ok());
}
