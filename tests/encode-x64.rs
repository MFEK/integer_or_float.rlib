#![cfg(feature = "x64-backing-store")] 

use integer_or_float::IntegerOrFloat::{self};

#[test]
fn test_encode_x64() {
    eprintln!("{:b}", f64::NAN.to_bits());
    eprintln!("{:b}", f64::INFINITY.to_bits());
    eprintln!("{:b}", 0.1f64.to_bits());
    assert!(f64::from_bits(0b111111111111000000000000000000000000000000000000000000000000000).is_nan());
    assert_eq!(f64::INFINITY, f64::from_bits(0b111111111110000000000000000000000000000000000000000000000000000));
    assert_eq!(0.1, f64::from_bits(0b11111110111001100110011001100110011001100110011001100110011010));
    assert!(f64::from(IntegerOrFloat::from_bits(0b111111111111000000000000000000000000000000000000000000000000000)).is_nan());
    assert_eq!(f64::INFINITY, f64::from(IntegerOrFloat::from_bits(0b111111111110000000000000000000000000000000000000000000000000000)));
    assert_eq!(0.1, f64::from(IntegerOrFloat::from_bits(0b11111110111001100110011001100110011001100110011001100110011010)));
}
