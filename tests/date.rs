use fastdate::{Date, DateTime};
use std::str::FromStr;

#[test]
fn test_date_empty() {
    let d = Date::from_str("");
    assert_eq!(d.is_err(),true);
}

#[test]
fn test_from_str() {
    let d = Date::from_str("2022-12-13").unwrap();
    assert_eq!(d.to_string(),"2022-12-13");
}

#[test]
fn test_from_str_2() {
    let d = Date::from_str("2022/12/13").unwrap();
    assert_eq!(d.to_string(),"2022-12-13");
}


#[test]
fn test_date() {
    let d = Date::from_str("2022-12-13 11:12:13.123456").unwrap();
    println!("{}", d);
    assert_eq!("2022-12-13".to_string(), d.to_string());
}

#[test]
fn test_ser() {
    let d = DateTime::from_str("2022-12-13T11:12:13Z").unwrap();
    println!("{}", d);
    let v = serde_json::to_string(&d).unwrap();
    assert_eq!(v, "\"2022-12-13T11:12:13Z\"");
}
