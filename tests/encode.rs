#![cfg(not(feature = "x64-backing-store"))]

use integer_or_float::IntegerOrFloat::{self};

#[test]
fn test_encode() {
    eprintln!("{:b}", f32::NAN.to_bits());
    eprintln!("{:b}", f32::INFINITY.to_bits());
    eprintln!("{:b}", 0.1f32.to_bits());
    assert!(f32::from_bits(0b1111111110000000000000000000000).is_nan());
    assert_eq!(f32::INFINITY, f32::from_bits(0b1111111100000000000000000000000));
    assert_eq!(0.1, f32::from_bits(0b0111101110011001100110011001101));
    assert!(f32::from(IntegerOrFloat::from_bits(0b1111111110000000000000000000000)).is_nan());
    assert_eq!(f32::INFINITY, f32::from(IntegerOrFloat::from_bits(0b1111111100000000000000000000000)));
    assert_eq!(0.1, f32::from(IntegerOrFloat::from_bits(0b0111101110011001100110011001101)));
}
