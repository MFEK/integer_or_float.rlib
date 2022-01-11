pub type MantissaExpSignTriplet = (u64, i16, i8);
pub trait IntegerEncode {
    fn integer_encode(mest: MantissaExpSignTriplet) -> f64 {
        let (mantissa, exponent, sign) = mest;
        let sign_f = sign as f64;
        let mantissa_f = mantissa as f64;
        let exponent_f = 2.0_f64.powf(exponent as f64);
        sign_f * mantissa_f * exponent_f
    }   
}
impl IntegerEncode for MantissaExpSignTriplet {}
#[cfg(feature = "hash")]
pub use crate::hash::FromEncoded;
