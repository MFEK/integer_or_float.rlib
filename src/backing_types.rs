//! Allow user to have an i64/f64-holding backing store. Useful for kurbo etc.
//!
//! TODO: Fix testing infrastructure to support this build flag.

#![allow(non_camel_case_types)]

#[cfg(feature = "x64-backing-store")]
pub type f_iof = f64;
#[cfg(feature = "x64-backing-store")]
pub type i_iof = i64;
#[cfg(feature = "x64-backing-store")]
pub type u_iof = u64;
#[cfg(not(feature = "x64-backing-store"))]
pub type f_iof = f32;
#[cfg(not(feature = "x64-backing-store"))]
pub type i_iof = i32;
#[cfg(not(feature = "x64-backing-store"))]
pub type u_iof = u32;

#[cfg(feature = "x64-backing-store")]
pub const IOF_BACKING_STORE_BITLEN: u8 = 64;
#[cfg(not(feature = "x64-backing-store"))]
pub const IOF_BACKING_STORE_BITLEN: u8 = 32;
