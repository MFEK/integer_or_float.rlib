#[cfg(feature = "log")]
use log;

use super::IntegerOrFloat;
#[allow(unused_imports)]
use crate::{f_iof, i_iof, u_iof};

/// These functions should never be used when you're holding the [`IntegerOrFloat::Integer`] variant.
///
/// When [`log::warn`] is available, we warn you.
impl IntegerOrFloat {
    /// Cf. [`f_iof::from_bits`]
    pub fn from_bits(bits: u_iof) -> Self {
        Self::Float(f_iof::from_bits(bits))
    }
    /// Cf. [`f_iof::to_bits`]
    pub fn to_bits(&self) -> u_iof {
        match self {
            IntegerOrFloat::Float(f) => f.to_bits(),
            IntegerOrFloat::Integer(i) => {
                #[cfg(feature = "log")]
                log::warn!(
                    "Calling to_bits(…) on an integer to encode it as a float is almost certainly not what you want."
                );
                (*i as f_iof).to_bits()
            }
        }
    }
}
