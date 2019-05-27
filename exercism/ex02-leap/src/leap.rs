pub fn is_leap(year: u64) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    if year % 4 == 0 {
        return true;
    }
    false
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
