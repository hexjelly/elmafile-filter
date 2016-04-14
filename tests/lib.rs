extern crate elma;
extern crate rand;
#[cfg(test)]
mod tests {
    use elma::{ lev, rec };
    use rand::random;

    #[test]
    fn test_decrypt_encrypt () {
        let mut initial: Vec<u8> = vec![];
        for _ in 0..688 {
            initial.push(random::<u8>());
        }
        let decrypted = lev::crypt_top10(initial.clone());
        let encrypted = lev::crypt_top10(decrypted);
        assert_eq!(initial, encrypted);
    }

    #[test]
    fn test_lev_default_values () {
        let level = lev::Level::new();
        assert_eq!(level.version, lev::Version::Elma);
        assert_eq!(level.link, 0);
        assert_eq!(level.integrity, [0.0_f64; 4]);
        assert_eq!(level.name, String::new();
        assert_eq!(level.lgr, String::from("default");
        assert_eq!(level.ground, String::from("ground");
        assert_eq!(level.sky, String::from("sky");
    }

    #[test]
    fn test_lev_load_level () {
        let level = lev::Level::load_level("tests/test.lev");
        assert_eq!(level.version, lev::Version::Elma);
        assert_eq!(level.link, 1524269776);
        assert_eq!(level.integrity, [-1148375.210607791,
                                      1164056.210607791,
                                      1162467.210607791,
                                      1162283.210607791]);
        assert_eq!(level.name, String::from("Rust test");
        assert_eq!(level.lgr, String::from("default");
        assert_eq!(level.ground, String::from("ground");
        assert_eq!(level.sky, String::from("sky");
        assert_eq!(level.polygons.len(), 2);
        // TODO: add polygon vertex and grass tests etc.
        //assert_eq!(level.polygons, vec![lev::Polygon { grass: false, vertices: vec![] }, lev::Polygon { grass: true, vertices: vec![] }]);
        // TODO: proper object tests
        assert_eq!(level.objects.len(), 8);
        // TODO: proper pictures tests
        assert_eq!(level.pictures.len(), 2);
        // TODO: test top10 list
    }

    #[test]
    fn test_rec_default_values () {
        let rec = rec::Rec::new();
        assert_eq!(rec.frame_count, 0);
        assert_eq!(rec.multi, false);
        assert_eq!(rec.flag_tag, false);
        assert_eq!(rec.link, 0);
        assert_eq!(rec.level, String::new();
        assert_eq!(rec.frames, vec![]);
        assert_eq!(rec.events, vec![]);
    }
}
