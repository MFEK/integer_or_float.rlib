//! Allow user to have an i64/f64-holding backing store. Useful for kurbo etc.

#![allow(non_camel_case_types)]

/// primitive float
#[cfg(feature = "x64-backing-store")]
pub type f_iof = f64;
/// primitive signed int
#[cfg(feature = "x64-backing-store")]
pub type i_iof = i64;
/// primitive unsigned int
#[cfg(feature = "x64-backing-store")]
pub type u_iof = u64;
/// primitive float (note: `x64-backing-store` is disabled)
#[cfg(not(feature = "x64-backing-store"))]
pub type f_iof = f32;
/// primitive signed int (note: `x64-backing-store` is disabled)
#[cfg(not(feature = "x64-backing-store"))]
pub type i_iof = i32;
/// primitive unsigned int (note: `x64-backing-store` is disabled)
#[cfg(not(feature = "x64-backing-store"))]
pub type u_iof = u32;

/// Could be useful for [`crate::IntegerOrFloat::from_bits`] / [`crate::IntegerOrFloat::to_bits`].
// I'm unsure.
#[cfg(feature = "x64-backing-store")]
pub const IOF_BACKING_STORE_BITLEN: u8 = 64;
/// Could be useful for [`crate::IntegerOrFloat::from_bits`] / [`crate::IntegerOrFloat::to_bits`].
#[cfg(not(feature = "x64-backing-store"))]
pub const IOF_BACKING_STORE_BITLEN: u8 = 32;
