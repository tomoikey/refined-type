use crate::error::Result;
use crate::rule::Rule;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

/// Refined is a versatile type in ensuring that `T` satisfies the conditions of `RULE` (predicate type)
/// # Example
/// ```rust
/// # use std::ops::Deref;
/// use refined_type::{NonEmptyStringRule, Result};
/// use refined_type::{NonEmptyString, Refined};
///
/// let non_empty_string_result: Result<NonEmptyString> = Refined::new("Hello World".to_string(), &NonEmptyStringRule);
/// assert_eq!(non_empty_string_result.unwrap().deref(), "Hello World");
///
/// let empty_string_result: Result<NonEmptyString> = Refined::new("".to_string(), &NonEmptyStringRule);
/// assert!(empty_string_result.is_err())
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Refined<RULE, T> {
    value: T,
    _rule: PhantomData<RULE>,
}

impl<RULE, T> Refined<RULE, T>
where
    RULE: Rule<TARGET = T>,
{
    pub fn new(value: T, rule: &RULE) -> Result<Self> {
        Ok(Self {
            value: RULE::validate(rule, value)?,
            _rule: Default::default(),
        })
    }

    fn add_rule<NewRule>(self, rule: &NewRule) -> Result<Refined<NewRule, T>>
    where
        NewRule: Rule<TARGET = T>,
    {
        Refined::new(self.value, rule)
    }

    fn add_rules<NewRule>(self, rules: &Vec<NewRule>) -> Result<Self>
    where
        NewRule: Rule<TARGET = T>,
    {
        let mut result = self.value;
        for rule in rules {
            result = NewRule::validate(rule, result)?;
        }
        Ok(Refined {
            value: result,
            _rule: Default::default(),
        })
    }
}

impl<RULE, ITERATOR, T> Refined<RULE, ITERATOR>
where
    RULE: Rule<TARGET = T>,
    ITERATOR: IntoIterator<Item = T> + FromIterator<T>,
{
    pub fn from_iter(value: ITERATOR, rule: &RULE) -> Result<Self> {
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
    RULE: Rule<TARGET = T>,
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
    use crate::error::Result;
    use crate::refined::Refined;
    use crate::rule::NonEmptyStringRule;
    use crate::AlphabetRule;
    use std::collections::HashSet;

    #[test]
    fn test_refined_non_empty_string_ok() -> Result<()> {
        let non_empty_string = Refined::new("Hello".to_string(), &NonEmptyStringRule)?;
        assert_eq!(non_empty_string.value, "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_non_empty_string_err() -> Result<()> {
        let non_empty_string = Refined::new("".to_string(), &NonEmptyStringRule);
        assert!(non_empty_string.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_array_of_non_empty_string_ok() -> Result<()> {
        let strings = vec![
            "Good Morning".to_string(),
            "Hello".to_string(),
            "Good Evening".to_string(),
        ];
        let array_non_empty_string = Refined::from_iter(strings.clone(), &NonEmptyStringRule)?;
        assert_eq!(array_non_empty_string.value, strings);
        Ok(())
    }

    #[test]
    fn test_refined_hash_set_of_non_empty_string_ok() -> Result<()> {
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

        let array_non_empty_string = Refined::from_iter(set.clone(), &NonEmptyStringRule)?;
        assert_eq!(array_non_empty_string.value, set);
        Ok(())
    }

    #[test]
    fn test_refined_array_of_non_empty_string_err() -> Result<()> {
        let strings = vec![
            "Good Morning".to_string(),
            "".to_string(),
            "Good Evening".to_string(),
        ];
        let array_non_empty_string_result =
            Refined::from_iter(strings.clone(), &NonEmptyStringRule);
        assert!(array_non_empty_string_result.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_display() -> Result<()> {
        let non_empty_string = Refined::new("Hello".to_string(), &NonEmptyStringRule)?;
        assert_eq!(format!("{}", non_empty_string), "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_add_rule_ok() -> Result<()> {
        let non_empty_string =
            Refined::new("Hello".to_string(), &NonEmptyStringRule)?.add_rule(&AlphabetRule)?;
        assert_eq!(non_empty_string.value, "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_add_rule_err() -> Result<()> {
        let non_empty_string =
            Refined::new("Hello1".to_string(), &NonEmptyStringRule)?.add_rule(&AlphabetRule);
        assert!(non_empty_string.is_err());
        Ok(())
    }
}
