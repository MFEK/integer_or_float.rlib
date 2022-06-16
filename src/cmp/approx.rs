use crate::{IntegerOrFloat, *};

#[cfg(not(feature = "x64-backing-store"))]
use float_cmp::F32Margin as FMargin;
#[cfg(feature = "x64-backing-store")]
use float_cmp::F64Margin as FMargin;
use float_cmp::{ApproxEq, Ulps};

impl ApproxEq for IntegerOrFloat {
    type Margin = FMargin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin: FMargin = margin.into();
        if let (Ok(i1), Ok(i2), true) = (
            self.holding_integer(),
            other.holding_integer(),
            0.0 == margin.epsilon && margin.ulps == FMargin::zero().ulps,
        ) {
            return i1 == i2;
        }

        f_iof::from(self).approx_eq(other.into(), margin)
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
