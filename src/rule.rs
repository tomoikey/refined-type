mod string;

use anyhow::Result;

pub use string::non_empty_string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type TARGET;
    fn validate(target: Self::TARGET) -> Result<Self::TARGET>;
}
