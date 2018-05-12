extern crate elma;

use elma::lgr::*;
use std::env;

#[test]
fn correctly_loads_lgr_1() {
    let lgr = LGR::load("tests/assets/lgr/Default.lgr");
    assert!(lgr.is_ok());
}

#[test]
fn correctly_loads_lgr_2() {
    let lgr = LGR::load("tests/assets/lgr/Across.lgr");
    assert!(lgr.is_ok());
}

#[test]
fn correctly_loads_saves_and_reloads_lgrs() {
    let mut dir = env::temp_dir();
    dir.push("resaved.lgr");

    let orig_default_lgr = LGR::load("tests/assets/lgr/Default.lgr").unwrap();
    orig_default_lgr.save(&dir).unwrap();
    let reloaded_default_lgr = LGR::load(&dir).unwrap();
    assert_eq!(orig_default_lgr, reloaded_default_lgr);

    let orig_across_lgr = LGR::load("tests/assets/lgr/Across.lgr").unwrap();
    orig_across_lgr.save(&dir).unwrap();
    let reloaded_across_lgr = LGR::load(&dir).unwrap();
    assert_eq!(orig_across_lgr, reloaded_across_lgr);
}
