extern crate elma;
extern crate rand;
#[cfg(test)]
mod tests {
    use elma::{ lev, rec, Position, time_format, trim_string };
    use rand::random;

    // Helper functions.
    #[test]
    fn test_decrypt_encrypt () {
        let mut initial: Vec<u8> = vec![];
        for _ in 0..688 {
            initial.push(random::<u8>());
        }
        let decrypted = lev::crypt_top10(&initial);
        let encrypted = lev::crypt_top10(&decrypted);
        assert_eq!(initial, encrypted);
    }

    #[test]
    #[should_panic]
    fn test_trim_string () {
        let bytes: [u8;5] = [222,222,222,100,211];
        trim_string(&bytes).unwrap(); }

    #[test]
    fn test_time_format () {
        assert_eq!("11:48,01", time_format(114801).unwrap());
        assert_eq!("01:00,21", time_format(10021).unwrap());
        assert_eq!("01:00,99", time_format(10099).unwrap());
        assert_eq!("59:00,99", time_format(590099).unwrap());
        assert_eq!("00:10,00", time_format(1000).unwrap());
        assert_eq!("10:00,00", time_format(100000).unwrap());
        assert_eq!("00:00,00", time_format(0).unwrap());
        assert_eq!("59:59,99", time_format(1922039).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_invalid_time_format_1 () {
        time_format(16039_i32).unwrap(); }

    #[test]
    #[should_panic]
    fn test_invalid_time_format_2 () {
        time_format(601039_i32).unwrap(); }


    #[test]
    // Probably redundant, but maybe some new fields are added in the future.
    // Doesn't hurt or impact anything.
    fn level_default_values () {
        let mut default_lev = lev::Level::default();
        let mut new_lev = lev::Level::new();
        default_lev.link = 1000;
        new_lev.link = 1000;
        assert_eq!(default_lev, new_lev);
    }

    #[test]
    fn construct_level_and_save () {
        let mut level = lev::Level { raw: vec![],
                                     version: lev::Version::default(),
                                     link: random::<i32>(),
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

        level.polygons.push(lev::Polygon::new());
        level.objects.push(lev::Object::new());
        level.objects.push(lev::Object { position: Position { x: 0_f64, y: 0_f64 },
                                         object_type: lev::ObjectType::Apple { gravity: lev::Direction::Down,
                                                                               animation: 1 }});
        level.pictures.push(lev::Picture::new());
        level.pictures.push(lev::Picture::new());
        level.pictures.push(lev::Picture::new());
        level.pictures[1].clip = lev::Clip::Unclipped;
        level.pictures[2].clip = lev::Clip::Ground;
        level.top10_single.push(lev::ListEntry::new());
        level.top10_multi.push(lev::ListEntry::new());
        level.generate_link();
        let _ = level.get_raw(false).unwrap();
        level.save("tests/constructed.lev", true).unwrap();
    }

    #[test]
    #[should_panic]
    fn load_invalid_level_path () {
        let _ = rec::Replay::load("tests/missing.lev").unwrap(); }

    #[test]
    #[should_panic]
    fn load_across_level_1 () {
        let _ = lev::Level::load("tests/across.lev").unwrap(); }

    #[test]
    #[should_panic]
    fn save_across_level_1 () {
        let mut level = lev::Level::new();
        level.version = lev::Version::Across;
        level.save("tests/save_across_level_1.lev", false).unwrap(); }

    #[test]
    fn load_valid_level_1 () {
        let level = lev::Level::load("tests/test_1.lev").unwrap();
        assert_eq!(level.version, lev::Version::Elma);
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
        assert_eq!(level.polygons, vec![lev::Polygon {
                grass: false, vertices: vec![
                    Position { x: -23.993693053024586_f64, y: -3.135779367971911_f64 },
                    Position { x: -15.989070625361132_f64, y: -3.135779367971911_f64 },
                    Position { x: -15.989070625361132_f64, y: 1.995755366905195_f64 },
                    Position { x: -24_f64, y: 2_f64 }]
            },
            lev::Polygon {
                grass: true, vertices: vec![
                    Position { x: -23.83645939819548_f64, y: 2.310222676563402_f64 },
                    Position { x: -17.60428907951465_f64, y: 2.2816347393217473_f64 },
                    Position { x: -17.53281923641051_f64, y: 1.8956975865594021_f64 },
                    Position { x: -23.96510511578293_f64, y: 1.924285523801057_f64 }]
            }
        ]);

        // Object tests.
        assert_eq!(level.objects.len(), 8);
        assert_eq!(level.objects, vec![lev::Object {
                position: Position { x: -23.221818747499896_f64, y: -1.3204453531268072_f64 },
                object_type: lev::ObjectType::Killer
            },
            lev::Object {
                position: Position { x: -20.37252715482359_f64, y: -0.3124543521844827_f64 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Normal, animation: 9 }
            },
            lev::Object {
                position: Position { x: -20.3914786548306_f64, y: 0.5277288147929609_f64 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Up, animation: 1 }
            },
            lev::Object {
                position: Position { x: -19.526026821177144_f64, y: 0.36348248139887396_f64 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Right, animation: 5 }
            },
            lev::Object {
                position: Position { x: -21.269564821822065_f64, y: 0.38243398140588436_f64 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Left, animation: 1 }
            },
            lev::Object {
                position: Position { x: -19.55761265452216_f64, y: -0.4387976855645497_f64 },
                object_type: lev::ObjectType::Apple { gravity: lev::Direction::Up, animation: 1 }
            },
            lev::Object {
                position: Position { x: -20.075620321380434_f64, y: -1.2473950191969765_f64 },
                object_type: lev::ObjectType::Exit
            },
            lev::Object {
                position: Position { x: -22.94993115577695_f64, y: 1.5068896484884773_f64 },
                object_type: lev::ObjectType::Player
            }
        ]);

        // Picture tests.
        assert_eq!(level.pictures.len(), 2);
        assert_eq!(level.pictures, vec![lev::Picture {
            name: String::from("barrel"),
            texture: String::new(),
            mask: String::new(),
            position: Position { x: -19.37674118849727_f64, y: 0.895119783101471_f64 },
            distance: 380,
            clip: lev::Clip::Sky
        },
        lev::Picture {
            name: String::new(),
            texture: String::from("stone1"),
            mask: String::from("maskbig"),
            position: Position { x: -24.465394017511894_f64, y: -3.964829547979911_f64 },
            distance: 750,
            clip: lev::Clip::Sky
        }]);

        // Top10 tests.
        assert_eq!(level.top10_single.len(), 10);
        assert_eq!(level.top10_single[0], lev::ListEntry {
            name_1: String::from("Rust"),
            name_2: String::from("Cargo"),
            time: 201
        });
        assert_eq!(level.top10_single[2], lev::ListEntry {
            name_1: String::from("Cargo"),
            name_2: String::from("Rust"),
            time: 206
        });
        assert_eq!(level.top10_single[9], lev::ListEntry {
            name_1: String::from("Rust"),
            name_2: String::from("Cargo"),
            time: 308
        });
    }

    #[test]
    fn load_valid_level_2 () {
        let level = lev::Level::load("tests/test_2.lev").unwrap();
        assert_eq!(level.version, lev::Version::Elma);
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
        let mut level = lev::Level::load("tests/test_1.lev").unwrap();
        level.save("tests/save_level_1_wtop10.lev", true).unwrap();
        let level_saved = lev::Level::load("tests/save_level_1_wtop10.lev").unwrap();
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
        let mut level = lev::Level::load("tests/test_1.lev").unwrap();
        level.save("tests/save_level_1_notop10.lev", false).unwrap();
        let level_saved = lev::Level::load("tests/save_level_1_notop10.lev").unwrap();
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
    #[should_panic]
    fn load_invalid_level_1 () {
        let _ = lev::Level::load("tests/invalid_1.lev").unwrap(); }

    #[test]
    #[should_panic]
    fn load_invalid_gravity_level_1 () {
        let _ = lev::Level::load("tests/invalid_grav.lev").unwrap(); }

    #[test]
    #[should_panic]
    fn load_invalid_object_level_1 () {
        let _ = lev::Level::load("tests/invalid_obj.lev").unwrap(); }

    #[test]
    #[should_panic]
    fn load_invalid_clip_level_1 () {
        let _ = lev::Level::load("tests/invalid_clip.lev").unwrap(); }


    #[test]
    // Probably redundant, but maybe some new fields are added in the future.
    // Doesn't hurt or impact anything.
    fn rec_default_values () {
        let frame = rec::Frame::new();
        assert_eq!(frame, rec::Frame {
            bike: Position { x: 0_f32, y: 0_f32 },
            left_wheel: Position { x: 0, y: 0 },
            right_wheel: Position { x: 0, y: 0 },
            head: Position { x: 0, y: 0 },
            rotation: 0,
            left_wheel_rotation: 0,
            right_wheel_rotation: 0,
            throttle: false,
            right: false,
            volume: 0
        });
        let event = rec::Event::new();
        assert_eq!(event, rec::Event {
            time: 0_f64,
            event_type: rec::EventType::Touch { index: 0 }
        });
        let replay = rec::Replay::new();
        assert_eq!(replay, rec::Replay {
            raw: vec![],
            multi: false,
            flag_tag: false,
            link: 0,
            level: String::new(),
            frames: vec![],
            events: vec![],
            frames_2: vec![],
            events_2: vec![]
        });
    }

    #[test]
    #[should_panic]
    fn load_invalid_replay_path () {
        let _ = rec::Replay::load("tests/missing.rec").unwrap(); }

    #[test]
    fn load_valid_replay_1 () {
        let replay = rec::Replay::load("tests/test_1.rec").unwrap();
        assert_eq!(replay.multi, false);
        assert_eq!(replay.flag_tag, false);
        assert_eq!(replay.link, 2549082363);
        assert_eq!(replay.level, "tutor14.lev");

        // Some random frames.
        assert_eq!(replay.frames.len(), 440);
        assert_eq!(replay.frames[0], rec::Frame {
            bike: Position { x: 34.30250_f32, y: -1.1253119_f32 },
            left_wheel: Position { x: -850, y: -524 },
            right_wheel: Position { x: 849 , y: -524 },
            head: Position { x: 0, y: 439 },
            rotation: 10000,
            left_wheel_rotation: 250,
            right_wheel_rotation: 0,
            throttle: true,
            right: false,
            volume: 5120
        });
        assert_eq!(replay.frames[100], rec::Frame {
            bike: Position { x: 27.142517089844_f32, y: -1.1152113676071_f32 },
            left_wheel: Position { x: -903, y: -514 },
            right_wheel: Position { x: 586, y: -534 },
            head: Position { x: 74, y: 397 },
            rotation: 9826,
            left_wheel_rotation: 248,
            right_wheel_rotation: 238,
            throttle: true,
            right: false,
            volume: -5398
        });
        assert_eq!(replay.frames[201], rec::Frame {
            bike: Position { x: 11.07129573822_f32, y: 2.8753623962402_f32 },
            left_wheel: Position { x: -511, y: 917 },
            right_wheel: Position { x: -692, y: -789 },
            head: Position { x: 471, y: 10 },
            rotation: 7325,
            left_wheel_rotation: 25,
            right_wheel_rotation: 23,
            throttle: true,
            right: false,
            volume: -5398
        });
        assert_eq!(replay.frames[439], rec::Frame {
            bike: Position { x: -34.779712677002_f32, y: 11.526465415955_f32 },
            left_wheel: Position { x: -1050, y: -33 },
            right_wheel: Position { x: 286, y: -757 },
            head: Position { x: 226, y: 376 },
            rotation: 9047,
            left_wheel_rotation: 73,
            right_wheel_rotation: 163,
            throttle: true,
            right: false,
            volume: 5652
        });

        // Some random event.
        assert_eq!(replay.events.len(), 24);
        assert_eq!(replay.events[0], rec::Event {
            time: 1.57728480001688_f64,
            event_type: rec::EventType::VoltRight
         });
        assert_eq!(replay.events[1], rec::Event {
            time: 1.6974048000097273_f64,
            event_type: rec::EventType::Ground { alternative: false }
         });
        assert_eq!(replay.events[11], rec::Event {
            time: 3.9464880000114437_f64,
            event_type: rec::EventType::VoltLeft
         });
        assert_eq!(replay.events[23], rec::Event {
            time: 6.398683200001716_f64,
            event_type: rec::EventType::Touch { index: 3 }
         });
    }

    #[test]
    fn load_valid_multi_replay_1 () {
        let replay = rec::Replay::load("tests/test_2.rec").unwrap();
        assert_eq!(replay.multi, true);
        assert_eq!(replay.flag_tag, false);
        assert_eq!(replay.link, 2549082363);
        assert_eq!(replay.level, "tutor14.lev");
        assert_eq!(replay.frames.len(), 440);
        assert_eq!(replay.frames[439], rec::Frame {
            bike: Position { x: -34.779712677002_f32, y: 11.526465415955_f32 },
            left_wheel: Position { x: -1050, y: -33 },
            right_wheel: Position { x: 286, y: -757 },
            head: Position { x: 226, y: 376 },
            rotation: 9047,
            left_wheel_rotation: 73,
            right_wheel_rotation: 163,
            throttle: true,
            right: false,
            volume: 5652
        });
        assert_eq!(replay.events.len(), 24);
        assert_eq!(replay.frames_2.len(), 441);
        assert_eq!(replay.frames_2[100].bike.x, 27.138593673706_f32);
        assert_eq!(replay.frames_2[0].bike.y, -1.1253118515015_f32);
        assert_eq!(replay.events_2.len(), 23);
    }

    #[test]
    fn load_valid_replay_1_and_save () {
        let replay = rec::Replay::load("tests/test_1.rec").unwrap();
        replay.save("tests/save_replay_1.rec").unwrap();
        let replay_saved = rec::Replay::load("tests/save_replay_1.rec").unwrap();
        assert_eq!(replay.multi, replay_saved.multi);
        assert_eq!(replay.flag_tag, replay_saved.flag_tag);
        assert_eq!(replay.link, replay_saved.link);
        assert_eq!(replay.level, replay_saved.level);
        assert_eq!(replay.frames, replay_saved.frames);
        assert_eq!(replay.events, replay_saved.events);
        assert_eq!(replay.frames_2, replay_saved.frames_2);
        assert_eq!(replay.events_2, replay_saved.events_2);
    }

    #[test]
    fn load_valid_replay_2_and_save () {
        let replay = rec::Replay::load("tests/test_3.rec").unwrap();
        replay.save("tests/save_replay_2.rec").unwrap();
        let replay_saved = rec::Replay::load("tests/save_replay_2.rec").unwrap();
        assert_eq!(replay.multi, replay_saved.multi);
        assert_eq!(replay.flag_tag, replay_saved.flag_tag);
        assert_eq!(replay.link, replay_saved.link);
        assert_eq!(replay.level, replay_saved.level);
        assert_eq!(replay.frames, replay_saved.frames);
        assert_eq!(replay.events, replay_saved.events);
        assert_eq!(replay.frames_2, replay_saved.frames_2);
        assert_eq!(replay.events_2, replay_saved.events_2);
    }

    #[test]
    fn load_valid_multi_replay_1_and_save () {
        let replay = rec::Replay::load("tests/test_2.rec").unwrap();
        replay.save("tests/save_multi_replay_2.rec").unwrap();
        let replay_saved = rec::Replay::load("tests/save_multi_replay_2.rec").unwrap();
        assert_eq!(replay.multi, replay_saved.multi);
        assert_eq!(replay.flag_tag, replay_saved.flag_tag);
        assert_eq!(replay.link, replay_saved.link);
        assert_eq!(replay.level, replay_saved.level);
        assert_eq!(replay.frames, replay_saved.frames);
        assert_eq!(replay.events, replay_saved.events);
        assert_eq!(replay.frames_2, replay_saved.frames_2);
        assert_eq!(replay.events_2, replay_saved.events_2);
    }

    #[test]
    #[should_panic]
    fn load_invalid_event_replay () {
        let _ = rec::Replay::load("tests/invalid_event.rec").unwrap(); }

    #[test]
    fn replay_get_time () {
        let replay = rec::Replay::load("tests/test_1.rec").unwrap();
        let (time, finished) = replay.get_time_ms();
        assert_eq!(time, 14649);
        assert_eq!(finished, true);
    }
}
