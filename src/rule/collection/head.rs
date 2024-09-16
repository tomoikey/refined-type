mod collection;
mod string;

use crate::rule::Rule;
use crate::Refined;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// A type that holds a value satisfying the `HeadRule`
pub type Head<RULE, ITERABLE> = Refined<HeadRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `HeadRule`
pub type HeadVec<RULE> = Head<RULE, Vec<<RULE as Rule>::Item>>;

/// A type that holds a VecDeque value satisfying the `HeadRule`
pub type HeadVecDeque<RULE> = Head<RULE, VecDeque<<RULE as Rule>::Item>>;

/// A type that holds a String value satisfying the `HeadRule`
pub type HeadString<RULE> = Head<RULE, String>;

/// Rule where the first element satisfies the condition
pub struct HeadRule<RULE, ITERABLE>
where
    RULE: Rule,
{
    _phantom_data: PhantomData<(RULE, ITERABLE)>,
}

#[cfg(test)]
mod tests {
    use crate::rule::{HeadVec, NonEmptyStringRule};

    #[test]
    fn head_valid() -> anyhow::Result<()> {
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
