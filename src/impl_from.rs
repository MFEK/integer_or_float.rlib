//! impl_from.rs — Helpers for implementing From for primitive numbers and IntegerOrFloat
use super::IntegerOrFloat;
use super::{f_iof, i_iof};
// Setup
/// This macro is for implementing the `From` trait for all primitive types and our type. Output is whatever $primitive type requested.
///
/// Because we want to be able to use our type in place of primitive types, we
/// implement `From` for our type to primitive types.
macro_rules! impl_from_iof_for_primitive {
    ($primitive:ident) => {
        impl From<IntegerOrFloat> for $primitive {
            fn from(iof: IntegerOrFloat) -> Self {
                match iof {
                    IntegerOrFloat::Float(f) => f as Self,
                    IntegerOrFloat::Integer(i) => i as Self,
                }
            }
        }
    };
}
/// This helper macro calls  `impl_from_iof_for_primitive!` (which
/// implements `From<IOF> for $primitive`) for a list of types.
macro_rules! impl_from_iof_for_primitives_all {
    ($($primitive:ident),+) => {
        $(
            impl_from_iof_for_primitive!($primitive);
        )+
    }
}
/// This helper macro implements `From<$primitive> for IOF` (where `IOF` is
/// whatever type is passed to the macro) for integer types.
macro_rules! impl_from_integer_for_iof {
    ($integer_type:ident) => {
        impl From<$integer_type> for IntegerOrFloat {
            fn from(p: $integer_type) -> Self {
                IntegerOrFloat::Integer(i_iof::try_from(p).unwrap())
            }
        }
    };
}
/// This helper macro calls `impl_from_integer_for_iof!` (which
/// implements `From<$integer_type> for IOF`) for a list of types.
/// It's used for implementing the `From<IntegerType> for IntegerOrFloat
/// for all integer types.
macro_rules! impl_from_integer_for_iof_all {
    ($($primitive:ident),+) => {
        $(
            impl_from_integer_for_iof!($primitive);
        )+
    }
}
/// This helper macro implements `From<$primitive> for IOF` (where `IOF` is
/// whatever type is passed to the macro) for float types.
macro_rules! impl_from_float_for_iof {
    ($integer_type:ident) => {
        impl From<$integer_type> for IntegerOrFloat {
            fn from(p: $integer_type) -> Self {
                IntegerOrFloat::Float(p as f_iof)
            }
        }
    };
}
/// This helper macro calls `impl_from_float_for_iof!` (which
/// implements `From<$primitive> for IOF`) for a list of types.
/// It's used for implementing the `From<$float> for IntegerOrFloat`
/// for all float types (currently only f32 and f64).
macro_rules! impl_from_float_for_iof_all {
    ($($primitive:ident),+) => {
        $(
            impl_from_float_for_iof!($primitive);
        )+
    }
}
// Call
impl_from_iof_for_primitives_all!(i8, i16, i32, i64, isize);
impl_from_iof_for_primitives_all!(u8, u16, u32, u64, usize);
impl_from_iof_for_primitives_all!(f32, f64);
impl_from_integer_for_iof_all!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
impl_from_float_for_iof_all!(f32, f64);
