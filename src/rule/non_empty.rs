mod non_empty_string;
mod non_empty_vec;

use crate::rule::composer::Not;
use crate::rule::Empty;
pub use non_empty_string::*;
pub use non_empty_vec::*;

/// Rule where the data is non-empty
/// ```rust
/// use refined_type::rule::{NonEmpty, Rule};
/// assert!(NonEmpty::<String>::validate("non empty".to_string()).is_ok());
/// assert!(NonEmpty::<String>::validate("".to_string()).is_err());
///
/// assert!(NonEmpty::<Vec<u8>>::validate(vec![1, 2, 3]).is_ok());
/// assert!(NonEmpty::<Vec<u8>>::validate(Vec::new()).is_err());
///
/// assert!(NonEmpty::<u8>::validate(1).is_ok());
/// assert!(NonEmpty::<u8>::validate(0).is_err());
/// ```
pub type NonEmpty<T> = Not<Empty<T>>;
