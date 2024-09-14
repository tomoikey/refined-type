mod collection;
mod string;

use std::marker::PhantomData;

use crate::rule::Rule;
use crate::Refined;

/// A type that holds a value satisfying the `ForAllRule`
pub type ForAll<RULE, ITERABLE> = Refined<ForAllRule<RULE, ITERABLE>>;

/// Rule where all the data in the collection satisfies the condition
pub struct ForAllRule<RULE, ITERABLE>
where
    RULE: Rule,
{
    _phantom_data: PhantomData<(RULE, ITERABLE)>,
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::for_all::ForAll;
    use crate::rule::{NonEmptyStringRule, Rule};

    #[test]
    fn for_all_1() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let for_all: ForAll<NonEmptyStringRule, Vec<_>> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }

    #[test]
    fn for_all_2() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "".to_string()];
        let for_all_result = ForAll::<NonEmptyStringRule, Vec<_>>::new(value.clone());
        assert!(for_all_result.is_err());
        Ok(())
    }

    #[test]
    fn for_all_3() -> anyhow::Result<()> {
        struct CharRule;
        impl Rule for CharRule {
            type Item = char;

            fn validate(target: &Self::Item) -> Result<(), Error> {
                if target.is_alphabetic() {
                    Ok(())
                } else {
                    Err(Error::new(format!("{} is not an alphabet", target)))
                }
            }
        }

        let value = "hello".to_string();
        let for_all: ForAll<CharRule, String> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }
}
