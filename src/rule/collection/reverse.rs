mod collection;
mod string;

use crate::Refined;
use std::marker::PhantomData;

/// A type that holds a value satisfying the `ReverseRule`
pub type Reverse<RULE, ITERABLE> = Refined<ReverseRule<RULE, ITERABLE>>;

/// Rule where the data in the collection satisfies the condition after reversing
pub struct ReverseRule<RULE, ITERABLE> {
    _phantom_data: PhantomData<(RULE, ITERABLE)>,
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{Index0VecRule, NonEmptyStringRule, Reverse};

    #[test]
    fn test_reverse_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            vec!["hey".to_string(), "hello".to_string()],
            vec!["hello".to_string()],
        ];

        for input in table {
            let refined = Reverse::<Index0VecRule<NonEmptyStringRule>, _>::new(input.clone())?;
            assert_eq!(refined.into_value(), input);
        }

        Ok(())
    }

    #[test]
    fn test_reverse_invalid() {
        let table = vec![vec!["".to_string()], vec![]];

        for input in table {
            let refined = Reverse::<Index0VecRule<NonEmptyStringRule>, _>::new(input.clone());
            assert!(refined.is_err());
        }
    }
}
