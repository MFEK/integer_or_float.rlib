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

#[cfg(std)]
impl ToString for IntegerOrFloat {
    fn to_string(&self) -> String {
        String::from(*self)
    }
}

#[cfg(no_std)]
impl fmt::Display for IntegerOrFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IntegerOrFloat::Float(f) => f.fmt(formatter),
            IntegerOrFloat::Integer(i) => i.fmt(formatter)
        }
    }
}

#[cfg(std)]
use std::error::Error;
#[cfg(no_std)]
trait Error {}

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

impl FromStr for IntegerOrFloat {
    type Err = ConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
