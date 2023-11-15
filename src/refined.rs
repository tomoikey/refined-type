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
/// let non_empty_string_result = Refined::new("Hello World".to_string(), &NonEmptyStringRule::default());
/// assert_eq!(non_empty_string_result.unwrap().deref(), "Hello World");
///
/// let empty_string_result = Refined::new("".to_string(), &NonEmptyStringRule::default());
/// assert!(empty_string_result.is_err())
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Refined<RULE, T> {
    value: T,
    _rule: PhantomData<RULE>,
}

impl<RULE, T> Refined<RULE, T>
where
    RULE: Rule<Item = T>,
{
    pub fn new(value: T, rule: &RULE) -> Result<Self, Error<T>> {
        Ok(Self {
            value: RULE::validate(rule, value)?,
            _rule: Default::default(),
        })
    }
}

impl<RULE, ITERATOR, T> Refined<RULE, ITERATOR>
where
    RULE: Rule<Item = T>,
    ITERATOR: IntoIterator<Item = T> + FromIterator<T>,
{
    pub fn from_iter(value: ITERATOR, rule: &RULE) -> Result<Self, Error<T>> {
        let mut result = Vec::new();
        for i in value.into_iter() {
            result.push(RULE::validate(rule, i)?)
        }
        Ok(Self {
            value: result.into_iter().collect(),
            _rule: Default::default(),
        })
    }
}

impl<RULE, T> Deref for Refined<RULE, T>
where
    RULE: Rule<Item = T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<RULE, T> Display for Refined<RULE, T>
where
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
    use std::collections::HashSet;

    #[test]
    fn test_refined_non_empty_string_ok() -> Result<(), Error<String>> {
        let non_empty_string = Refined::new("Hello".to_string(), &NonEmptyStringRule::default())?;
        assert_eq!(non_empty_string.value, "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_non_empty_string_err() -> Result<(), String> {
        let non_empty_string = Refined::new("".to_string(), &NonEmptyStringRule::default());
        assert!(non_empty_string.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_array_of_non_empty_string_ok() -> Result<(), Error<String>> {
        let strings = vec![
            "Good Morning".to_string(),
            "Hello".to_string(),
            "Good Evening".to_string(),
        ];
        let array_non_empty_string =
            Refined::from_iter(strings.clone(), &NonEmptyStringRule::default())?;
        assert_eq!(array_non_empty_string.value, strings);
        Ok(())
    }

    #[test]
    fn test_refined_hash_set_of_non_empty_string_ok() -> Result<(), Error<String>> {
        let mut set = HashSet::new();
        vec![
            "Good Morning".to_string(),
            "Hello".to_string(),
            "Good Evening".to_string(),
        ]
        .into_iter()
        .for_each(|n| {
            set.insert(n);
        });

        let array_non_empty_string =
            Refined::from_iter(set.clone(), &NonEmptyStringRule::default())?;
        assert_eq!(array_non_empty_string.value, set);
        Ok(())
    }

    #[test]
    fn test_refined_array_of_non_empty_string_err() -> Result<(), String> {
        let strings = vec![
            "Good Morning".to_string(),
            "".to_string(),
            "Good Evening".to_string(),
        ];
        let array_non_empty_string_result =
            Refined::from_iter(strings.clone(), &NonEmptyStringRule::default());
        assert!(array_non_empty_string_result.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_display() -> Result<(), Error<String>> {
        let non_empty_string = Refined::new("Hello".to_string(), &NonEmptyStringRule::default())?;
        assert_eq!(format!("{}", non_empty_string), "Hello");
        Ok(())
    }
}
