use crate::rule::{Rule, SkipFirst, SkipRule};
use crate::Refined;
use std::collections::VecDeque;

/// A type that holds a value satisfying the `TailRule`
pub type Tail<RULE, ITERABLE, ITEM> = Refined<TailRule<RULE, ITERABLE, ITEM>>;

/// A type that holds a `Vec` value satisfying the `TailRule`
pub type TailVec<RULE> = Tail<RULE, Vec<<RULE as Rule>::Item>, <RULE as Rule>::Item>;

/// A type that holds a `VecDeque` value satisfying the `TailRule`
pub type TailVecDeque<RULE> = Tail<RULE, VecDeque<<RULE as Rule>::Item>, <RULE as Rule>::Item>;

/// A type that holds a `String` value satisfying the `TailRule`
pub type TailString<RULE> = Tail<RULE, String, char>;

/// Rule where the data in the collection satisfies the condition after skipping the first element
pub type TailRule<RULE, ITERABLE, ITEM> = SkipRule<RULE, ITERABLE, SkipFirst<ITEM>>;

/// Rule where the data in the `Vec` satisfies the condition after skipping the first element
pub type TailVecRule<RULE> = TailRule<RULE, Vec<<RULE as Rule>::Item>, <RULE as Rule>::Item>;

/// Rule where the data in the `VecDeque` satisfies the condition after skipping the first element
pub type TailVecDequeRule<RULE> =
    TailRule<RULE, VecDeque<<RULE as Rule>::Item>, <RULE as Rule>::Item>;

/// Rule where the data in the `String` satisfies the condition after skipping the first element
pub type TailStringRule<RULE> = TailRule<RULE, String, char>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{NonEmptyStringRule, TailVec};

    #[test]
    fn test_tail_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            (
                vec!["hey".to_string(), "hello".to_string(), "world".to_string()],
                vec!["hey".to_string(), "hello".to_string(), "world".to_string()],
            ),
            (
                vec!["".to_string(), "hello".to_string(), "world".to_string()],
                vec!["".to_string(), "hello".to_string(), "world".to_string()],
            ),
            (vec!["".to_string()], vec!["".to_string()]),
            (vec![], vec![]),
        ];

        for (input, expected) in table {
            let refined = TailVec::<NonEmptyStringRule>::new(input.clone())?;
            assert_eq!(refined.into_value(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_tail_invalid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            vec!["hey".to_string(), "hello".to_string(), "".to_string()],
            vec!["hey".to_string(), "".to_string(), "".to_string()],
            vec!["".to_string(), "hello".to_string(), "".to_string()],
            vec!["".to_string(), "".to_string(), "".to_string()],
            vec!["hey".to_string(), "hello".to_string(), "".to_string()],
            vec!["hey".to_string(), "".to_string(), "".to_string()],
            vec!["".to_string(), "hello".to_string(), "".to_string()],
            vec!["".to_string(), "".to_string(), "".to_string()],
        ];

        for input in table {
            let refined = TailVec::<NonEmptyStringRule>::new(input.clone());
            assert!(refined.is_err());
        }

        Ok(())
    }
}
