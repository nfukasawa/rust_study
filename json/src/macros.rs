#[macro_export]
macro_rules! value {
    (null) => {
        $crate::Value::Null
    };
    ([ $($val:tt),* ]) => {
        {
            let mut arr = Vec::new();
            $(
                arr.push($crate::value!($val));
            )*
            $crate::Value::Array(arr)
        }
    };
    ({ $($key:tt : $val:tt),* }) => {
        {
            let mut obj = ::std::collections::HashMap::new();
            $(
                obj.insert(::std::string::ToString::to_string($key), $crate::value!($val));
            )*
            $crate::Value::Object(Box::new(obj))
        }
    };
    ($val:expr) => {
        $crate::Value::from($val)
    };
}

use super::value::Value;
use std::convert::From;

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Value::Boolean(v)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Value {
        Value::String(v.to_string())
    }
}
impl From<String> for Value {
    fn from(v: String) -> Value {
        Value::String(v)
    }
}
macro_rules! impl_from_numbers_for_value {
    ( $( $t:ident ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Value::Number(v as f64)
                }
            }
        )*
    };
}
impl_from_numbers_for_value!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
