use json;
use std::str::FromStr;

#[test]
fn test_null() {
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("null"));
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("   null"));
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("null   "));
    assert_eq!(Ok(json::Value::Null), json::Value::from_str("  null   "));
    assert_ne!(Ok(json::Value::Null), json::Value::from_str("  null 1"));
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
    assert_eq!(
        Ok(json::Value::String("hello\nこんにちは".to_string())),
        json::Value::from_str(r#" "hello\nこんにちは" "#)
    );
    assert_eq!(
        Ok(json::Value::String("\"\\/\x08\x0C\n\r\t".to_string())),
        json::Value::from_str(r#" "\"\\\/\b\f\n\r\t" "#)
    );
    assert_eq!(
        Ok(json::Value::String("\u{AB12}".to_string())),
        json::Value::from_str(r#" "\uAb12" "#)
    );
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
