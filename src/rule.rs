pub use collection::*;
pub use empty::*;
pub use length::*;
pub use non_empty::*;
pub use number::*;
pub use string::*;

use crate::result::Error;

mod collection;
pub mod composer;
mod empty;
mod length;
mod non_empty;
mod number;
mod string;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(target: &Self::Item) -> Result<(), Error>;
}
