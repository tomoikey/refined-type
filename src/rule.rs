mod number;
mod string;

use crate::error::Result;
pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type TARGET;
    fn validate(&self, target: Self::TARGET) -> Result<Self::TARGET>;
}
