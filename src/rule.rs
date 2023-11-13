pub mod composer;
mod number;
mod string;

use crate::result::Result;

pub use number::*;
pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(&self, target: Self::Item) -> Result<Self::Item, Self::Item>;
}
