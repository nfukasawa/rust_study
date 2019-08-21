use json;
use std::str::FromStr;

#[test]
fn test_null() {
    assert_eq!(Ok(json::value!(null)), json::Value::from_str("null"));
}

#[test]
fn test_boolean() {
    let cases: Vec<(&str, bool)> = vec![("true", true), ("false", false)];
    for case in cases {
        assert_eq!(Ok(json::value!(case.1)), json::Value::from_str(case.0));
    }
}

#[test]
fn test_string() {
    let cases: Vec<(&str, &str)> = vec![
        (r#" "" "#, ""),
        (r#" "hello\nこんにちは" "#, "hello\nこんにちは"),
        (r#" "\"\\\/\b\f\n\r\t" "#, "\"\\/\x08\x0C\n\r\t"),
        (r#" "\uAb12" "#, "\u{AB12}"),
        (r#" "𩸽" "#, "𩸽"),
        (r#" "🤔" "#, "🤔"),
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
        ("1234", 1234 as f64),
        ("1234 ", 1234 as f64),
        ("-1234", -1234 as f64),
        ("0.123", 0.123 as f64),
        ("-0.123", -0.123 as f64),
        ("1.234", 1.234 as f64),
        ("1234e+3", 1234e+3 as f64),
        ("0.123e-3", 0.123e-3 as f64),
    ];

    for case in cases {
        assert_eq!(Ok(json::value!(case.1)), json::Value::from_str(case.0));
    }
}

#[test]
fn test_array() {
    assert_eq!(
        Ok(json::value!([123, "abc", true, ["foo", "bar"]])),
        json::Value::from_str(
            r#"[ 
            123  , 
            "abc",
            true  , 
            ["foo", "bar"]
            ]"#
        )
    )
}

#[test]
fn test_object() {
    assert_eq!(
        Ok(json::value!({
            "num": 123,
            "str": "abc",
            "bool": true,
            "obj": {
                "one": 1,
                "two": 2
            }
        })),
        json::Value::from_str(
            r#"{ 
            "num" :  123  , 
            "str"  : "abc"  ,  
            "bool":   true,  
            "obj" : {"one":1,"two":2}
            }"#
        )
    )
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
