pub mod composer;
mod empty;
mod non_empty_vec;
mod number;
mod string;

use crate::result::Error;
pub use non_empty_vec::*;
pub use number::*;
pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(&self, target: Self::Item) -> Result<Self::Item, Error<Self::Item>>;
}
