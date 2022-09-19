use fastdate::{Date, DateTime, DurationFrom, Time};
use std::str::FromStr;
use std::time::Duration;

#[test]
fn test_other_space() {
    let d = DateTime::from_str("1234_12_13_11_12_13.123456").unwrap();
    println!("{}", d);
    assert_eq!("1234-12-13 11:12:13.123456".to_string(), d.to_string());
}

#[test]
fn test_date() {
    let d = DateTime::from_str("1234-12-13 11:12:13.123456").unwrap();
    println!("{}", d);
    assert_eq!("1234-12-13 11:12:13.123456".to_string(), d.to_string());
}

#[test]
fn test_date_utc() {
    let d = DateTime::now();
    println!("{}", d);
}

#[test]
fn test_date_utc_add() {
    let d = DateTime::now();
    let added = d.clone() + Duration::from_secs(1);
    println!("{},{}", d, added);
    assert_eq!(d.add(Duration::from_secs(1)), added);
}

#[test]
fn test_offset() {
    let utc = DateTime::from_str("2022-12-12 12:12:12.000000").unwrap();
    assert_eq!(format!("{}", utc.set_offset(1)), "2022-12-12 12:12:13");
}

#[test]
fn test_timestamp() {
    let mut now = DateTime::utc();
    now.nano = 0;
    let timestamp = now.unix_timestamp();
    let new_time = DateTime::from_timestamp(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_timestamp_micros() {
    let mut now = DateTime::utc();
    now.nano = 0;
    let timestamp = now.unix_timestamp_micros();
    let new_time = DateTime::from_timestamp_micros(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_timestamp_millis() {
    let mut now = DateTime::utc();
    now.nano = 0;
    let timestamp = now.unix_timestamp_millis();
    let new_time = DateTime::from_timestamp_millis(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_timestamp_nano() {
    let now = DateTime::utc();
    let timestamp = now.unix_timestamp_nano();
    let new_time = DateTime::from_timestamp_nano(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_unix_timestamp() {
    let d = DateTime::now().unix_timestamp();
    println!("unix:{}", d);
    let d = DateTime::utc().unix_timestamp();
    println!("unix:{}", d);

    let d = DateTime::now().unix_timestamp_millis();
    println!("unix ms:{}", d);
    let d = DateTime::utc().unix_timestamp_millis();
    println!("unix ms:{}", d);

    let d = DateTime::now().unix_timestamp_nano();
    println!("unix nano:{}", d);
    let d = DateTime::utc().unix_timestamp_nano();
    println!("unix nano:{}", d);
}

#[test]
fn test_offset_zone() {
    let utc = DateTime::from_str("2022-12-12 00:00:00-08:00").unwrap();
    println!("{}", utc);
}

#[test]
fn test_into() {
    let utc = DateTime::from_str("2022-12-12 00:00:00+08:00").unwrap();
    println!("utc={}", utc);
    let date: Date = utc.clone().into();
    let time: Time = utc.into();
    println!("date={},time={}", date, time);
    assert_eq!("2022-12-12", date.to_string());
    assert_eq!("08:00:00", time.to_string());
}

#[test]
fn test_befor_after() {
    let date1 = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    let date2 = DateTime::from_str("2022-12-12 01:00:00").unwrap();
    assert_eq!(date2.after(&date1), true);
    assert_eq!(date1.before(&date2), true);
}

#[test]
fn test_parse_z() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000Z").unwrap();
    assert_eq!("2022-12-12 08:00:00", date.to_string());
}

#[test]
fn test_parse_z_add() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000+09:00").unwrap();
    let date_offset = date.clone();
    assert_eq!("2022-12-12 09:00:00", date_offset.to_string());
}

#[test]
fn test_parse_z_sub() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000-09:00").unwrap();
    let date_offset = date.clone();
    assert_eq!("2022-12-11 15:00:00", date_offset.to_string());
}

#[test]
fn test_set_offset_sub() {
    let mut date = DateTime::from_str("2022-12-12 09:00:00").unwrap();
    date = date.set_offset(-9 * 3600);
    assert_eq!("2022-12-12 00:00:00", date.to_string());
}

#[test]
fn test_time_sub_time() {
    let date = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    let date2 = DateTime::from_str("2022-12-11 00:00:00").unwrap();
    let sub = date - date2;
    assert_eq!(86400, sub.as_secs());
}

#[test]
fn test_parse_format() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2022-12-13 11:12:14.123456").unwrap();
    println!("{}", date);
}

#[test]
fn test_parse_format2() {
    let date = DateTime::parse("hh:mm:ss.000000,YYYY-MM-DD", "11:12:14.123456,2022-12-13").unwrap();
    println!("{}", date);
}

#[test]
fn test_week() {
    let date = DateTime::from_str("2022-07-27 00:27:11.000000").unwrap();
    println!("week,{}", date.week_day());
    assert_eq!(3, date.week_day());
}

#[test]
fn test_nano() {
    let date1 = DateTime::from_str("2019-04-28 00:00:00.023333333").unwrap();
    println!("{}", date1.to_string());
    assert_eq!("2019-04-28 00:00:00.023333333", date1.to_string());
}

#[test]
fn test_nano_more_than() {
    let date1 = DateTime::from_str("2019-04-28 00:00:00.0233333333");
    assert_eq!(date1.err().unwrap().to_string(), "SecondFractionTooLong");
}

#[test]
fn test_parse_date() {
    let date = DateTime::from_str("2013-10-06").unwrap();
    assert_eq!(date.to_string(), "2013-10-06 00:00:00");
}

#[test]
fn test_ser_date() {
    let date = DateTime::from_str("2022-09-19 14:01:58.175861").unwrap();
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!("\"2022-09-19 14:01:58.175861\"", js);
}

#[test]
fn test_add_minute() {
    let date = DateTime::from_str("2013-10-06").unwrap().add(Duration::from_minute(1));
    assert_eq!(date.to_string(), "2013-10-06 00:01:00");
}

#[test]
fn test_add_hour() {
    let date = DateTime::from_str("2013-10-06").unwrap().add(Duration::from_hour(1));
    assert_eq!(date.to_string(), "2013-10-06 01:00:00");
}

#[test]
fn test_add_day() {
    let date = DateTime::from_str("2013-10-06").unwrap().add(Duration::from_day(1));
    assert_eq!(date.to_string(), "2013-10-07 00:00:00");
}