mod collection;
mod string;

use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;

/// A type that holds a value satisfying the `ForAllRule`
pub type ForAll<RULE, ITERABLE> = Refined<ForAllRule<RULE, ITERABLE>>;

pub trait Iterable<'a> {
    type Item: 'a;
    fn into_iterator(self) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}

/// Rule where all the data in the collection satisfies the condition
pub struct ForAllRule<RULE, ITERABLE>
where
    RULE: Rule,
{
    _phantom_data: PhantomData<(RULE, ITERABLE)>,
}

impl<'a, RULE, ITERABLE, ITEM> Rule for ForAllRule<RULE, ITERABLE>
where
    RULE: Rule<Item = ITEM>,
    ITERABLE: Iterable<'a, Item = ITEM> + FromIterator<ITEM>,
{
    type Item = ITERABLE;

    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        let mut remains = target.into_iterator();
        let mut result = VecDeque::new();
        let mut failed = false;

        for item in remains.by_ref() {
            match RULE::validate(item) {
                Ok(validated_item) => result.push_back(validated_item),
                Err(err) => {
                    result.push_back(err.into_value());
                    failed = true;
                    break;
                }
            }
        }

        if failed {
            result.append(&mut remains.collect::<VecDeque<_>>());
            let result = result.into_iter().collect::<Vec<_>>();
            Err(Error::new(
                ITERABLE::from_iter(result),
                "Failed to validate all items",
            ))
        } else {
            let result = result.into_iter().collect::<Vec<_>>();
            Ok(ITERABLE::from_iter(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::for_all::ForAll;
    use crate::rule::{NonEmptyStringRule, Rule};

    #[test]
    fn for_all_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let for_all: ForAll<NonEmptyStringRule, _> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }

    #[test]
    fn for_all_2() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "".to_string()];
        let for_all_result = ForAll::<NonEmptyStringRule, Vec<_>>::new(value.clone());
        assert!(for_all_result.is_err());
        Ok(())
    }

    #[test]
    fn for_all_3() -> Result<(), Error<String>> {
        struct CharRule;
        impl Rule for CharRule {
            type Item = char;

            fn validate(target: Self::Item) -> Result<char, Error<char>> {
                if target.is_alphabetic() {
                    Ok(target)
                } else {
                    let message = format!("{} is not an alphabet", target);
                    Err(Error::new(target, message))
                }
            }
        }

        let value = "hello".to_string();
        let for_all: ForAll<CharRule, String> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }
}
