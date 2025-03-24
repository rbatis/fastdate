use fastdate::{DateTime, offset_sec};
use std::time::Duration;

#[test]
fn test_offset_sec() {
    let d = offset_sec();
    println!("{}", d);
}

#[test]
fn test_offset_time() {
    let z_time = DateTime::now().sub_duration(Duration::from_secs(offset_sec() as u64));
    println!("{}", z_time);
}
