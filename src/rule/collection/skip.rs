mod option;

use std::collections::VecDeque;
use std::marker::PhantomData;

pub use option::*;

use crate::rule::{Iterable, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `SkipRule`
pub type Skip<RULE, ITERABLE, OPTION> = Refined<SkipRule<RULE, ITERABLE, OPTION>>;

/// A type that holds a `Vec` value satisfying the `SkipRule`
pub type SkipVec<RULE, OPTION> = Refined<SkipVecRule<RULE, OPTION>>;

/// A type that holds a `VecDeque` value satisfying the `SkipRule`
pub type SkipVecDeque<RULE, OPTION> = Refined<SkipVecDequeRule<RULE, OPTION>>;

/// A type that holds a `String` value satisfying the `SkipRule`
pub type SkipString<RULE, OPTION> = Refined<SkipStringRule<RULE, OPTION>>;

/// Rule where the data in the collection satisfies the condition after skipping the first element
pub struct SkipRule<RULE, ITERABLE, OPTION>
where
    RULE: Rule,
    OPTION: SkipOption,
{
    _phantom_data: PhantomData<(RULE, ITERABLE, OPTION)>,
}

impl<'a, RULE, ITERABLE, OPTION> Rule for SkipRule<RULE, ITERABLE, OPTION>
where
    RULE: Rule,
    ITERABLE: Iterable<'a, Item = RULE::Item> + FromIterator<RULE::Item>,
    OPTION: SkipOption<Item = RULE::Item>,
{
    type Item = ITERABLE;

    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        let mut remains = target.into_iterator();
        let mut result = VecDeque::new();
        let (mut is_valid, mut message) = (true, String::new());
        let mut accumlator = None;
        for (i, item) in remains.by_ref().enumerate() {
            if OPTION::should_skip(i, accumlator.as_mut(), &item) {
                result.push_back(item);
                continue;
            }
            match RULE::validate(item) {
                Ok(validated_item) => result.push_back(validated_item),
                Err(err) => {
                    is_valid = false;
                    message = format!(
                        "the item at index {} does not satisfy the condition: {}",
                        i, err
                    );
                    result.push_back(err.into_value());
                }
            }
        }

        if is_valid {
            Ok(result.into_iter().collect())
        } else {
            result.append(&mut remains.collect::<VecDeque<_>>());
            Err(crate::result::Error::new(
                result.into_iter().collect(),
                message,
            ))
        }
    }
}

/// Rule where the data in the `Vec` satisfies the condition after skipping the first element
pub type SkipVecRule<RULE, OPTION> = SkipRule<RULE, Vec<<RULE as Rule>::Item>, OPTION>;

/// Rule where the data in the `VecDeque` satisfies the condition after skipping the first element
pub type SkipVecDequeRule<RULE, OPTION> = SkipRule<RULE, VecDeque<<RULE as Rule>::Item>, OPTION>;

/// Rule where the data in the `String` satisfies the condition after skipping the first element
pub type SkipStringRule<RULE, OPTION> = SkipRule<RULE, String, OPTION>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{NonEmptyStringRule, SkipFirst, SkipVec};

    #[test]
    fn test_skip_first_valid() -> Result<(), Error<Vec<String>>> {
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

        for (data, expected) in table {
            let value = SkipVec::<NonEmptyStringRule, SkipFirst<_>>::new(data)?;
            assert_eq!(value.into_value(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_skip_first_invalid() {
        let table = vec![
            vec!["hey".to_string(), "".to_string(), "world".to_string()],
            vec!["".to_string(), "".to_string(), "world".to_string()],
            vec!["hey".to_string(), "hello".to_string(), "".to_string()],
            vec!["".to_string(), "hello".to_string(), "".to_string()],
        ];

        for data in table {
            let value = SkipVec::<NonEmptyStringRule, SkipFirst<_>>::new(data);
            assert!(value.is_err());
        }
    }
}
