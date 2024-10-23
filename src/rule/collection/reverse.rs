use crate::result::Error;
use crate::rule::{Iterable, Rule};
use crate::Refined;
use std::marker::PhantomData;

/// A type that holds a value satisfying the `ReverseRule`
pub type Reverse<'a, RULE> = Refined<ReverseRule<'a, RULE>>;

/// Rule where the data in the collection satisfies the condition after reversing
pub struct ReverseRule<'a, RULE>
where
    RULE: Rule,
{
    _phantom_data: PhantomData<&'a RULE>,
}

impl<'a, RULE, ITERABLE> Rule for ReverseRule<'a, RULE>
where
    RULE: Rule<Item = ITERABLE>,
    ITERABLE: Iterable<'a> + FromIterator<ITERABLE::Item>,
{
    type Item = RULE::Item;

    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        match RULE::validate(ITERABLE::from_iter(target.into_iterator().rev())) {
            Ok(iterable) => Ok(ITERABLE::from_iter(iterable.into_iterator().rev())),
            Err(e) => {
                let message = format!("ReverseRule validation failed: {}", e);
                let item = ITERABLE::from_iter(e.into_value().into_iterator().rev());
                Err(Error::new(item, message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{IndexRuleVec, NonEmptyStringRule, Reverse};

    #[test]
    fn test_reverse_string_valid() -> Result<(), Error<String>> {
        let table = vec!["hello".to_string(), "world".to_string()];

        for input in table {
            let refined = Reverse::<NonEmptyStringRule>::new(input.clone())?;
            assert_eq!(refined.into_value(), input);
        }

        Ok(())
    }

    #[test]
    fn test_reverse_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![
            vec!["hey".to_string(), "hello".to_string()],
            vec!["hello".to_string()],
        ];

        for input in table {
            let refined = Reverse::<IndexRuleVec<0, NonEmptyStringRule>>::new(input.clone())?;
            assert_eq!(refined.into_value(), input);
        }

        Ok(())
    }

    #[test]
    fn test_reverse_invalid() {
        let table = vec![vec!["".to_string()], vec![]];

        for input in table {
            let refined = Reverse::<IndexRuleVec<0, NonEmptyStringRule>>::new(input.clone());
            assert!(refined.is_err());
        }
    }
}
