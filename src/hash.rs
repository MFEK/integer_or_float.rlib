use core::hash::{Hash, Hasher};

use super::IntegerOrFloat::{self, *};

impl Hash for IntegerOrFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Integer(i) => i.hash(state),
            Float(f) => {
                if f.is_nan() {
                    panic!("Cannot hash a NaN")
                }
                let u64_f = f.to_bits();
                u64_f.hash(state)
            }
        }
    }
}
