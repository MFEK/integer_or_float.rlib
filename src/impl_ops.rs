//! impl_ops.rs — Helpers for implementing std ops for primitive numbers and IntegerOrFloat

use super::IntegerOrFloat;
use super::{f_iof, i_iof};

/* Helpers for implementing std ops for primitive numbers and IntegerOrFloat */

///  This helper macro is for implementing std ops for our type and our type.
///  The `$op` argument is the operator, the `$trait` is the trait, the `$fn` is the
///  trait method.
///
///  TODO: Maybe use a trait to define the operator, so you don't
///  have to pass it in as a literal?
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

/// This helper macro is for implementing std ops for primitive numbers and our type,
/// where IOF is the Output type.
///
/// The `$op` argument is the operator, the `$trait` is the trait, the `$fn` is the
/// trait method, `($($types:ident),+)` is a list of the types that this macro
/// should be implemented for, `$coerce_i` is the type to coerce the integer to
/// when the output type is an integer, and `$when_int` is the variant of our
/// enum to use when the output type is an integer.
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

/// This helper macro is for implementing std ops for primitive numbers and our type, where the
/// primitive type is the output type.
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

/// This macro is for implementing std ops for all primitive numbers and our type. (primitive type
/// is output type)
macro_rules! impl_std_ops_iof_primitive_all {
    ($op:tt, $trait:ident, $fn:ident, ($($types:ident),+), ($coerce_i:tt, $when_int:ident)) => {
        $(
            impl_std_ops_iof_primitive!($op, $trait, $fn, $types, ($coerce_i, $when_int));
        )+
    }
}

///  This macro is for implementing std ops for all primitive numbers and our type. (our type is output type)
macro_rules! impl_std_ops_iof_primitives_all {
    ($op:tt, $trait:ident, $fn:ident) => {
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (i8, i16, i32, i64, isize), (i_iof, Integer));
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (u8, u16, u32, u64, usize), (i_iof, Integer));
        impl_std_ops_iof_primitive_all!($op, $trait, $fn, (f32, f64), (f_iof, Float));
    };
}

///  This macro is for implementing std ops for all primitive numbers and our type, where the output
///  type is our type, except for floats where the output type is just an f32 or f64.
macro_rules! impl_std_ops_integer_iof_all {
    ($op:tt, $trait:ident, $fn:ident, ($($types:ident),+)) => {
        $(
            impl_std_ops_primitive_iof!($op, $trait, $fn, $types, (i_iof, Integer));
        )+
    }
}

///  This macro is for implementing std ops for all primitive numbers and our type. (primitive type is output type)
macro_rules! impl_std_ops_primitives_iof_all {
    ($op:tt, $trait:ident, $fn:ident) => {
        impl_std_ops_integer_iof_all!($op, $trait, $fn, (i8, i16, i32, i64, isize));
        impl_std_ops_integer_iof_all!($op, $trait, $fn, (u8, u16, u32, u64, usize));
        impl_std_ops_primitive_iof!($op, $trait, $fn, f32, (f_iof, Float));
        impl_std_ops_primitive_iof!($op, $trait, $fn, f64, (f_iof, Float));
    };
}

use core::ops::{Add, Div, Mul, Rem, Sub};
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
