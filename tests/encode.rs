#![cfg(all(feature = "num-traits", not(feature = "no_std")))]

use integer_or_float::FromEncoded as _;
use num_traits::float::FloatCore as _;

#[test]
fn test_enc_inequality() {
    assert!(f64::from_encoded(f64::NAN.integer_decode()) != f64::NAN);
    assert!(f64::from_encoded(f64::INFINITY.integer_decode()) == f64::INFINITY);
}
