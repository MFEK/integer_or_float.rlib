use core::fmt;
use core::str::FromStr;

use super::IntegerOrFloat::{self, *};
use crate::ConversionError as IofConversionError;

type ConversionError = IofConversionError<core::num::ParseFloatError>;

use crate::{f_iof, i_iof};

#[macro_use]
mod ryu;
use maybe as maybe_ryu;

#[cfg(std)]
impl From<IntegerOrFloat> for String {
    fn from(iof: IntegerOrFloat) -> Self {
        match iof {
            IntegerOrFloat::Float(f) => { maybe_ryu!(f) },
            IntegerOrFloat::Integer(i) => i.to_string(),
        }
    }
}

// this implements ToString iff feature(alloc), which has:
// impl<T> ToString for T where T: std::fmt::Display, T: ?Sized { … };
impl fmt::Display for IntegerOrFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IntegerOrFloat::Float(f) => f.fmt(formatter),
            IntegerOrFloat::Integer(i) => i.fmt(formatter),
        }
    }
}

impl fmt::Debug for IntegerOrFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            IntegerOrFloat::Float(f) => formatter.write_fmt(format_args!("Float({})", f)),
            IntegerOrFloat::Integer(i) => formatter.write_fmt(format_args!("Integer({})", i)),
        }
    }
}

use core::convert::TryFrom;
impl TryFrom<&str> for IntegerOrFloat {
    type Error = ConversionError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.parse::<i_iof>() {
            Ok(i) => Ok(Integer(i)),
            Err(ice) => {
                match s.parse::<f_iof>() {
                    Ok(f) => Ok(Float(f)),
                    Err(fce) => Err(ConversionError::kind_for_string(s, ice, fce))
                }
            }
        }
    }
}

#[cfg(feature = "num-traits")]
impl num_traits::Num for IntegerOrFloat {
    type FromStrRadixErr = ConversionError;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if s.contains('.') {
            Ok(Float(f_iof::from_str_radix(s, radix)?))
        } else {
            Ok(Integer(i_iof::from_str_radix(s, radix)?))
        }
    }
}

impl FromStr for IntegerOrFloat {
    type Err = ConversionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
