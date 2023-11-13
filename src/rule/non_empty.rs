mod non_empty_string;
mod non_empty_vec;

use crate::rule::composer::Not;
use crate::rule::Empty;
pub use non_empty_string::*;
pub use non_empty_vec::*;

/// Rule where the data is non-empty
/// ```rust
/// use refined_type::rule::{NonEmpty, Rule};
/// assert!(NonEmpty::default().validate("non empty".to_string()).is_ok());
/// assert!(NonEmpty::default().validate("".to_string()).is_err());
///
/// assert!(NonEmpty::default().validate(vec![1, 2, 3]).is_ok());
/// assert!(NonEmpty::default().validate(Vec::<u8>::new()).is_err());
///
/// assert!(NonEmpty::default().validate(1).is_ok());
/// assert!(NonEmpty::default().validate(0).is_err());
/// ```
pub type NonEmpty<'a, T> = Not<'a, T, Empty<T>>;
