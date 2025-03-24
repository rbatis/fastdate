use fastdate::Time;
use std::str::FromStr;
use std::time::Duration;

#[test]
fn test_time_empty() {
    let d = Time::from_str("");
    assert!(d.is_err());
}

#[test]
fn test_time_empty2() {
    let d = Time::from_str("111");
    assert!(d.is_err());
}

#[test]
fn test_time_hour_out() {
    let d = Time::from_str("66:04:05.000000");
    assert!(d.is_err());
}

#[test]
fn test_time_minute_out() {
    let d = Time::from_str("01:66:05.000000");
    assert!(d.is_err());
}

#[test]
fn test_time_sec_out() {
    let d = Time::from_str("01:00:66.000000");
    assert!(d.is_err());
}

#[test]
fn test_time_nano_tolong() {
    let d = Time::from_str("01:00:66.000000000001");
    assert!(d.is_err());
}

#[test]
fn test_time_nano_miss() {
    let d = Time::from_str("01:00:00.");
    println!("{}", d.clone().err().unwrap());
    assert!(d.is_err());
}

#[test]
fn test_set_nano() {
    let d = Time::from_str("01:00:00").unwrap().set_nano(1);
    assert_eq!(d.get_nano(), 1);
}

#[test]
fn test_set_micro() {
    let d = Time::from_str("01:00:00").unwrap().set_micro(1);
    assert_eq!(d.get_micro(), 1);
}

#[test]
fn test_set_sec() {
    let d = Time::from_str("01:00:00").unwrap().set_sec(1);
    assert_eq!(d.get_sec(), 1);
}

#[test]
fn test_set_minute() {
    let d = Time::from_str("01:00:00").unwrap().set_minute(1);
    assert_eq!(d.get_minute(), 1);
}

#[test]
fn test_set_hour() {
    let d = Time::from_str("01:00:00").unwrap().set_hour(1);
    assert_eq!(d.get_hour(), 1);
}

#[test]
fn test_display() {
    let d = Time {
        nano: 0,
        sec: 0,
        minute: 0,
        hour: 8,
    };
    assert_eq!("08:00:00", d.to_string());
}

#[test]
fn test_display2() {
    let d = Time {
        nano: 123456,
        sec: 0,
        minute: 0,
        hour: 8,
    };
    assert_eq!("08:00:00.000123456", format!("{}", d));
}

#[test]
fn test_date_12345678910() {
    let d = Time::from_str("11:12:13.12345678910");
    assert_eq!("SecondFractionTooLong", d.err().unwrap().to_string());
}

#[test]
fn test_date_123456789() {
    let d = Time::from_str("11:12:13.123456789").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.123456789".to_string(), d.to_string());
}

#[test]
fn test_date_12345678() {
    let d = Time::from_str("11:12:13.12345678").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.12345678".to_string(), d.to_string());
}

#[test]
fn test_date_1234567() {
    let d = Time::from_str("11:12:13.1234567").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.1234567".to_string(), d.to_string());
}

#[test]
fn test_date_123456() {
    let d = Time::from_str("11:12:13.123456").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.123456".to_string(), d.to_string());
}

#[test]
fn test_date_12345() {
    let d = Time::from_str("11:12:13.12345").unwrap();
    println!("{}=>{:?}", d, d);
    assert_eq!("11:12:13.12345".to_string(), d.to_string());
}

#[test]
fn test_date_1234() {
    let d = Time::from_str("11:12:13.1234").unwrap();
    println!("{}=>{:?}", d, d);
    assert_eq!("11:12:13.1234".to_string(), d.to_string());
}

#[test]
fn test_date_123() {
    let d = Time::from_str("11:12:13.123").unwrap();
    println!("{}=>{:?}", d, d);
    assert_eq!("11:12:13.123".to_string(), d.to_string());
}

#[test]
fn test_date_12() {
    let d = Time::from_str("11:12:13.12").unwrap();
    println!("{}=>{:?}", d, d);
    assert_eq!("11:12:13.12".to_string(), d.to_string());
}

#[test]
fn test_from_micros() {
    let d = Duration::from_micros(3 * 60 * 60 * 1000000 + 1);
    let t = Time::from(d);
    println!("{}", t);
    assert_eq!(t.to_string(), "03:00:00.000001");
}

#[test]
fn test_from_time() {
    let d = Duration::from(Time {
        nano: 1000,
        sec: 1,
        minute: 1,
        hour: 1,
    });
    println!("{}", d.as_nanos());
    assert_eq!(d.as_nanos(), 3661000001000);
}

#[test]
fn test_display_time_nano_zero() {
    let d = Time {
        nano: 0,
        sec: 0,
        minute: 0,
        hour: 0,
    };
    let mut buf: [u8; 18] = *b"00:00:00.000000000";
    let len = d.display_time(0, &mut buf);
    assert_eq!(len, 8);
}

#[test]
fn test_display_time_nano_zero_no() {
    let d = Time {
        nano: 1,
        sec: 0,
        minute: 0,
        hour: 0,
    };
    let mut buf: [u8; 18] = *b"00:00:00.000000000";
    let len = d.display_time(0, &mut buf);
    println!("{}", std::str::from_utf8(&buf).unwrap());
    assert_eq!(len, 18);
}

#[test]
fn test_ser_de() {
    let date = Time::from_str("14:01:58.175861").unwrap();
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!("\"14:01:58.175861\"", js);
    let r: Time = serde_json::from_str(&js).unwrap();
    assert_eq!(r, date);
}

#[test]
fn test_get_micro() {
    let date = Time::from_str("14:01:58.175861").unwrap();
    assert_eq!(175861, date.get_micro());
}
