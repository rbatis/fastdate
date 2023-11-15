use fastdate::{offset_sec, DateTime};
use std::time::Duration;

#[test]
fn test_offset_sec() {
    let d = offset_sec();
    println!("{}", d);
}

#[test]
fn test_offset_time() {
    let z_time = DateTime::now().sub(Duration::from_secs(offset_sec() as u64));
    println!("{}", z_time);
}
