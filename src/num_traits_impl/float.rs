use core::num::FpCategory;
use crate::IntegerOrFloat as IOF;
use num_traits::Float;

impl Float for IOF {
    // Statics
    fn nan() -> Self { IOF::Float(f32::NAN) }
    fn infinity() -> Self { IOF::Float(f32::INFINITY) }
    fn neg_infinity() -> Self { IOF::Float(-f32::INFINITY) }
    fn neg_zero() -> Self { IOF::Float(-0.0f32) }
    fn min_value() -> Self { IOF::Float(f32::MIN) }
    fn min_positive_value() -> Self { IOF::Float(f32::MIN_POSITIVE) }
    fn max_value() -> Self { IOF::Float(f32::MAX) }
    // bool-returing
    fn is_nan(self) -> bool { f32::from(self).is_nan() }
    fn is_infinite(self) -> bool { f32::from(self).is_infinite() }
    fn is_finite(self) -> bool { f32::from(self).is_finite() }
    fn is_normal(self) -> bool { f32::from(self).is_normal() }
    fn is_sign_positive(self) -> bool { f32::from(self).is_sign_positive() }
    fn is_sign_negative(self) -> bool { f32::from(self).is_sign_negative() }
    // FpCategory-returning
    fn classify(self) -> FpCategory { f32::from(self).classify() }
    // Self → Self functions
    fn floor(self) -> Self { IOF::Float(f32::from(self).floor()) }
    fn ceil(self) -> Self { IOF::Float(f32::from(self).ceil()) }
    fn round(self) -> Self { IOF::Float(f32::from(self).round()) }
    fn trunc(self) -> Self { IOF::Float(f32::from(self).trunc()) }
    fn fract(self) -> Self { IOF::Float(f32::from(self).fract()) }
    fn abs(self) -> Self { IOF::Float(f32::from(self).abs()) }
    fn signum(self) -> Self { IOF::Float(f32::from(self).signum()) }
    fn recip(self) -> Self { IOF::Float(f32::from(self).recip()) }
    fn sqrt(self) -> Self { IOF::Float(f32::from(self).sqrt()) }
    fn exp(self) -> Self { IOF::Float(f32::from(self).exp()) }
    fn exp2(self) -> Self { IOF::Float(f32::from(self).exp2()) }
    fn ln(self) -> Self { IOF::Float(f32::from(self).ln()) }
    fn log2(self) -> Self { IOF::Float(f32::from(self).log2()) }
    fn log10(self) -> Self { IOF::Float(f32::from(self).log10()) }
    fn cbrt(self) -> Self { IOF::Float(f32::from(self).cbrt()) }
    fn sin(self) -> Self { IOF::Float(f32::from(self).sin()) }
    fn cos(self) -> Self { IOF::Float(f32::from(self).cos()) }
    fn tan(self) -> Self { IOF::Float(f32::from(self).tan()) }
    fn asin(self) -> Self { IOF::Float(f32::from(self).asin()) }
    fn acos(self) -> Self { IOF::Float(f32::from(self).acos()) }
    fn atan(self) -> Self { IOF::Float(f32::from(self).atan()) }
    fn exp_m1(self) -> Self { IOF::Float(f32::from(self).exp_m1()) }
    fn ln_1p(self) -> Self { IOF::Float(f32::from(self).ln_1p()) }
    fn sinh(self) -> Self { IOF::Float(f32::from(self).sinh()) }
    fn cosh(self) -> Self { IOF::Float(f32::from(self).cosh()) }
    fn tanh(self) -> Self { IOF::Float(f32::from(self).tanh()) }
    fn asinh(self) -> Self { IOF::Float(f32::from(self).asinh()) }
    fn acosh(self) -> Self { IOF::Float(f32::from(self).acosh()) }
    fn atanh(self) -> Self { IOF::Float(f32::from(self).atanh()) }
    // has rhs & rhs2
    fn mul_add(self, rhs: Self, rhs2: Self) -> Self { IOF::Float(f32::from(self).mul_add(f32::from(rhs), f32::from(rhs2))) }
    // has rhs
    fn powi(self, rhs: i32) -> Self { IOF::Float(f32::from(self).powi(rhs)) }
    fn powf(self, rhs: Self) -> Self { IOF::Float(f32::from(self).powf(f32::from(rhs))) }
    fn log(self, rhs: Self) -> Self { IOF::Float(f32::from(self).log(f32::from(rhs))) }
    fn max(self, rhs: Self) -> Self { IOF::Float(f32::from(self).max(f32::from(rhs))) }
    fn min(self, rhs: Self) -> Self { IOF::Float(f32::from(self).min(f32::from(rhs))) }
    fn hypot(self, rhs: Self) -> Self { IOF::Float(f32::from(self).hypot(f32::from(rhs))) }
    fn atan2(self, rhs: Self) -> Self { IOF::Float(f32::from(self).atan2(f32::from(rhs))) }
    // specials
    fn integer_decode(self) -> (u64, i16, i8) { f32::from(self).integer_decode() }
    fn sin_cos(self) -> (Self, Self) {
        let (r1, r2) = f32::from(self).sin_cos();
        (IOF::Float(r1), IOF::Float(r2))
    }
    #[allow(deprecated)]
    fn abs_sub(self, rhs: Self) -> Self { IOF::Float(f32::from(self).abs_sub(f32::from(rhs))) }
}
