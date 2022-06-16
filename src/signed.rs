use super::IntegerOrFloat::{self, *};
#[allow(unused_imports)]
use super::{i_iof, f_iof};

/// A generic container for a "negative integer or a float".
///
/// Was made originally for the UFO data type "negative integer or float".
#[cfg_attr(use_serde, derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Display, Debug, Deref, DerefMut, AsRef, AsMut)]
#[display(fmt = "{}", inner)]
pub struct Negative {
    inner: IntegerOrFloat
}

impl Negative {
    pub fn new(iof: impl Into<IntegerOrFloat>) -> Self {
        let mut inner = iof.into();
        if !(f_iof::from(inner)).is_sign_negative() {
            inner = inner * Float(-1.0);
        }
        Self { inner }
    }
}


/// A generic container for a "positive integer or a float".
#[cfg_attr(use_serde, derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Display, Debug, Deref, DerefMut, AsRef, AsMut)]
#[display(fmt = "{}", inner)]
pub struct Positive {
    inner: IntegerOrFloat
}

impl Positive {
    pub fn new(iof: impl Into<IntegerOrFloat>) -> Self {
        let mut inner = iof.into();
        if !(f_iof::from(inner)).is_sign_positive() {
            inner = inner * Float(-1.0);
        }
        Self { inner }
    }
}
