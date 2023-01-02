#![doc = include_str!("../README.md")]

#![cfg_attr(no_std, no_std)]
#![warn(missing_docs)]

mod impl_from;
mod impl_misc;
mod impl_ops;

pub mod backing_types;
pub use backing_types::{f_iof, i_iof, u_iof};

#[cfg(with_impl_hash)]
mod hash;

mod cmp;
mod encode;

#[cfg(feature = "num-traits")]
mod num_traits_impl;

mod str_conv;
pub mod error;
pub use error::ConversionError;

/// A generic container for an "integer or a float".
///
/// Was made originally for the UFO data type "integer or float".
#[cfg_attr(feature = "more-serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone)]
pub enum IntegerOrFloat {
    /// An integer: normally `i32`, but if compiled with `x64-backing-store` feature, can be an
    /// `f64`.
    Integer(i_iof),
    /// A float: normally `f32`, but if compiled with `x64-backing-store` feature, can be an
    /// `f64`.
    Float(f_iof),
}
pub use IntegerOrFloat::{Float, Integer};
