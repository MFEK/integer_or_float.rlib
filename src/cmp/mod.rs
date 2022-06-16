#[cfg(feature = "float-cmp")]
mod approx;

use core::cmp::Ordering;

use crate::{f_iof, i_iof};
use super::{IntegerOrFloat::self, *};

impl IntegerOrFloat {
    pub fn holding_integer(&self) -> Result<i_iof, f_iof> {
        match self {
            Integer(i) => Ok(*i),
            Float(f) => Err(*f)
        }
    }

    pub fn holding_float(&self) -> Result<f_iof, i_iof> {
        match self {
            Integer(i) => Err(*i),
            Float(f) => Ok(*f)
        }
    }

    /// Give back the float we're holding…panic if we're holding an integer.
    pub fn unwrap_float(&self) -> f_iof {
        match self.holding_float() {
            Ok(f) => f,
            Err(i) => panic!("IntegerOrFloat was holding integer {}, not a float!", i)
        }
    }

    /// Give back the float we're holding…panic if we're holding an integer.
    pub fn unwrap_integer(&self) -> i_iof {
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
            (Ok(i), Err(f)) | (Err(f), Ok(i)) => i as f_iof == f,
        }
    }
}

impl PartialOrd<IntegerOrFloat> for IntegerOrFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.holding_integer(), other.holding_integer()) {
            (Ok(i1), Ok(i2)) => i1.partial_cmp(&i2),
            (Err(f1), Err(f2)) => f1.partial_cmp(&f2),
            (Ok(i), Err(f)) | (Err(f), Ok(i)) => (i as f_iof).partial_cmp(&f),
        }
    }
}
