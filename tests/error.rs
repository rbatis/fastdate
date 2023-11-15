use fastdate::error::Error;

#[test]
fn test_from_str() {
    let e = Error::from("e");
    assert_eq!(e.to_string(), "e");
}

#[test]
fn test_from_string() {
    let e = Error::from("e".to_string());
    assert_eq!(e.to_string(), "e");
}

#[test]
fn test_display() {
    let e = Error::from("e".to_string());
    assert_eq!(format!("{}", e), "e");
}

#[test]
fn test_default() {
    let e = Error::default();
    assert_eq!(format!("{}", e), "");
}
