pub fn is_leap(year: u64) -> bool {
    match year {
        year if year % 400 == 0 => true,
        year if year % 100 == 0 => false,
        year if year % 4 == 0 => true,
        _ => false,
    }
}

#[test]
fn name() {
    assert!(!is_leap(1900));
    assert!(is_leap(2000));
    assert!(!is_leap(2001));
    assert!(!is_leap(2002));
    assert!(!is_leap(2003));
    assert!(is_leap(2004));
}
