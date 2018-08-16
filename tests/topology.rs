extern crate elma;

use elma::lev::*;
use elma::Position;

#[test]
fn topology_ok() {
    let mut level = Level::new();

    // Add multiple exits.
    level.objects.push(Object {
        position: Position::new(1_f64, 0_f64),
        object_type: ObjectType::Exit,
    });
    level.objects.push(Object {
        position: Position::new(2_f64, 0_f64),
        object_type: ObjectType::Exit,
    });

    // Add exact allowed dimensions.
    level.polygons = vec![Polygon {
        grass: false,
        vertices: vec![
            Position::new(0_f64, 188_f64),
            Position::new(188_f64, 188_f64),
            Position::new(188_f64, 0_f64),
            Position::new(0_f64, 0_f64),
        ],
    }];

    assert_eq!(level.check_topology().is_ok(), true);
}

#[test]
fn topology_err_too_wide() {
    let mut level = Level::new();
    level.polygons.push(Polygon {
        grass: false,
        vertices: vec![
            Position::new(0_f64, 0_f64),
            Position::new(188.0000000000001_f64, 0_f64),
            Position::new(0_f64, 1_f64),
        ],
    });
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::TooWide(0.00000000000011368683772161603_f64)
    );
}

#[test]
fn topology_err_too_high() {
    let mut level = Level::new();
    level.polygons.push(Polygon {
        grass: false,
        vertices: vec![
            Position::new(0_f64, 0_f64),
            Position::new(0_f64, 188.0000000000001_f64),
            Position::new(1_f64, 0_f64),
        ],
    });
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::TooHigh(0.00000000000011368683772161603_f64)
    );
}

#[test]
fn topology_missing_exit() {
    let mut level = Level::new();
    level.objects = vec![Object {
        position: Position::new(0_f64, 0_f64),
        object_type: ObjectType::Player,
    }];
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::MissingExit
    );
}

#[test]
fn topology_err_too_many_players() {
    let mut level = Level::new();
    level.objects = vec![
        Object {
            position: Position::new(0_f64, 0_f64),
            object_type: ObjectType::Player,
        },
        Object {
            position: Position::new(0_f64, 0_f64),
            object_type: ObjectType::Player,
        },
    ];
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::InvalidPlayerCount(2)
    );
}

#[test]
fn topology_err_missing_player() {
    let mut level = Level::new();
    level.objects = vec![Object {
        position: Position::new(0_f64, 0_f64),
        object_type: ObjectType::Exit,
    }];
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::InvalidPlayerCount(0)
    );
}

#[test]
fn topology_err_polygon_count() {
    let mut level = Level::new();
    // Add too many polygons
    for _ in 0..1005 {
        level.polygons.push(Polygon::new());
    }
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::MaxPolygons(6)
    );
}

#[test]
fn topology_err_object_count() {
    let mut level = Level::new();
    // Add too many objects
    for _ in 0..255 {
        level.objects.push(Object::new());
    }
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::MaxObjects(5)
    );
}

#[test]
fn topology_err_picture_count() {
    let mut level = Level::new();
    // Add too many pictures
    for _ in 0..5005 {
        level.pictures.push(Picture::new());
    }
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::MaxPictures(5)
    );
}

#[test]
fn topology_err_invalid_vertex_count() {
    let mut level = Level::new();
    // Add three polygons, two with less than three vertices, with one valid in between.
    level.polygons.push(Polygon {
        grass: false,
        vertices: vec![
            Position::new(1_f64, 12_f64),
            Position::new(138_f64, 118_f64),
        ],
    });
    level.polygons.push(Polygon {
        grass: false,
        vertices: vec![
            Position::new(1_f64, 12_f64),
            Position::new(138_f64, 118_f64),
            Position::new(18_f64, 28_f64),
        ],
    });
    level.polygons.push(Polygon {
        grass: false,
        vertices: vec![Position::new(12_f64, 32_f64), Position::new(7_f64, 83_f64)],
    });
    assert_eq!(
        level.check_topology().unwrap_err(),
        TopologyError::InvalidVertexCount(vec![1, 3])
    );
}
