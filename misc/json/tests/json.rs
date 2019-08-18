use json;
use std::str::FromStr;

#[test]
fn test_null() {
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("null"));
}

#[test]
fn test_boolean() {
    assert_eq!(
        Ok(json::Value::Boolean(true)),
        json::Value::from_str("true")
    );
    assert_eq!(
        Ok(json::Value::Boolean(false)),
        json::Value::from_str("false")
    );
}

#[test]
fn test_string() {
    let cases: Vec<(&str, &str)> = vec![
        (r#" "" "#, ""),
        (r#" "hello\nこんにちは" "#, "hello\nこんにちは"),
        (r#" "\"\\\/\b\f\n\r\t" "#, "\"\\/\x08\x0C\n\r\t"),
        (r#" "\uAb12" "#, "\u{AB12}"),
    ];

    for case in cases {
        assert_eq!(
            Ok(json::Value::String(case.1.to_string())),
            json::Value::from_str(case.0)
        );
    }
}

#[test]
fn test_number() {
    let cases: Vec<(&str, f64)> = vec![
        ("123456789", 123456789 as f64),
        ("1234 ", 1234 as f64),
        ("-1234 ", -1234 as f64),
        ("0.123 ", 0.123 as f64),
        ("-0.123 ", -0.123 as f64),
        ("1.234 ", 1.234 as f64),
        ("1234e+3 ", 1234e+3 as f64),
        ("0.123e-3 ", 0.123e-3 as f64),
    ];

    for case in cases {
        assert_eq!(
            Ok(json::Value::Number(case.1)),
            json::Value::from_str(case.0)
        );
    }
}

#[test]
fn test_spaces() {
    assert_eq!(
        Ok(json::Value::Null),
        json::Value::from_str("\n\t\r\n null")
    );
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("null   "));
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("  null   "));
    // TODO: error test
    assert_ne!(Ok(json::Value::Null), json::Value::from_str("  null 1"));
}
