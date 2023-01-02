use core::fmt;
use core::num::ParseIntError;

#[cfg(std)]
use std::error::Error;
#[cfg(no_std)]
trait Error {}

/// Conversion from string, specifically to float, failed.
///
/// NB: Strings returned are identical to num-traits and [`core::num::ParseFloatError`].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum IofParseFloatError {
    /// cannot parse float from empty string
    Empty,
    #[default]
    /// invalid float literal
    Invalid,
}

impl fmt::Display for IofParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "cannot parse float from empty string"),
            Self::Invalid => write!(f, "invalid float literal")
        }
    }
}

macro_rules! float_conversion_define_enum {
    ($($ty:tt)*) => (
        #[derive(Debug)]
        #[non_exhaustive]
        /// Conversion from string of either [`crate::IntegerOrFloat`] variant failed.
        pub enum ConversionError<T: $($ty)*> where T: $($ty)* {
            /// Bad string conversion requested
            IntegerConversionError(ParseIntError),
            /// `T` depends on your build flags. Can be [`core::num::ParseFloatError`] or the
            /// idential trait in num-traits crate.
            FloatConversionError(T),
        }

        float_conversion_blanket_impl!($($ty)*);
        float_conversion_display_error!($($ty)*);
        #[cfg(feature = "num-traits")]
        impl_from_pfe!(num_traits::ParseFloatError);
        impl_from_pfe!(core::num::ParseFloatError);
        float_conversion_kind_for_string!($($ty)*);
        #[cfg(not(feature = "num-traits"))]
        float_conversion_error_conversion!(( $($ty)* ), core::num::ParseFloatError);
        #[cfg(feature = "num-traits")]
        float_conversion_error_conversion!(( $($ty)* ), num_traits::ParseFloatError);
    )
}

macro_rules! float_conversion_blanket_impl {
    ($($ty:tt)*) => {
        impl<T: $($ty)*> ConversionError<T> where T: $($ty)* {}
    }
}

macro_rules! float_conversion_display_error {
    ($($ty:tt)*) => (
        use ConversionError::*;
        impl<T> fmt::Display for ConversionError<T> where T: $($ty)* {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    IntegerConversionError(e) => write!(f, "String not an integer: {:?}", e),
                    FloatConversionError(e) => write!(f, "String not a floating point number: {:?}", e),
                }
            }
        }

        impl Error for IofParseFloatError {}
        impl<T> Error for ConversionError<T> where T: $($ty)* {}
    )
}

macro_rules! impl_from_pfe {
    ($($ty:tt)*) => {
        impl From<$($ty)*> for IofParseFloatError {
            fn from(e: $($ty)*) -> Self {
                if e.to_string() == "".parse::<f32>().unwrap_err().to_string() {
                    Self::Empty
                } else {
                    Self::Invalid
                }
            }
        }
    }
}

macro_rules! float_conversion_kind_for_string {
    ($($ty:tt)*) => (
        impl<T> ConversionError<T> where T: $($ty)* {
            pub(crate) fn kind_for_string(s: impl AsRef<str>, ice: ParseIntError, fce: T) -> Self {
                if s.as_ref().contains('.') {
                    ConversionError::FloatConversionError(T::from(fce.into()))
                } else {
                    ConversionError::IntegerConversionError(ice)
                }
            }
        }
    )
}
macro_rules! float_conversion_error_conversion {
    (($($ty:tt)*), $pf:path) => (
        impl<T> From<$pf> for ConversionError<T> where T: $($ty)* {
            fn from(e: $pf) -> Self {
                e.into()
            }
        }
    )
}



float_conversion_define_enum!(fmt::Debug + Into<core::num::ParseFloatError> + From<core::num::ParseFloatError>);
impl<T> From<core::num::ParseIntError> for ConversionError<T> where T: fmt::Debug + Into<core::num::ParseFloatError> + From<core::num::ParseFloatError> {
    fn from(e: ParseIntError) -> Self {
        ConversionError::IntegerConversionError(e)
    }
}

#[test]
fn test_err_strconv() {
    let invalid = "a".parse::<f32>().unwrap_err();
    let empty = "".parse::<f32>().unwrap_err();
    assert_eq!(IofParseFloatError::Invalid, invalid.into());
    assert_eq!(IofParseFloatError::Empty, empty.into());
}

#[cfg(feature = "num-traits")]
#[test]
fn test_err_strconv_num_traits() {
    use num_traits::ParseFloatError;
    use num_traits::FloatErrorKind;
    macro_rules! invalid {
        () => (ParseFloatError { kind: FloatErrorKind::Invalid });
    }
    macro_rules! empty {
        () => (ParseFloatError { kind: FloatErrorKind::Empty });
    }
    assert_eq!(IofParseFloatError::Invalid, invalid!().into());
    assert_eq!(IofParseFloatError::Empty, empty!().into());

}
