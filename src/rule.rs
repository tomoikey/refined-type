pub mod composer;
mod empty;
mod non_empty;
mod string;

use crate::result::Error;
pub use empty::*;
pub use non_empty::*;
pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>>;
}
