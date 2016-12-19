extern crate elma;

use elma::{ Position };
use elma::lev::*;

#[test]
fn topology_ok() {
    let level = Level::load("tests/assets/levels/test_1.lev").unwrap();
    assert_eq!(level.check_topology().is_ok(), true);
}

#[test]
fn topology_err_too_wide() {
    let mut level = Level::new();
    level.polygons.push(Polygon { grass: false, vertices: vec![
        Position { x: 0_f64, y: 0_f64 },
        Position { x: 188.00001_f64, y: 0_f64 },
        Position { x: 0_f64, y: 1_f64 }]});
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::TooWide(0.000010000000003174137_f64));
}

#[test]
fn topology_err_too_high() {
    let mut level = Level::new();
    level.polygons.push(Polygon { grass: false, vertices: vec![
        Position { x: 0_f64, y: 0_f64 },
        Position { x: 0_f64, y: 188.00001_f64 },
        Position { x: 1_f64, y: 0_f64 }]});
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::TooHigh(0.000010000000003174137_f64));
}
