use crate::{IntegerOrFloat::self, *};

use float_cmp::{ApproxEq, F32Margin, Ulps};

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
