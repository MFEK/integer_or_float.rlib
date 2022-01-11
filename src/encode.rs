#[cfg(feature = "no_std")]
use micromath::F32Ext as _;
#[cfg(feature = "no_std")]
macro_rules! _f64 {
    () => (f32)
}
#[cfg(not(feature = "no_std"))]
macro_rules! _f64 {
    () => (f64)
}

fn powf(a: _f64!(), b: _f64!()) -> _f64!() {
    (a as _f64!()).powf(b as _f64!())
}

pub type MantissaExpSignTriplet = (u64, i16, i8);
pub trait IntegerEncode {
    fn integer_encode(mest: MantissaExpSignTriplet) -> _f64!() {
        let (mantissa, exponent, sign) = mest;
        let sign_f = sign as _f64!();
        let mantissa_f = mantissa as _f64!();
        let exponent_f = exponent as _f64!();
        let exponent_2f = powf(2.0, exponent_f);
        sign_f * mantissa_f * exponent_2f
    }   
}
impl IntegerEncode for MantissaExpSignTriplet {}

pub trait FromEncoded: Copy {
    fn from_encoded(mest: impl Into<MantissaExpSignTriplet>) -> Self;
}

macro_rules! impl_from_encoded {
    ($type:ident) => {
        impl FromEncoded for $type {
            fn from_encoded(mest: impl Into<MantissaExpSignTriplet>) -> Self {
                MantissaExpSignTriplet::integer_encode(mest.into()) as $type
            }
        }
    }
}

impl_from_encoded!(f32);
#[cfg(not(feature = "no_std"))]
impl_from_encoded!(f64);
