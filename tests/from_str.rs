use integer_or_float::IntegerOrFloat::{self, *};

#[test]
fn test_from_str() {
    assert_eq!("12".parse::<IntegerOrFloat>().unwrap(), Integer(12));
    assert_eq!("12.0".parse::<IntegerOrFloat>().unwrap(), Float(12.0));
}

#[test]
fn test_from_str_subnormal_nan() {
    assert!(f32::from("NaN".parse::<IntegerOrFloat>().unwrap()).is_nan());
    assert!(f32::from("INF".parse::<IntegerOrFloat>().unwrap()).is_infinite());
    assert!(f64::from("NaN".parse::<IntegerOrFloat>().unwrap()).is_nan());
    assert!(f64::from("INF".parse::<IntegerOrFloat>().unwrap()).is_infinite());
}

#[test]
fn test_from_str_fail() {
    assert!("12.v0".parse::<IntegerOrFloat>().is_err());
    assert!("nanners".parse::<IntegerOrFloat>().is_err());
}
