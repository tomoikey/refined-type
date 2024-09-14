use std::marker::PhantomData;

use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;

/// A type that holds a value satisfying the `ForAllRule`
pub type ForAll<RULE> = Refined<ForAllRule<RULE>>;

/// Rule where all the data in the collection satisfies the condition
pub struct ForAllRule<RULE> {
    _phantom_data: PhantomData<RULE>,
}

impl<RULE, ITEM> Rule for ForAllRule<RULE>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = Vec<ITEM>;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.iter().all(|item| RULE::validate(item).is_ok()) {
            Ok(())
        } else {
            Err(Error::new("not all items satisfy the condition"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::for_all::ForAll;
    use crate::rule::NonEmptyStringRule;

    #[test]
    fn for_all_1() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let for_all: ForAll<NonEmptyStringRule> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }

    #[test]
    fn for_all_2() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "".to_string()];
        let for_all_result = ForAll::<NonEmptyStringRule>::new(value.clone());
        assert!(for_all_result.is_err());
        Ok(())
    }
}
