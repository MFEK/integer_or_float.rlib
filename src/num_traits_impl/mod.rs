use super::IntegerOrFloat::{self, *};
use crate::f_iof;
use num_traits::{cast::ToPrimitive, Zero, One, NumCast};

mod float;

#[cfg(feature = "num-traits")]
impl ToPrimitive for IntegerOrFloat {
    fn to_u64(&self) -> Option<u64> {
        match self {
            Integer(i) => i.to_u64(),
            Float(f) => f.to_u64(),
        }
    }
    fn to_i64(&self) -> Option<i64> {
        match self {
            Integer(i) => i.to_i64(),
            Float(f) => f.to_i64(),
        }
    }
}

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

impl NumCast for IntegerOrFloat {
    fn from<N: ToPrimitive>(num: N) -> Option<IntegerOrFloat> {
        num.to_f64().map(|n| IntegerOrFloat::Float(n as f_iof))
    }
}
