pub mod composer;
mod empty;
mod non_empty;
mod number;
mod string;
mod for_all;

use crate::result::Error;
pub use empty::*;
pub use non_empty::*;
pub use number::*;
pub use string::*;
pub use for_all::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(target: &Self::Item) -> Result<(), Error>;
}
