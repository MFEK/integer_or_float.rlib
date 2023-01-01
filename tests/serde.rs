#![cfg(feature = "more-serde")]

use integer_or_float::IntegerOrFloat::{self, *};
use serde_json;

#[test]
fn test_serde() {
    let i = Integer(7);
    let j = serde_json::to_string(&i).unwrap();
    assert_eq!(&j, "{\"Integer\":7}");
    let f = Float(7.77);
    let j = serde_json::to_string(&f).unwrap();
    assert_eq!(&j, "{\"Float\":7.77}");
    let f2: IntegerOrFloat = serde_json::from_str(&j).unwrap();
    assert_eq!(f, f2);
}
