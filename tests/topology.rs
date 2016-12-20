extern crate elma;

use elma::{ Position };
use elma::lev::*;

#[test]
fn topology_ok() {
    let mut level = Level::new();

    // Add multiple exits.
    level.objects.push(Object { position: Position { x: 1_f64, y: 0_f64 }, object_type: ObjectType::Exit });
    level.objects.push(Object { position: Position { x: 2_f64, y: 0_f64 }, object_type: ObjectType::Exit });

    // Add exact allowed dimensions.
    level.polygons = vec![
        Polygon {
            grass: false,
            vertices: vec![
                Position { x: 0_f64, y: 188_f64 },
                Position { x: 188_f64, y: 188_f64 },
                Position { x: 188_f64, y: 0_f64 },
                Position { x: 0_f64, y: 0_f64 }
            ]
        }
    ];

    assert_eq!(level.check_topology().is_ok(), true);
}

#[test]
fn topology_err_too_wide() {
    let mut level = Level::new();
    level.polygons.push(Polygon { grass: false, vertices: vec![
        Position { x: 0_f64, y: 0_f64 },
        Position { x: 188.0000000000001_f64, y: 0_f64 },
        Position { x: 0_f64, y: 1_f64 }]});
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::TooWide(0.00000000000011368683772161603_f64));
}

#[test]
fn topology_err_too_high() {
    let mut level = Level::new();
    level.polygons.push(Polygon { grass: false, vertices: vec![
        Position { x: 0_f64, y: 0_f64 },
        Position { x: 0_f64, y: 188.0000000000001_f64 },
        Position { x: 1_f64, y: 0_f64 }]});
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::TooHigh(0.00000000000011368683772161603_f64));
}

#[test]
fn topology_missing_exit() {
    let mut level = Level::new();
    level.objects = vec![Object { position: Position { x: 0_f64, y: 0_f64 }, object_type: ObjectType::Player }];
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::MissingExit);
}

#[test]
fn topology_err_too_many_players() {
    let mut level = Level::new();
    level.objects = vec![
        Object { position: Position { x: 0_f64, y: 0_f64 }, object_type: ObjectType::Player },
        Object { position: Position { x: 0_f64, y: 0_f64 }, object_type: ObjectType::Player }
    ];
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::InvalidPlayerCount(2));
}

#[test]
fn topology_err_missing_player() {
    let mut level = Level::new();
    level.objects = vec![Object { position: Position { x: 0_f64, y: 0_f64 }, object_type: ObjectType::Exit }];
    assert_eq!(level.check_topology().unwrap_err(), TopologyError::InvalidPlayerCount(0));
}

// TODO: AppleInsideGround(usize),
// TODO: IntersectingPolygons,
// TODO: MaxObjects(usize),
// TODO: MaxPictures(usize),
// TODO: MaxPolygons(usize),
