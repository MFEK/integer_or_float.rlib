use super::IntegerOrFloat;
#[allow(unused_imports)]
use super::{i_iof, f_iof};

impl Default for IntegerOrFloat {
    fn default() -> Self {
        IntegerOrFloat::Float(f_iof::default())
    }
}
