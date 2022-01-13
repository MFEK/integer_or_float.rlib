use core::cmp::Ordering;

use float_cmp::{ApproxEq, F32Margin, Ulps};

use super::{IntegerOrFloat::self, *};

impl IntegerOrFloat {
    pub fn holding_integer(&self) -> Result<i32, f32> {
        match self {
            Integer(i) => Ok(*i),
            Float(f) => Err(*f)
        }
    }

    pub fn holding_float(&self) -> Result<f32, i32> {
        match self {
            Integer(i) => Err(*i),
            Float(f) => Ok(*f)
        }
    }

    /// Give back the float we're holding…panic if we're holding an integer.
    pub fn unwrap_float(&self) -> f32 {
        match self.holding_float() {
            Ok(f) => f,
            Err(i) => panic!("IntegerOrFloat was holding integer {}, not a float!", i)
        }
    }

    /// Give back the float we're holding…panic if we're holding an integer.
    pub fn unwrap_integer(&self) -> i32 {
        match self.holding_integer() {
            Ok(i) => i,
            Err(f) => panic!("IntegerOrFloat was holding float {}, not an integer!", f)
        }
    }
}

impl PartialEq<IntegerOrFloat> for IntegerOrFloat {
    fn eq(&self, other: &Self) -> bool {
        match (self.holding_integer(), other.holding_integer()) {
            (Ok(i1), Ok(i2)) => i1 == i2,
            (Err(f1), Err(f2)) => f1 == f2,
            (Ok(i), Err(f)) | (Err(f), Ok(i)) => i as f32 == f,
        }
    }
}

impl PartialOrd<IntegerOrFloat> for IntegerOrFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.holding_integer(), other.holding_integer()) {
            (Ok(i1), Ok(i2)) => i1.partial_cmp(&i2),
            (Err(f1), Err(f2)) => f1.partial_cmp(&f2),
            (Ok(i), Err(f)) | (Err(f), Ok(i)) => (i as f32).partial_cmp(&f),
        }
    }
}

impl ApproxEq for IntegerOrFloat {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin: F32Margin = margin.into();
        if let (Ok(i1), Ok(i2), true) = (self.holding_integer(), other.holding_integer(), 0.0 == margin.epsilon && margin.ulps == F32Margin::zero().ulps) {
            return i1 == i2
        }

        f32::from(self).approx_eq(other.into(), margin)
    }
}

impl Ulps for IntegerOrFloat {
    type U = Self;

    fn ulps(&self, other: &Self) -> Self {
        Integer(self.unwrap_float().ulps(&other.unwrap_float()))
    }
    fn next(&self) -> Self {
        Float(self.unwrap_float().next())
    }
    fn prev(&self) -> Self {
        Float(self.unwrap_float().prev())
    }
}
