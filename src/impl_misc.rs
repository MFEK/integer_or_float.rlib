use super::IntegerOrFloat;
#[allow(unused_imports)]
use super::{f_iof, i_iof};

impl Default for IntegerOrFloat {
    fn default() -> Self {
        IntegerOrFloat::Float(f_iof::default())
    }
}
