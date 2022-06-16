use crate::f_iof;
use crate::IntegerOrFloat as IOF;
use core::num::FpCategory;
use num_traits::Float;

#[rustfmt::skip]
impl Float for IOF {
    // Statics
    fn nan() -> Self { IOF::Float(f_iof::NAN) }
    fn infinity() -> Self { IOF::Float(f_iof::INFINITY) }
    fn neg_infinity() -> Self { IOF::Float(-f_iof::INFINITY) }
    fn neg_zero() -> Self { IOF::Float(-f_iof::from(0.0)) }
    fn min_value() -> Self { IOF::Float(f_iof::MIN) }
    fn min_positive_value() -> Self { IOF::Float(f_iof::MIN_POSITIVE) }
    fn max_value() -> Self { IOF::Float(f_iof::MAX) }
    // bool-returing
    fn is_nan(self) -> bool { f_iof::from(self).is_nan() }
    fn is_infinite(self) -> bool { f_iof::from(self).is_infinite() }
    fn is_finite(self) -> bool { f_iof::from(self).is_finite() }
    fn is_normal(self) -> bool { f_iof::from(self).is_normal() }
    fn is_sign_positive(self) -> bool { f_iof::from(self).is_sign_positive() }
    fn is_sign_negative(self) -> bool { f_iof::from(self).is_sign_negative() }
    // FpCategory-returning
    fn classify(self) -> FpCategory { f_iof::from(self).classify() }
    // Self → Self functions
    fn floor(self) -> Self { IOF::Float(f_iof::from(self).floor()) }
    fn ceil(self) -> Self { IOF::Float(f_iof::from(self).ceil()) }
    fn round(self) -> Self { IOF::Float(f_iof::from(self).round()) }
    fn trunc(self) -> Self { IOF::Float(f_iof::from(self).trunc()) }
    fn fract(self) -> Self { IOF::Float(f_iof::from(self).fract()) }
    fn abs(self) -> Self { IOF::Float(f_iof::from(self).abs()) }
    fn signum(self) -> Self { IOF::Float(f_iof::from(self).signum()) }
    fn recip(self) -> Self { IOF::Float(f_iof::from(self).recip()) }
    fn sqrt(self) -> Self { IOF::Float(f_iof::from(self).sqrt()) }
    fn exp(self) -> Self { IOF::Float(f_iof::from(self).exp()) }
    fn exp2(self) -> Self { IOF::Float(f_iof::from(self).exp2()) }
    fn ln(self) -> Self { IOF::Float(f_iof::from(self).ln()) }
    fn log2(self) -> Self { IOF::Float(f_iof::from(self).log2()) }
    fn log10(self) -> Self { IOF::Float(f_iof::from(self).log10()) }
    fn cbrt(self) -> Self { IOF::Float(f_iof::from(self).cbrt()) }
    fn sin(self) -> Self { IOF::Float(f_iof::from(self).sin()) }
    fn cos(self) -> Self { IOF::Float(f_iof::from(self).cos()) }
    fn tan(self) -> Self { IOF::Float(f_iof::from(self).tan()) }
    fn asin(self) -> Self { IOF::Float(f_iof::from(self).asin()) }
    fn acos(self) -> Self { IOF::Float(f_iof::from(self).acos()) }
    fn atan(self) -> Self { IOF::Float(f_iof::from(self).atan()) }
    fn exp_m1(self) -> Self { IOF::Float(f_iof::from(self).exp_m1()) }
    fn ln_1p(self) -> Self { IOF::Float(f_iof::from(self).ln_1p()) }
    fn sinh(self) -> Self { IOF::Float(f_iof::from(self).sinh()) }
    fn cosh(self) -> Self { IOF::Float(f_iof::from(self).cosh()) }
    fn tanh(self) -> Self { IOF::Float(f_iof::from(self).tanh()) }
    fn asinh(self) -> Self { IOF::Float(f_iof::from(self).asinh()) }
    fn acosh(self) -> Self { IOF::Float(f_iof::from(self).acosh()) }
    fn atanh(self) -> Self { IOF::Float(f_iof::from(self).atanh()) }
    // has rhs & rhs2
    fn mul_add(self, rhs: Self, rhs2: Self) -> Self { IOF::Float(f_iof::from(self).mul_add(f_iof::from(rhs), f_iof::from(rhs2))) }
    // has rhs
    /// note: powi always i32 regardless of i_iof
    fn powi(self, rhs: i32) -> Self { IOF::Float(f_iof::from(self).powi(rhs)) }
    fn powf(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).powf(f_iof::from(rhs))) }
    fn log(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).log(f_iof::from(rhs))) }
    fn max(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).max(f_iof::from(rhs))) }
    fn min(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).min(f_iof::from(rhs))) }
    fn hypot(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).hypot(f_iof::from(rhs))) }
    fn atan2(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).atan2(f_iof::from(rhs))) }
    // specials
    fn integer_decode(self) -> (u64, i16, i8) { f_iof::from(self).integer_decode() }
    fn sin_cos(self) -> (Self, Self) {
        let (r1, r2) = f_iof::from(self).sin_cos();
        (IOF::Float(r1), IOF::Float(r2))
    }
    #[allow(deprecated)]
    fn abs_sub(self, rhs: Self) -> Self { IOF::Float(f_iof::from(self).abs_sub(f_iof::from(rhs))) }
}
