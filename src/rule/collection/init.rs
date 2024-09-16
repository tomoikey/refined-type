use crate::rule::Rule;
use crate::Refined;
use std::collections::VecDeque;
use std::marker::PhantomData;

mod collection;
mod string;

/// A type that holds a value satisfying the `InitRule`
pub type Init<RULE, ITERABLE> = Refined<InitRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `InitRule`
pub type InitVec<RULE> = Refined<InitVecRule<RULE>>;

/// A type that holds a VecDeque value satisfying the `InitRule`
pub type InitVecDeque<RULE> = Refined<InitVecDequeRule<RULE>>;

/// A type that holds a String value satisfying the `InitRule`
pub type InitString<RULE> = Refined<InitStringRule<RULE>>;

/// Rule that applies to the initialization of a collection
pub struct InitRule<RULE, ITERABLE> {
    _phantom_data: PhantomData<(RULE, ITERABLE)>,
}

/// Rule that applies to the initialization of a `Vec`
pub type InitVecRule<RULE> = InitRule<RULE, Vec<<RULE as Rule>::Item>>;

/// Rule that applies to the initialization of a `VecDeque`
pub type InitVecDequeRule<RULE> = InitRule<RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule that applies to the initialization of a `String`
pub type InitStringRule<RULE> = InitRule<RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{InitVec, NonEmptyStringRule};

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
            let init = InitVec::<NonEmptyStringRule>::new(value.clone())?;
            assert_eq!(init.into_value(), value);
        }

        Ok(())
    }
}
