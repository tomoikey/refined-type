mod string;

use anyhow::Result;

pub use string::non_empty_string::*;

pub trait Rule {
    type TARGET;
    fn validate(target: Self::TARGET) -> Result<Self::TARGET>;
}
