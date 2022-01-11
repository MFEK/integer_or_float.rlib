use core::hash::{Hash, Hasher};
use num_traits::float::FloatCore as _;

use super::IntegerOrFloat::{self, *};

impl Hash for IntegerOrFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Integer(i) => i.hash(state),
            Float(f) => {
                if f.is_nan() {
                    panic!("Cannot hash a NaN")
                }
                let mest = f.integer_decode();
                mest.hash(state)
            }
        }
    }
}
