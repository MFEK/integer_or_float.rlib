use core::hash::{Hash, Hasher};
use num_traits::Float;

use super::IntegerOrFloat::{self, *};

impl Hash for IntegerOrFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Integer(i) => i.hash(state),
            Float(f) => {
                let mest = f.integer_decode();
                mest.hash(state)
            }
        }
    }
}
