extern crate elma;

use elma::lgr::*;
use elma::{Clip, ElmaError};
use std::env;
use std::fs::File;
use std::io::Read;

#[test]
// Probably redundant, but maybe some new fields are added in the future. I don't know.
// Doesn't hurt or impact anything.
fn correctly_loads_lgr() {
    let lgr = LGR::load("tests/assets/lgr/Default.lgr");
    assert!(lgr.is_ok());
}
