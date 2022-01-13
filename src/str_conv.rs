use core::fmt;
use core::str::FromStr;

use super::IntegerOrFloat::{self, *};

#[cfg(std)]
impl From<IntegerOrFloat> for String {
    fn from(iof: IntegerOrFloat) -> Self {
        match iof {
            IntegerOrFloat::Float(f) => f.to_string(),
            IntegerOrFloat::Integer(i) => i.to_string()
        }
    }
}

// this implements ToString iff feature(alloc), which has:
// impl<T> ToString for T where T: std::fmt::Display, T: ?Sized { … };
impl fmt::Display for IntegerOrFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IntegerOrFloat::Float(f) => f.fmt(formatter),
            IntegerOrFloat::Integer(i) => i.fmt(formatter)
        }
    }
}

impl fmt::Debug for IntegerOrFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IntegerOrFloat::Float(f) => {
                formatter.write_fmt(format_args!("Float({})", f))
            }
            IntegerOrFloat::Integer(i) => {
                formatter.write_fmt(format_args!("Integer({})", i))
            }
        }
    }
}

#[cfg(std)]
use std::error::Error;
#[cfg(no_std)]
trait Error {}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
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

use core::convert::TryFrom;
impl TryFrom<&str> for IntegerOrFloat {
    type Error = ConversionError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = s.parse::<i32>() {
            Ok(Integer(i))
        } else {
            if let Ok(f) = s.parse::<f32>() {
                Ok(Float(f))
            } else {
                if !s.contains('.') {
                    Err(IntegerConversionError)
                } else {
                    Err(FloatConversionError)
                }
            }
        }
    }
}

#[cfg(feature = "num-traits")]
impl From<num_traits::ParseFloatError> for ConversionError {
    fn from(_: num_traits::ParseFloatError) -> Self {
        ConversionError::FloatConversionError
    }
}

impl From<core::num::ParseIntError> for ConversionError {
    fn from(_: core::num::ParseIntError) -> Self {
        ConversionError::IntegerConversionError
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Num for IntegerOrFloat {
    type FromStrRadixErr = ConversionError;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if s.contains('.') {
            Ok(Integer(i32::from_str_radix(s, radix)?))
        } else {
            Ok(Float(f32::from_str_radix(s, radix)?))
        }
    }
}

impl FromStr for IntegerOrFloat {
    type Err = ConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
