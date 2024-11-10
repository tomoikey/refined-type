use crate::result::Error;
pub use collection::*;
pub use empty::*;
pub use length::*;
pub use non_empty::*;
pub use number::*;
pub use string::*;

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
    fn validate(target: Self::Item) -> crate::Result<Self::Item>;
}

/// This is a `Rule` that always returns `Ok`
pub struct Valid<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Rule for Valid<T> {
    type Item = T;
    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        Ok(target)
    }
}

/// This is a `Rule` that always returns `Err`
pub struct Invalid<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Rule for Invalid<T> {
    type Item = T;
    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        Err(Error::new(target, "Invalid"))
    }
}
