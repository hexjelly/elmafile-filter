extern crate elma;

use elma::Time;

#[test]
fn correct_time_format() {
    assert_eq!("19:08,01", Time(114801).to_string());
    assert_eq!("01:40,21", Time(10021).to_string());
    assert_eq!("01:40,99", Time(10099).to_string());
    assert_eq!("01:38:20,99", Time(590099).to_string());
    assert_eq!("10,00", Time(1000).to_string());
    assert_eq!("10:00,00", Time(60000).to_string());
    assert_eq!("00,00", Time(0).to_string());
    assert_eq!("05:20:20,39", Time(1922039).to_string());
}

#[test]
fn correct_time_to_parts() {
    assert_eq!((0, 19, 8, 1), Time(114801).to_parts());
    assert_eq!((0, 1, 40, 21), Time(10021).to_parts());
    assert_eq!((0, 1, 40, 99), Time(10099).to_parts());
    assert_eq!((1, 38, 20, 99), Time(590099).to_parts());
    assert_eq!((0, 0, 10, 0), Time(1000).to_parts());
    assert_eq!((0, 10, 0, 0), Time(60000).to_parts());
    assert_eq!((0, 0, 0, 0), Time(0).to_parts());
    assert_eq!((5, 20, 20, 39), Time(1922039).to_parts());
}

#[test]
fn string_to_time() {
    assert_eq!(Time::from("320:20,39"), Time(1922039));
    assert_eq!(Time::from("-320:-20,39"), Time(-1922039));
    assert_eq!(Time::from("98:20,99"), Time(590099));
    assert_eq!(Time::from("01:38:20,99"), Time(590099));
    assert_eq!(Time::from("19:08,01"), Time(114801));
    assert_eq!(Time::from("0:08,01"), Time(801));
    assert_eq!(Time::from("00:08,01"), Time(801));
    assert_eq!(Time::from("08,01"), Time(801));
    assert_eq!(Time::from("8,01"), Time(801));
    assert_eq!(Time::from("-8,01"), Time(-801));
    assert_eq!(Time::from("01"), Time(1));
    assert_eq!(Time::from("1"), Time(1));
    assert_eq!(Time::from("0"), Time(0));

    assert_eq!(Time::from("00,00:00"), Time(0));
    assert_eq!(Time::from("01:00;00"), Time(6000));
    assert_eq!(Time::from("01:00,00"), Time(6000));
}

#[test]
fn time_ops_work_correctly() {
    assert_eq!(Time(114832), Time(114801) + Time(31));
    assert_eq!(Time(331), Time::from("00,00:00") + Time(331));
    assert_eq!(
        Time::from("01,20:01"),
        Time::from("01,20:00") + Time::from("00:01")
    );
    assert_eq!(
        Time::from("90:00,00"),
        Time::from("98:20,99") - Time::from("08:20,99")
    );
    assert_eq!(
        Time::from("1:30:00,00"),
        Time::from("1:38:20,99") - Time::from("08:20,99")
    );
    assert_eq!(Time(0), Time::from("18:23,19") - Time::from("18:23,19"));
    assert_eq!(Time(-100), Time::from("00:23,19") - Time::from("00:24,19"));
    assert_eq!(Time(640139), Time(643451) - Time(3312));
}
