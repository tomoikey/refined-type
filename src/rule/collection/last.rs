use std::collections::VecDeque;

use crate::rule::{IndexRule, ReverseRule, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LastRule`
pub type Last<RULE, ITERABLE> = Refined<LastRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `LastRule`
pub type LastVec<RULE> = Refined<LastVecRule<RULE>>;

/// A type that holds a VecDeque value satisfying the `LastRule`
pub type LastVecDeque<RULE> = Refined<LastVecDequeRule<RULE>>;

/// A type that holds a String value satisfying the `LastRule`
pub type LastString<RULE> = Refined<LastStringRule<RULE>>;

/// Rule where the last element satisfies the condition
pub type LastRule<RULE, ITERABLE> = ReverseRule<IndexRule<0, RULE, ITERABLE>>;

/// Rule where the last element in the `Vec` satisfies the condition
pub type LastVecRule<RULE> = LastRule<RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where the last element in the `VecDeque` satisfies the condition
pub type LastVecDequeRule<RULE> = LastRule<RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where the last element in the `String` satisfies the condition
pub type LastStringRule<RULE> = LastRule<RULE, String>;

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
