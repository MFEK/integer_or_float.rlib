#[cfg(feature = "log")]
use log;

use super::IntegerOrFloat;

impl IntegerOrFloat {
    pub fn from_bits(bits: u32) -> Self {
        Self::Float(f32::from_bits(bits))
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
