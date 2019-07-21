pub fn number(user_number: &str) -> Option<String> {
    let s: String = user_number.chars().filter(|ch| ch.is_digit(10)).collect();
    match s.len() {
        10 => validate(s),
        11 => match s.chars().nth(0).unwrap() {
            '1' => validate(s.get(1..).unwrap().to_string()),
            _ => None,
        },
        _ => None,
    }
}

fn validate(s: String) -> Option<String> {
    if s.chars().nth(0).unwrap().to_digit(10).unwrap() < 2
        || s.chars().nth(3).unwrap().to_digit(10).unwrap() < 2
    {
        None
    } else {
        Some(s)
    }
}
