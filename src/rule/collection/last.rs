use std::collections::VecDeque;

use crate::rule::{Index0Rule, ReverseRule, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LastRule`
pub type Last<'a, RULE, ITERABLE> = Refined<LastRule<'a, RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `LastRule`
pub type LastVec<'a, RULE> = Refined<LastVecRule<'a, RULE>>;

/// A type that holds a VecDeque value satisfying the `LastRule`
pub type LastVecDeque<'a, RULE> = Refined<LastVecDequeRule<'a, RULE>>;

/// A type that holds a String value satisfying the `LastRule`
pub type LastString<'a, RULE> = Refined<LastStringRule<'a, RULE>>;

/// Rule where the last element satisfies the condition
pub type LastRule<'a, RULE, ITERABLE> = ReverseRule<'a, Index0Rule<RULE, ITERABLE>>;

/// Rule where the last element in the `Vec` satisfies the condition
pub type LastVecRule<'a, RULE> = LastRule<'a, RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where the last element in the `VecDeque` satisfies the condition
pub type LastVecDequeRule<'a, RULE> = LastRule<'a, RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where the last element in the `String` satisfies the condition
pub type LastStringRule<'a, RULE> = LastRule<'a, RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{LastVec, NonEmptyStringRule};

    #[test]
    fn last_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            vec!["".to_string(), "hello".to_string()],
            vec!["good morning".to_string(), "hello".to_string()],
        ];

        for value in table {
            let last = LastVec::<NonEmptyStringRule>::new(value.clone())?;
            assert_eq!(last.into_value(), value);
        }

        Ok(())
    }

    #[test]
    fn last_invalid() -> anyhow::Result<()> {
        let table = vec![
            vec![],
            vec!["".to_string()],
            vec!["hello".to_string(), "".to_string()],
        ];

        for value in table {
            let last_result = LastVec::<NonEmptyStringRule>::new(value.clone());
            assert!(last_result.is_err());
        }

        Ok(())
    }
}
