#![cfg(feature = "hash")]

use integer_or_float::IntegerOrFloat;
use IntegerOrFloat::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher as _, Hash as _};

fn test_hash_impl((iof1, iof2): (IntegerOrFloat, IntegerOrFloat), eq: bool) {
    let mut hasher = DefaultHasher::new();
    iof1.hash(&mut hasher);
    let mut hasher2 = DefaultHasher::new();
    iof2.hash(&mut hasher2);

    assert!(
        if eq {
            hasher.finish() == hasher2.finish()
        } else {
            hasher.finish() != hasher2.finish()
        }
    )
}

fn test_hash_eq(iof: (IntegerOrFloat, IntegerOrFloat)) {
    test_hash_impl(iof, true)
}

fn test_hash_ne(iof: (IntegerOrFloat, IntegerOrFloat)) {
    test_hash_impl(iof, false)
}

#[test]
fn test_hashes() {
    test_hash_eq((Integer(1), Integer(1)));
    test_hash_ne((Integer(0), Integer(1)));
    test_hash_eq((Float(1.0), Float(1.0)));
    test_hash_ne((Float(1.0), Float(1.1)));

    test_hash_eq((Float(1.0) + f64::EPSILON, Float(1.0)));
    test_hash_ne((Float(1.0) + f32::EPSILON, Float(1.0)));
}

#[should_panic]
#[test]
fn test_hash_nan() {
    //eprintln!("{:?}", f32::from_encoded(f32::NAN.integer_decode()));
    test_hash_eq((Float(f32::NAN), Float(f32::NAN)));
}

#[test]
fn test_hash_inf() {
    //eprintln!("{:?}", f32::from_encoded(f32::INFINITY.integer_decode()));
    test_hash_eq((Float(f32::INFINITY), Float(f32::INFINITY)));
}
