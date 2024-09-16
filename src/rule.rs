pub use empty::*;
pub use exists::*;
pub use for_all::*;
pub use length::*;
pub use non_empty::*;
pub use number::*;
pub use string::*;

pub mod composer;
mod empty;
mod exists;
mod for_all;
mod length;
mod non_empty;
mod number;
mod string;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(target: Self::Item) -> crate::Result<Self::Item>;
}
