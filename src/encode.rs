#[cfg(feature = "log")]
use log;

use super::IntegerOrFloat::{self, Integer};

use num_traits::*;

impl Zero for IntegerOrFloat {
    fn zero() -> Self {
        Integer(0)
    }
    fn is_zero(&self) -> bool {
        *self == Integer(0)
    }
}

impl One for IntegerOrFloat {
    fn one() -> Self {
        Integer(1)
    }
    fn is_one(&self) -> bool {
        *self == Integer(1)
    }
}

impl IntegerOrFloat {
    pub fn from_bits(bits: impl Into<u32>) -> Self {
        Self::Float(f32::from_bits(bits.into()))
    }
    pub fn to_bits(&self) -> u32 {
        match self {
            IntegerOrFloat::Float(f) => {
                f.to_bits()
            }
            IntegerOrFloat::Integer(i) => {
                #[cfg(feature = "log")]
                log::warn!("Calling to_bits(…) on an integer to encode it as a float is almost certainly not what you want.");
                (*i as f32).to_bits()
            }
        }
    }
}
