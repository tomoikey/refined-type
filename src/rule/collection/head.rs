use crate::rule::{IndexRule, Rule};
use crate::Refined;
use std::collections::VecDeque;

/// A type that holds a value satisfying the `HeadRule`
pub type Head<RULE, ITERABLE> = Refined<HeadRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `HeadRule`
pub type HeadVec<RULE> = Refined<HeadVecRule<RULE>>;

/// A type that holds a VecDeque value satisfying the `HeadRule`
pub type HeadVecDeque<RULE> = Refined<HeadVecDequeRule<RULE>>;

/// A type that holds a String value satisfying the `HeadRule`
pub type HeadString<RULE> = Refined<HeadStringRule<RULE>>;

/// Rule where the first element satisfies the condition
pub type HeadRule<RULE, ITERABLE> = IndexRule<0, RULE, ITERABLE>;

/// Rule where the first element in the `Vec` satisfies the condition
pub type HeadVecRule<RULE> = HeadRule<RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where the first element in the `VecDeque` satisfies the condition
pub type HeadVecDequeRule<RULE> = HeadRule<RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where the first element in the `String` satisfies the condition
pub type HeadStringRule<RULE> = HeadRule<RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{HeadVec, NonEmptyStringRule};

    #[test]
    fn head_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            vec!["good morning".to_string(), "".to_string()],
            vec!["hello".to_string(), "hello".to_string()],
        ];

        for value in table {
            let head = HeadVec::<NonEmptyStringRule>::new(value.clone())?;
            assert_eq!(head.into_value(), value);
        }

        Ok(())
    }

    #[test]
    fn head_invalid() -> anyhow::Result<()> {
        let table = vec![
            vec![],
            vec!["".to_string()],
            vec!["".to_string(), "hello".to_string()],
        ];

        for value in table {
            let head_result = HeadVec::<NonEmptyStringRule>::new(value.clone());
            assert!(head_result.is_err());
        }

        Ok(())
    }
}
