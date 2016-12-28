extern crate elma;
extern crate rand;

use elma::{ Position };
use elma::lev::*;
use std::env;
use rand::random;

#[test]
/// Generate random u8 data to simulate top10 lists, encrypting it and decrypting it,
/// and testing whether it returns the same unencrypted data.
fn decrypt_encrypt_top10 () {
    let mut initial: Vec<u8> = vec![];
    for _ in 0..688 {
        initial.push(random::<u8>());
    }
    let decrypted = crypt_top10(&initial);
    let encrypted = crypt_top10(&decrypted);
    assert_eq!(initial, encrypted);
}

#[test]
// Probably redundant, but maybe some new fields are added in the future. I don't know.
// Doesn't hurt or impact anything.
fn level_default_values () {
    let mut default_lev = Level::default();
    let mut new_lev = Level::new();
    default_lev.link = 1000;
    new_lev.link = 1000;
    assert_eq!(default_lev, new_lev);
}

#[test]
/// Generate a level with some arbitrary values and see if it saves.
fn construct_level_and_save () {
    let mut level = Level { raw: vec![],
                                 version: Version::default(),
                                 link: random::<u32>(),
                                 integrity: [0f64; 4],
                                 name: String::new(),
                                 lgr: String::from("default"),
                                 ground: String::from("ground"),
                                 sky: String::from("sky"),
                                 polygons: vec![],
                                 objects: vec![],
                                 pictures: vec![],
                                 top10_single: vec![],
                                 top10_multi: vec![] };

    level.polygons.push(Polygon::new());
    level.objects.push(Object::new());
    level.objects.push(Object { position: Position { x: 0_f64, y: 0_f64 },
                                     object_type: ObjectType::Apple { gravity: Direction::Down,
                                                                           animation: 1 }});
    level.pictures.push(Picture::new());
    level.pictures.push(Picture::new());
    level.pictures.push(Picture::new());
    level.pictures[1].clip = Clip::Unclipped;
    level.pictures[2].clip = Clip::Ground;
    level.top10_single.push(ListEntry::new());
    level.top10_multi.push(ListEntry::new());
    level.generate_link();
    let _ = level.get_raw(false).unwrap();
    let mut dir = env::temp_dir();
    dir.push("constructed.lev");
    level.save(&dir, true).unwrap();
}

#[test]
/// Test top 10 saving when lists are longer than 10 entries, and whether they get sorted.
fn overflow_top10_and_sort () {
    let mut level = Level::new();
    // Create more than 10 entries unordered.
    let top10_single = vec![ListEntry { time: 2221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 231, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 1221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 10221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 2321, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 22211, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 2201, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 5, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 5121, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 918, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 17, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                             ListEntry { time: 8172, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() }];
    // Make multi list shorter, but still unordered.
    let top10_multi = vec![ListEntry { time: 2221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                            ListEntry { time: 231, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                            ListEntry { time: 2321, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                            ListEntry { time: 22211, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                            ListEntry { time: 918, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                            ListEntry { time: 17, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                            ListEntry { time: 8172, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() }];
    level.top10_single = top10_single;
    level.top10_multi = top10_multi;
    // Save and then load it again to see whether it worked.
    let mut dir = env::temp_dir();
    dir.push("top10_overflow_and_sort.lev");
    level.save(&dir, true).unwrap();
    let level = Level::load(&dir).unwrap();
    // Check if we get the expected sorted times.
    let expected_single = vec![ListEntry { time: 5, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 17, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 231, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 918, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 1221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 2201, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 2221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 2321, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 5121, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                        ListEntry { time: 8172, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() }];
    let expected_multi = vec![ListEntry { time: 17, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                                ListEntry { time: 231, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                                ListEntry { time: 918, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                                ListEntry { time: 2221, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                                ListEntry { time: 2321, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                                ListEntry { time: 8172, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() },
                                ListEntry { time: 22211, name_1: "test_p1".to_string(), name_2: "test_p2".to_string() }];
    assert_eq!(level.top10_single, expected_single);
    assert_eq!(level.top10_multi, expected_multi);
}

#[test]
fn load_invalid_level_path () {
    assert_eq!(Level::load("tests/assets/levels/missing.lev").unwrap_err(), elma::ElmaError::Io(std::io::ErrorKind::NotFound));
}

#[test]
/// This should error until Across support is added, if ever.
fn load_across_level_1 () {
    assert_eq!(Level::load("tests/assets/levels/across.lev").unwrap_err(), elma::ElmaError::AcrossUnsupported);
}

#[test]
/// Until Across is supported, should generate error when you try to save a Across level.
fn save_across_level_1 () {
    let mut level = Level::new();
    level.version = Version::Across;
    let mut dir = env::temp_dir();
    dir.push("save_across_level_1.lev");
    assert_eq!(level.save(&dir, false).unwrap_err(), elma::ElmaError::AcrossUnsupported);
}

#[test]
fn load_valid_level_1 () {
    let level = Level::load("tests/assets/levels/test_1.lev").unwrap();
    assert_eq!(level.version, Version::Elma);
    assert_eq!(level.link, 1524269776);
    assert_eq!(level.integrity, [-1148375.210607791_f64,
                                  1164056.210607791_f64,
                                  1162467.210607791_f64,
                                  1162283.210607791_f64]);
    assert_eq!(level.name, "Rust test");
    assert_eq!(level.lgr, "default");
    assert_eq!(level.ground, "ground");
    assert_eq!(level.sky, "sky");

    // Polygon tests.
    assert_eq!(level.polygons.len(), 2);
    assert_eq!(level.polygons, vec![Polygon {
            grass: false, vertices: vec![
                Position { x: -23.993693053024586_f64, y: -3.135779367971911_f64 },
                Position { x: -15.989070625361132_f64, y: -3.135779367971911_f64 },
                Position { x: -15.989070625361132_f64, y: 1.995755366905195_f64 },
                Position { x: -24_f64, y: 2_f64 }]
        },
        Polygon {
            grass: true, vertices: vec![
                Position { x: -23.83645939819548_f64, y: 2.310222676563402_f64 },
                Position { x: -17.60428907951465_f64, y: 2.2816347393217473_f64 },
                Position { x: -17.53281923641051_f64, y: 1.8956975865594021_f64 },
                Position { x: -23.96510511578293_f64, y: 1.924285523801057_f64 }]
        }
    ]);

    // Object tests.
    assert_eq!(level.objects.len(), 8);
    assert_eq!(level.objects, vec![Object {
            position: Position { x: -23.221818747499896_f64, y: -1.3204453531268072_f64 },
            object_type: ObjectType::Killer
        },
        Object {
            position: Position { x: -20.37252715482359_f64, y: -0.3124543521844827_f64 },
            object_type: ObjectType::Apple { gravity: Direction::Normal, animation: 9 }
        },
        Object {
            position: Position { x: -20.3914786548306_f64, y: 0.5277288147929609_f64 },
            object_type: ObjectType::Apple { gravity: Direction::Up, animation: 1 }
        },
        Object {
            position: Position { x: -19.526026821177144_f64, y: 0.36348248139887396_f64 },
            object_type: ObjectType::Apple { gravity: Direction::Right, animation: 5 }
        },
        Object {
            position: Position { x: -21.269564821822065_f64, y: 0.38243398140588436_f64 },
            object_type: ObjectType::Apple { gravity: Direction::Left, animation: 1 }
        },
        Object {
            position: Position { x: -19.55761265452216_f64, y: -0.4387976855645497_f64 },
            object_type: ObjectType::Apple { gravity: Direction::Up, animation: 1 }
        },
        Object {
            position: Position { x: -20.075620321380434_f64, y: -1.2473950191969765_f64 },
            object_type: ObjectType::Exit
        },
        Object {
            position: Position { x: -22.94993115577695_f64, y: 1.5068896484884773_f64 },
            object_type: ObjectType::Player
        }
    ]);

    // Picture tests.
    assert_eq!(level.pictures.len(), 2);
    assert_eq!(level.pictures, vec![Picture {
        name: String::from("barrel"),
        texture: String::new(),
        mask: String::new(),
        position: Position { x: -19.37674118849727_f64, y: 0.895119783101471_f64 },
        distance: 380,
        clip: Clip::Sky
    },
    Picture {
        name: String::new(),
        texture: String::from("stone1"),
        mask: String::from("maskbig"),
        position: Position { x: -24.465394017511894_f64, y: -3.964829547979911_f64 },
        distance: 750,
        clip: Clip::Sky
    }]);

    // Top10 tests.
    assert_eq!(level.top10_single.len(), 10);
    assert_eq!(level.top10_single[0], ListEntry {
        name_1: String::from("Rust"),
        name_2: String::from("Cargo"),
        time: 201
    });
    assert_eq!(level.top10_single[2], ListEntry {
        name_1: String::from("Cargo"),
        name_2: String::from("Rust"),
        time: 206
    });
    assert_eq!(level.top10_single[9], ListEntry {
        name_1: String::from("Rust"),
        name_2: String::from("Cargo"),
        time: 308
    });
}

#[test]
fn load_valid_level_2 () {
    let level = Level::load("tests/assets/levels/test_2.lev").unwrap();
    assert_eq!(level.version, Version::Elma);
    assert_eq!(level.link, 1505288190);
    assert_eq!(level.name, "");
    assert_eq!(level.ground, "brick");
    assert_eq!(level.sky, "ground");
    assert_eq!(level.polygons.len(), 5);
    assert_eq!(level.polygons[0].grass, false);
    assert_eq!(level.polygons[0].vertices.len(), 4);
    assert_eq!(level.polygons[0].vertices[0].x, 18.507991950076164);
    assert_eq!(level.polygons[0].vertices[1].y, 17.978810742022475);
    assert_eq!(level.objects.len(), 17);
    assert_eq!(level.pictures.len(), 3);
    assert_eq!(level.top10_single.len(), 0);
}

#[test]
fn load_valid_level_1_and_save_with_top10 () {
    let mut level = Level::load("tests/assets/levels/test_1.lev").unwrap();
    let mut dir = env::temp_dir();
    dir.push("save_level_1_wtop10.lev");
    level.save(&dir, true).unwrap();
    let level_saved = Level::load(&dir).unwrap();
    assert_eq!(level.name, level_saved.name);
    assert_eq!(level.ground, level_saved.ground);
    assert_eq!(level.sky, level_saved.sky);
    assert_eq!(level.polygons, level_saved.polygons);
    assert_eq!(level.objects, level_saved.objects);
    assert_eq!(level.pictures, level_saved.pictures);
    assert_eq!(level.top10_single, level_saved.top10_single);
    assert_eq!(level.top10_multi, level_saved.top10_multi);
}

#[test]
fn load_valid_level_1_and_save_without_top10 () {
    let mut level = Level::load("tests/assets/levels/test_1.lev").unwrap();
    let mut dir = env::temp_dir();
    dir.push("save_level_1_notop10.lev");
    level.save(&dir, false).unwrap();
    let level_saved = Level::load(&dir).unwrap();
    assert_eq!(level.name, level_saved.name);
    assert_eq!(level.ground, level_saved.ground);
    assert_eq!(level.sky, level_saved.sky);
    assert_eq!(level.polygons, level_saved.polygons);
    assert_eq!(level.objects, level_saved.objects);
    assert_eq!(level.pictures, level_saved.pictures);
    assert!(level.top10_single != level_saved.top10_single);
    assert!(level.top10_multi != level_saved.top10_multi);
    assert_eq!(level_saved.top10_single.len(), 0);
    assert_eq!(level_saved.top10_multi.len(), 0);
}

#[test]
fn load_invalid_level_1 () {
    assert_eq!(Level::load("tests/assets/levels/invalid_1.lev").unwrap_err(), elma::ElmaError::InvalidLevelFile);
}

#[test]
fn load_invalid_gravity_level_1 () {
    assert_eq!(Level::load("tests/assets/levels/invalid_grav.lev").unwrap_err(), elma::ElmaError::InvalidGravity(6));
}

#[test]
fn load_invalid_object_level_1 () {
    assert_eq!(Level::load("tests/assets/levels/invalid_obj.lev").unwrap_err(), elma::ElmaError::InvalidObject(6));
}

#[test]
fn load_invalid_clip_level_1 () {
    assert_eq!(Level::load("tests/assets/levels/invalid_clip.lev").unwrap_err(), elma::ElmaError::InvalidClipping(3));
}
