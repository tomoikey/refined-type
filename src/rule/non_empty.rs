mod non_empty_string;
mod non_empty_vec;

use crate::rule::composer::Not;
use crate::rule::EmptyRule;
use crate::Refined;

pub use non_empty_string::*;
pub use non_empty_vec::*;

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
