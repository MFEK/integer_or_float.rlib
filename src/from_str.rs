use core::num::ParseFloatError;
use core::str::FromStr;

use super::IntegerOrFloat::{self, *};

impl FromStr for IntegerOrFloat {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s.parse::<i32>() {
                Ok(i) => Integer(i),
                Err(_) => Float(s.parse::<f32>()?),
            }
        )
    }
}
