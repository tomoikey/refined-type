use crate::rule::{Iterable, ReverseRule, Rule, SkipFirst, SkipRule};
use crate::Refined;
use std::collections::VecDeque;

/// A type that holds a value satisfying the `InitRule`
pub type Init<'a, RULE, ITERABLE> = Refined<InitRule<'a, RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `InitRule`
pub type InitVec<'a, RULE> = Refined<InitVecRule<'a, RULE>>;

/// A type that holds a VecDeque value satisfying the `InitRule`
pub type InitVecDeque<'a, RULE> = Refined<InitVecDequeRule<'a, RULE>>;

/// A type that holds a String value satisfying the `InitRule`
pub type InitString<'a, RULE> = Refined<InitStringRule<'a, RULE>>;

/// Rule that applies to the initialization of a collection
pub type InitRule<'a, RULE, ITERABLE> =
    ReverseRule<'a, SkipRule<RULE, ITERABLE, SkipFirst<<ITERABLE as Iterable<'a>>::Item>>>;

/// Rule that applies to the initialization of a `Vec`
pub type InitVecRule<'a, RULE> = InitRule<'a, RULE, Vec<<RULE as Rule>::Item>>;

/// Rule that applies to the initialization of a `VecDeque`
pub type InitVecDequeRule<'a, RULE> = InitRule<'a, RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule that applies to the initialization of a `String`
pub type InitStringRule<'a, RULE> = InitRule<'a, RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{Init, NonEmptyStringRule};

    #[test]
    fn init_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            vec![
                "hello".to_string(),
                "hello".to_string(),
                "hello".to_string(),
            ],
            vec!["hello".to_string(), "hello".to_string(), "".to_string()],
        ];

        for value in table {
            let init = Init::<NonEmptyStringRule, _>::new(value.clone())?;
            assert_eq!(init.into_value(), value);
        }

        Ok(())
    }
}
