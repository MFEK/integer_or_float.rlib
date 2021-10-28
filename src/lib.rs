//! This is a Rust implementation of the UFO data type "integer or float".
#[cfg(feature = "default")]
use serde::{Serialize, Deserialize};

/// The UFO data type "integer or float".
#[cfg_attr(feature = "default", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IntegerOrFloat {
    Integer(i32),
    Float(f32)
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
                        IntegerOrFloat::Float(f $op i2 as f32)
                    },
                    (IntegerOrFloat::Integer(i), IntegerOrFloat::Float(f2)) => {
                        IntegerOrFloat::Float(i as f32 $op f2)
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
                        IntegerOrFloat::Float(f as f32 $op rhs as f32)
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
                        IntegerOrFloat::Float(self as f32 $op f)
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
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (i8, i16, i32, i64, isize), (i32, Integer));
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (u8, u16, u32, u64, usize), (i32, Integer));
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (f32, f64), (f32, Float));
    }
}

macro_rules! impl_std_ops_integer_iof_all {
    ($op:tt, $trait:ident, $fn:ident, ($($types:ident),+)) => {
        $(
            impl_std_ops_primitive_iof!($op, $trait, $fn, $types, (i32, Integer));
        )+
    }
}

macro_rules! impl_std_ops_primitives_iof_all {
    ($op:tt, $trait:ident, $fn:ident) => {
        impl_std_ops_integer_iof_all!($op, $trait, $fn, (i8, i16, i32, i64, isize));
        impl_std_ops_integer_iof_all!($op, $trait, $fn, (u8, u16, u32, u64, usize));
        impl_std_ops_primitive_iof!($op, $trait, $fn, f32, (f32, Float));
        impl_std_ops_primitive_iof!($op, $trait, $fn, f64, (f32, Float));
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
                IntegerOrFloat::Integer(p as i32)
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
                IntegerOrFloat::Float(p as f32)
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

use std::ops::{Mul, Div, Add, Sub, Rem};
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

use std::ops::Neg;
impl Neg for IntegerOrFloat {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl From<IntegerOrFloat> for String {
    fn from(iof: IntegerOrFloat) -> Self {
        match iof {
            IntegerOrFloat::Float(f) => f.to_string(),
            IntegerOrFloat::Integer(i) => i.to_string()
        }
    }
}

impl ToString for IntegerOrFloat {
    fn to_string(&self) -> String {
        String::from(*self)
    }
}

use std::error::Error;
use std::fmt;
#[derive(Debug, PartialEq)]
pub enum ConversionError {
    IntegerConversionError,
    FloatConversionError
}
use ConversionError::*;
impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegerConversionError => write!(f, "String not an integer"),
            FloatConversionError => write!(f, "String not a floating point number")
        }
    }
}
impl Error for ConversionError {}

use std::convert::TryFrom;
impl TryFrom<&str> for IntegerOrFloat {
    type Error = ConversionError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !s.contains('.') {
            if let Ok(i) = s.parse::<i32>() {
                Ok(IntegerOrFloat::Integer(i))
            } else {
                Err(IntegerConversionError)
            }
        } else {
            if let Ok(f) = s.parse::<f32>() {
                Ok(IntegerOrFloat::Float(f))
            } else {
                Err(FloatConversionError)
            }
        }
    }
}

mod tests {
    #[test]
    fn test_impls() {
        use crate::IntegerOrFloat::{self, *};

        assert_eq!(Integer(3) * Float(-1.0), Float(-3.0));
        assert_eq!(Integer(3) * Integer(3), Integer(9));
        assert_eq!(Integer(5) % Integer(2), Integer(1));
        assert_eq!(Float(9.0) - Float(7.0), Float(2.0));
        assert_eq!(-Float(4.0), Float(-4.0));
        assert_eq!(Integer(5) * 5.0, Float(25.0));
        assert_eq!(Float(5.0) * 5, Float(25.0));
        assert_eq!(Integer(5) * 5, Integer(25));
        let x: usize = Integer(5).into();
        assert_eq!(x, 5);
        let x: isize = Integer(-5).into();
        assert_eq!(x, -5);
        let x: f64 = Integer(5).into();
        assert_eq!(x, 5.0);

        let x: IntegerOrFloat = (4.5).into();
        assert_eq!(x, Float(4.5));

        let x: IntegerOrFloat = (10).into();
        assert_eq!(x, Integer(10));
    }
}
