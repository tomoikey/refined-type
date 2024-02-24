mod empty_definition;
mod iterator;
mod number;
mod string;

use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Add;

pub use empty_definition::EmptyDefinition;

/// The `Empty` type is a type that indicates that its subject is empty.  
/// The definition of empty is defined by `EmptyDefinition`.  
///
/// # Example
/// ```rust
/// # use refined_type::rule::Empty;
/// let empty_1 = Empty::new(0).unwrap();
/// let empty_2 = Empty::new(0).unwrap();
/// let empty = empty_1 + empty_2;
///
/// assert_eq!(empty.into_value(), 0);
/// ```
pub type Empty<T> = Refined<EmptyRule<T>>;

impl<T> Add for Empty<T>
where
    T: EmptyDefinition,
{
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        self
    }
}

/// Rule where the data is empty
/// ```rust
/// use refined_type::rule::{EmptyRule, Rule};
///
/// assert!(EmptyRule::<String>::validate("".to_string()).is_ok());
/// assert!(EmptyRule::<String>::validate("non empty".to_string()).is_err());
///
/// assert!(EmptyRule::<Vec<u8>>::validate(Vec::<u8>::new()).is_ok());
/// assert!(EmptyRule::<Vec<u8>>::validate(vec![1, 2, 3]).is_err());
///
/// assert!(EmptyRule::<u8>::validate(0).is_ok());
/// assert!(EmptyRule::<u8>::validate(1).is_err());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct EmptyRule<T> {
    _phantom_data: PhantomData<T>,
}

impl<T> Rule for EmptyRule<T>
where
    T: EmptyDefinition,
{
    type Item = T;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.empty() {
            Ok(target)
        } else {
            Err(Error::new("The input value is not empty", target))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::Empty;

    #[test]
    fn test_add_empty() -> anyhow::Result<()> {
        let empty_1 = Empty::new(0)?;
        let empty_2 = Empty::new(0)?;
        let empty = empty_1 + empty_2;
        assert_eq!(empty.into_value(), 0);
        Ok(())
    }
}
