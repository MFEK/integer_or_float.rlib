use core::hash::{Hash, Hasher};
use num_traits::Float as _;

use super::IntegerOrFloat::{self, *};

impl Hash for IntegerOrFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Integer(i) => i.hash(state),
            Float(f) => {
                if f.is_nan() {
                    panic!("Cannot hash a NaN")
                }
                let mest = f.integer_decode();
                mest.hash(state)
            }
        }
    }
}

use crate::encode::*;

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
impl_from_encoded!(f64);
