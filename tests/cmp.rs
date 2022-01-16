#![cfg(feature = "float-cmp")]

use integer_or_float::IntegerOrFloat::{self, *};
use float_cmp::{ApproxEq, F32Margin};

static DEFAULT_MARGIN: (f32, i32) = (f32::EPSILON, 4);
static ZERO_MARGIN: (f32, i32) = (0.0, 0);

#[test]
fn test_cmp() {
    // If either of these two asserts fails, float_cmp defaults we're relying on to make reasonable
    // assumptions have broken down!
    assert!(DEFAULT_MARGIN.0 == F32Margin::default().epsilon && DEFAULT_MARGIN.1 == F32Margin::default().ulps);
    assert!(ZERO_MARGIN.0 == F32Margin::zero().epsilon && ZERO_MARGIN.1 == F32Margin::zero().ulps);

    assert!(Float(1.0).approx_eq(Float(1.0), F32Margin::default()));
    assert!(Float(-1.0).approx_eq(Float(-1.0 + f32::EPSILON), F32Margin::default()));
    assert!(Float(-1.0).approx_eq(Float(-1.0 - f32::EPSILON), F32Margin::default()));
    assert!(Float(1.0).approx_eq(Float(1.0 - (f32::EPSILON * 5.0)), F32Margin{epsilon: 0.0, ulps: 10}));
    assert!(Float(1.0).approx_eq(Float(1.0 + (f32::EPSILON * 5.0)), F32Margin{epsilon: 0.0, ulps: 10}));
    assert!(!Float(1.0).approx_eq(Float(1.0 + (f32::EPSILON * 10.0)), F32Margin{epsilon: 0.0, ulps: 1}));
    assert!(Float(1.0 + f32::EPSILON).approx_eq(Integer(1), F32Margin::default()));
    assert!(Integer(1).approx_eq(Integer(1), F32Margin::default()));
    assert!(Integer(0).approx_eq(Integer(0), F32Margin::default()));
    assert!(Integer(i32::MAX).approx_eq(Integer(i32::MAX), F32Margin::default()));
    assert!(Integer(i32::MAX - 2).approx_eq(Integer(i32::MAX), F32Margin::default()));
    assert!(!Integer(i32::MAX - 2).approx_eq(Integer(i32::MAX), F32Margin::zero()));
    assert!(!Integer(i32::MAX - 1).approx_eq(Integer(i32::MAX), F32Margin::zero()));
    assert!(!IntegerOrFloat::from((i32::MAX - 2) as i64).approx_eq(IntegerOrFloat::from((i32::MAX - 1) as i64), F32Margin::zero()));
    assert!(Integer(1).approx_eq(Float(1.0), F32Margin::default()));
}
