use crate::result::Error;
use crate::rule::Rule;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

/// Refined is a versatile type in ensuring that `T` satisfies the conditions of `RULE` (predicate type)
/// # Example
/// ```rust
/// # use std::ops::Deref;
/// use refined_type::rule::{NonEmptyString, NonEmptyStringRule};
/// use refined_type::Refined;
///
/// let non_empty_string_result = Refined::<NonEmptyStringRule>::new("Hello World".to_string());
/// assert_eq!(non_empty_string_result.unwrap().deref(), "Hello World");
///
/// let empty_string_result = Refined::<NonEmptyStringRule>::new("".to_string());
/// assert!(empty_string_result.is_err())
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Refined<RULE>
where
    RULE: Rule,
{
    value: RULE::Item,
    _rule: PhantomData<RULE>,
}

impl<RULE, T> Refined<RULE>
where
    RULE: Rule<Item = T>,
{
    pub fn new(value: T) -> Result<Self, Error<T>> {
        Ok(Self {
            value: RULE::validate(value)?,
            _rule: Default::default(),
        })
    }
}

impl<RULE, T> Deref for Refined<RULE>
where
    RULE: Rule<Item = T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<RULE, T> Display for Refined<RULE>
where
    RULE: Rule<Item = T>,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::refined::Refined;
    use crate::result::Error;
    use crate::rule::NonEmptyStringRule;

    #[test]
    fn test_refined_non_empty_string_ok() -> Result<(), Error<String>> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string())?;
        assert_eq!(non_empty_string.value, "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_non_empty_string_err() -> Result<(), String> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("".to_string());
        assert!(non_empty_string.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_display() -> Result<(), Error<String>> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string())?;
        assert_eq!(format!("{}", non_empty_string), "Hello");
        Ok(())
    }
}
