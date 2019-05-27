pub fn is_leap1(year: u64) -> bool {
    match year {
        year if year % 400 == 0 => true,
        year if year % 100 == 0 => false,
        year if year % 4 == 0 => true,
        _ => false,
    }
}

pub fn is_leap2(year: u64) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_is_leap() {
    let cases = vec![
        (1900, false),
        (2000, true),
        (2001, false),
        (2002, false),
        (2003, false),
        (2004, true),
    ];
    for (year, wants) in cases {
        assert_eq!(wants, is_leap1(year));
        assert_eq!(wants, is_leap2(year));
    }
}
