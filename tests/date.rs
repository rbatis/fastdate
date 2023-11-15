use fastdate::{Date, DateTime};
use std::str::FromStr;

#[test]
fn test_date_empty() {
    let d = Date::from_str("");
    assert_eq!(d.is_err(), true);
}

#[test]
fn test_from_str() {
    let d = Date::from_str("2022-12-13").unwrap();
    assert_eq!(d.to_string(), "2022-12-13");
}

#[test]
fn test_from_str_46911_mon() {
    let d = Date::from_str("2022-04-13").unwrap();
    assert_eq!(d.to_string(), "2022-04-13");
    let d = Date::from_str("2022-06-13").unwrap();
    assert_eq!(d.to_string(), "2022-06-13");
    let d = Date::from_str("2022-09-13").unwrap();
    assert_eq!(d.to_string(), "2022-09-13");
    let d = Date::from_str("2022-11-13").unwrap();
    assert_eq!(d.to_string(), "2022-11-13");

    let d = Date::from_str("2022-02-13").unwrap();
    assert_eq!(d.to_string(), "2022-02-13");
}

#[test]
fn test_from_str_run() {
    let d = Date::from_str("2024-02-13").unwrap();
    assert_eq!(d.to_string(), "2024-02-13");
}

#[test]
fn test_from_str_mon_out() {
    let d = Date::from_str("2024-14-13");
    assert_eq!(d.is_err(), true);
}

#[test]
fn test_from_str_day_zero() {
    let d = Date::from_str("2024-14-0");
    assert_eq!(d.is_err(), true);
}

#[test]
fn test_from_str_day_out() {
    let d = Date::from_str("2024-02-40");
    assert_eq!(d.is_err(), true);
}

#[test]
fn test_set_day() {
    let d = Date::from_str("2024-02-01").unwrap().set_day(1);
    assert_eq!(d.to_string(), "2024-02-01");
    assert_eq!(d.get_day(), 1);

    let d = Date::from_str("2024-02-01").unwrap().set_day(0);
    assert_eq!(d.to_string(), "2024-02-01");

    let d = Date::from_str("2024-02-01").unwrap().set_day(50);
    assert_eq!(d.to_string(), "2024-02-01");
}

#[test]
fn test_set_mon() {
    let d = Date::from_str("2024-02-01").unwrap().set_mon(2);
    assert_eq!(d.to_string(), "2024-02-01");
    assert_eq!(d.get_mon(), 2);

    let d = Date::from_str("2024-02-01").unwrap().set_mon(0);
    assert_eq!(d.to_string(), "2024-02-01");

    let d = Date::from_str("2024-02-01").unwrap().set_mon(50);
    assert_eq!(d.to_string(), "2024-02-01");
}

#[test]
fn test_set_year() {
    let d = Date::from_str("2024-02-01").unwrap().set_year(2024);
    assert_eq!(d.to_string(), "2024-02-01");
    assert_eq!(d.get_year(), 2024);

    let d = Date::from_str("2024-02-01").unwrap().set_year(-1);
    assert_eq!(d.to_string(), "2024-02-01");

    let d = Date::from_str("2024-02-01").unwrap().set_year(10000);
    assert_eq!(d.to_string(), "2024-02-01");
}

#[test]
fn test_ser_de() {
    let d = Date::from_str("2024-02-01").unwrap();
    let data = serde_json::to_value(&d).unwrap();
    let new_d = serde_json::from_value(data).unwrap();
    assert_eq!(d, new_d);
}

#[test]
fn test_from_str_2() {
    let d = Date::from_str("2022/12/13").unwrap();
    assert_eq!(d.to_string(), "2022-12-13");
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
