use fastdate::{Date, DateTime};
use std::str::FromStr;

#[test]
fn test_date() {
    let d = Date::from_str("2022-12-13 11:12:13.123456").unwrap();
    println!("{}", d);
    assert_eq!("2022-12-13".to_string(), d.to_string());
}

#[test]
fn test_ser() {
    let d = DateTime::from_str("2022-12-13 11:12:13").unwrap();
    println!("{}", d);
    let v = serde_json::to_string(&d).unwrap();
    assert_eq!("\"2022-12-13 11:12:13\"", v);
}
