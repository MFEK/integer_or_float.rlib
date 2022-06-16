//! This is a Rust implementation of the UFO data type "integer or float".
#![cfg_attr(no_std, no_std)]

#[macro_use] extern crate derive_more;

pub mod backing_types;
pub use backing_types::{i_iof, f_iof, u_iof};

#[cfg(with_impl_hash)]
mod hash;

mod cmp;
mod encode;
#[cfg(feature = "num-traits")]
mod num_traits_impl;
mod str_conv;
pub use str_conv::ConversionError;

/// A generic container for an "integer or a float".
///
/// Was made originally for the UFO data type "integer or float".
#[cfg_attr(use_serde, derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone)]
pub enum IntegerOrFloat {
    Integer(i_iof),
    Float(f_iof)
}
pub use IntegerOrFloat::{Integer, Float};
/// The UFO data type "negative integer or float".
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

impl Default for IntegerOrFloat {
    fn default() -> Self {
        IntegerOrFloat::Float(f_iof::default())
    }
}

macro_rules! impl_std_ops_iof_iof {
    ($op:tt, $trait:ident, $fn:ident) => {
        
        impl $trait<IntegerOrFloat> for IntegerOrFloat {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (IntegerOrFloat::Float(f), IntegerOrFloat::Float(f2)) => {
                        IntegerOrFloat::Float(f $op f2)
                    },
                    (IntegerOrFloat::Float(f), IntegerOrFloat::Integer(i2)) => {
                        IntegerOrFloat::Float(f $op i2 as f_iof)
                    },
                    (IntegerOrFloat::Integer(i), IntegerOrFloat::Float(f2)) => {
                        IntegerOrFloat::Float(i as f_iof $op f2)
                    },
                    (IntegerOrFloat::Integer(i), IntegerOrFloat::Integer(i2)) => {
                        IntegerOrFloat::Integer(i $op i2)
                    }
                }
            }
        }

    }
}

macro_rules! impl_std_ops_iof_primitive {
    ($op:tt, $trait:ident, $fn:ident, $rhs:tt, ($coerce_i:tt, $when_int:ident)) => {

        impl $trait<$rhs> for IntegerOrFloat {
            type Output = Self;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                match self {
                    IntegerOrFloat::Float(f) => {
                        IntegerOrFloat::Float(f as f_iof $op rhs as f_iof)
                    },
                    IntegerOrFloat::Integer(i) => {
                        IntegerOrFloat::$when_int(i as $coerce_i $op rhs as $coerce_i)
                    }
                }
            }
        }

    }
}

macro_rules! impl_std_ops_primitive_iof {
    ($op:tt, $trait:ident, $fn:ident, $lhs:tt, ($coerce_i:tt, $when_int:ident)) => {

        impl $trait<IntegerOrFloat> for $lhs {
            type Output = IntegerOrFloat;
            fn $fn(self, rhs: Self::Output) -> Self::Output {
                match rhs {
                    IntegerOrFloat::Integer(i) => {
                        IntegerOrFloat::$when_int(self as $coerce_i $op i as $coerce_i)
                    },
                    IntegerOrFloat::Float(f) => {
                        IntegerOrFloat::Float(self as f_iof $op f)
                    }
                }
            }
        }

    }
}

macro_rules! impl_std_ops_iof_primitive_all {
    ($op:tt, $trait:ident, $fn:ident, ($($types:ident),+), ($coerce_i:tt, $when_int:ident)) => {
        $(
            impl_std_ops_iof_primitive!($op, $trait, $fn, $types, ($coerce_i, $when_int));
        )+
    }
}

macro_rules! impl_std_ops_iof_primitives_all {
    ($op:tt, $trait:ident, $fn:ident) => {
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (i8, i16, i32, i64, isize), (i_iof, Integer));
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (u8, u16, u32, u64, usize), (i_iof, Integer));
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (f32, f64), (f_iof, Float));
    }
}

macro_rules! impl_std_ops_integer_iof_all {
    ($op:tt, $trait:ident, $fn:ident, ($($types:ident),+)) => {
        $(
            impl_std_ops_primitive_iof!($op, $trait, $fn, $types, (i_iof, Integer));
        )+
    }
}

macro_rules! impl_std_ops_primitives_iof_all {
    ($op:tt, $trait:ident, $fn:ident) => {
        impl_std_ops_integer_iof_all!($op, $trait, $fn, (i8, i16, i32, i64, isize));
        impl_std_ops_integer_iof_all!($op, $trait, $fn, (u8, u16, u32, u64, usize));
        impl_std_ops_primitive_iof!($op, $trait, $fn, f32, (f_iof, Float));
        impl_std_ops_primitive_iof!($op, $trait, $fn, f64, (f_iof, Float));
    }
}

macro_rules! impl_from_iof_for_primitive {
    ($primitive:ident) => {

        impl From<IntegerOrFloat> for $primitive {
            fn from(iof: IntegerOrFloat) -> Self {
                match iof {
                    IntegerOrFloat::Float(f) => f as Self,
                    IntegerOrFloat::Integer(i) => i as Self
                }
            }
        }

    }
}

macro_rules! impl_from_iof_for_primitives_all {
    ($($primitive:ident),+) => {
        $(
            impl_from_iof_for_primitive!($primitive);
        )+
    }
}

macro_rules! impl_from_integer_for_iof {
    ($integer_type:ident) => {

        impl From<$integer_type> for IntegerOrFloat {
            fn from(p: $integer_type) -> Self {
                IntegerOrFloat::Integer(i_iof::try_from(p).unwrap())
            }
        }

    }
}

macro_rules! impl_from_integer_for_iof_all {
    ($($primitive:ident),+) => {
        $(
            impl_from_integer_for_iof!($primitive);
        )+
    }
}

macro_rules! impl_from_float_for_iof {
    ($integer_type:ident) => {

        impl From<$integer_type> for IntegerOrFloat {
            fn from(p: $integer_type) -> Self {
                IntegerOrFloat::Float(p as f_iof)
            }
        }

    }
}

macro_rules! impl_from_float_for_iof_all {
    ($($primitive:ident),+) => {
        $(
            impl_from_float_for_iof!($primitive);
        )+
    }
}

impl_from_iof_for_primitives_all!(i8, i16, i32, i64, isize);
impl_from_iof_for_primitives_all!(u8, u16, u32, u64, usize);
impl_from_iof_for_primitives_all!(f32, f64);

impl_from_integer_for_iof_all!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
impl_from_float_for_iof_all!(f32, f64);

use core::ops::{Mul, Div, Add, Sub, Rem};
// IntegerOrFloat * IntegerOrFloat, etc.
impl_std_ops_iof_iof!(*, Mul, mul);
impl_std_ops_iof_iof!(/, Div, div);
impl_std_ops_iof_iof!(+, Add, add);
impl_std_ops_iof_iof!(-, Sub, sub);
impl_std_ops_iof_iof!(%, Rem, rem);

// IntegerOrFloat * 2, IntegerOrFloat * 2.5, etc.
impl_std_ops_iof_primitives_all!(*, Mul, mul);
impl_std_ops_iof_primitives_all!(/, Div, div);
impl_std_ops_iof_primitives_all!(+, Add, add);
impl_std_ops_iof_primitives_all!(-, Sub, sub);
impl_std_ops_iof_primitives_all!(%, Rem, rem);

// 2 * IntegerOrFloat, 2.5 * IntegerOrFloat, etc.
impl_std_ops_primitives_iof_all!(*, Mul, mul);
impl_std_ops_primitives_iof_all!(/, Div, div);
impl_std_ops_primitives_iof_all!(+, Add, add);
impl_std_ops_primitives_iof_all!(-, Sub, sub);
impl_std_ops_primitives_iof_all!(%, Rem, rem);

use core::ops::Neg;
impl Neg for IntegerOrFloat {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1
    }
}
