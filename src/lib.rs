//! This is a Rust type that holds an integer or a float.
//!
//! Before v1.4 (June 2022), it mostly focused on the UFO data type "integer or float", but is now
//! generic.
#![cfg_attr(no_std, no_std)]

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
pub use str_conv::ConversionError;

/// A generic container for an "integer or a float".
///
/// Was made originally for the UFO data type "integer or float".
#[cfg_attr(use_serde, derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone)]
pub enum IntegerOrFloat {
    Integer(i_iof),
    Float(f_iof),
}
pub use IntegerOrFloat::{Float, Integer};
