mod non_empty_map;
mod non_empty_set;
mod non_empty_string;
mod non_empty_vec;
mod non_empty_vec_deque;

use crate::rule::composer::Not;
use crate::rule::{EmptyDefinition, EmptyRule};
use crate::Refined;
use std::iter::Map;

pub use non_empty_map::*;
pub use non_empty_set::*;
pub use non_empty_string::*;
pub use non_empty_vec::*;
pub use non_empty_vec_deque::*;

pub type NonEmpty<T> = Refined<NonEmptyRule<T>>;

/// Rule where the data is non-empty
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

impl<I: ExactSizeIterator + EmptyDefinition> NonEmpty<I> {
    pub fn map<B, F>(self, f: F) -> Refined<NonEmptyRule<Map<I, F>>>
    where
        Self: Sized,
        F: FnMut(I::Item) -> B,
    {
        let map_into_iter = self.into_value().map(f);
        Refined::new(map_into_iter)
            .ok()
            .expect("This error is always unreachable")
    }

    pub fn collect<B: FromIterator<I::Item> + EmptyDefinition>(self) -> NonEmpty<B>
    where
        Self: Sized,
    {
        Refined::new(FromIterator::from_iter(self.into_value()))
            .ok()
            .expect("This error is always unreachable")
    }
}
