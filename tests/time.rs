use fastdate::Time;
use std::str::FromStr;
use std::time::Duration;

#[test]
fn test_date_123456() {
    let d = Time::from_str("11:12:13.123456").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.123456".to_string(), d.to_string());
}

#[test]
fn test_date_12345() {
    let d = Time::from_str("11:12:13.12345").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.012345".to_string(), d.to_string());
}

#[test]
fn test_date_1234() {
    let d = Time::from_str("11:12:13.1234").unwrap();
    println!("{}", d);
    assert_eq!("11:12:13.001234".to_string(), d.to_string());
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
        micro: 1,
        sec: 1,
        min: 1,
        hour: 1,
    });
    println!("{}", d.as_micros());
}
