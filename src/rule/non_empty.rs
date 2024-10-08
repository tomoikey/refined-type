mod non_empty_map;
mod non_empty_set;
mod non_empty_string;
mod non_empty_vec;
mod non_empty_vec_deque;

use crate::rule::composer::Not;
use crate::rule::{EmptyDefinition, EmptyRule};
use crate::Refined;
pub use non_empty_map::*;
pub use non_empty_set::*;
pub use non_empty_string::*;
pub use non_empty_vec::*;
pub use non_empty_vec_deque::*;
use std::fmt::Debug;
use std::iter::Map;

/// A type that holds a value satisfying the `NonEmptyRule`
/// The definition of empty is defined by `EmptyDefinition`.
pub type NonEmpty<T> = Refined<NonEmptyRule<T>>;

/// Rule where the input value is not empty
/// ```rust
/// use refined_type::rule::{NonEmptyRule, Rule};
/// assert!(NonEmptyRule::<String>::validate("non empty".to_string()).is_ok());
/// assert!(NonEmptyRule::<String>::validate("".to_string()).is_err());
///
/// assert!(NonEmptyRule::<Vec<u8>>::validate(vec![1, 2, 3]).is_ok());
/// assert!(NonEmptyRule::<Vec<u8>>::validate(Vec::new()).is_err());
///
/// assert!(NonEmptyRule::<u8>::validate(1).is_ok());
/// assert!(NonEmptyRule::<u8>::validate(0).is_err());
/// ```
pub type NonEmptyRule<T> = Not<EmptyRule<T>>;

impl<I: Debug + ExactSizeIterator + EmptyDefinition> NonEmpty<I> {
    pub fn map<B, F>(self, f: F) -> Refined<NonEmptyRule<Map<I, F>>>
    where
        Self: Sized,
        F: FnMut(I::Item) -> B,
    {
        let map_into_iter = self.into_value().map(f);
        Refined::<NonEmptyRule<Map<I, F>>>::new(map_into_iter)
            .expect("This error is always unreachable")
    }

    pub fn collect<B: Debug + FromIterator<I::Item> + EmptyDefinition>(self) -> NonEmpty<B>
    where
        Self: Sized,
    {
        NonEmpty::<B>::new(FromIterator::from_iter(self.into_value()))
            .expect("This error is always unreachable")
    }
}
