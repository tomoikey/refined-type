use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;
use crate::Refined;

/// A type that holds a value satisfying the `ExistsRule`
pub type Exists<RULE, ITERATOR> = Refined<ExistsRule<RULE, ITERATOR>>;

/// Rule where at least one data in the collection satisfies the condition
pub struct ExistsRule<RULE, ITERATOR> {
    _phantom_data: PhantomData<(RULE, ITERATOR)>,
}

impl<RULE, ITERATOR, ITEM> Rule for ExistsRule<RULE, ITERATOR>
where
    ITERATOR: AsRef<[ITEM]>,
    RULE: Rule<Item = ITEM>,
{
    type Item = ITERATOR;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target
            .as_ref()
            .iter()
            .any(|item| RULE::validate(item).is_ok())
        {
            Ok(())
        } else {
            Err(Error::new("no item satisfies the condition"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::{Exists, NonEmptyStringRule};

    #[test]
    fn exists_1() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let exists: Exists<NonEmptyStringRule, _> = Exists::new(value.clone())?;
        assert_eq!(exists.into_value(), value);
        Ok(())
    }

    #[test]
    fn exists_2() -> anyhow::Result<()> {
        let value = vec!["".to_string(), "".to_string()];
        let exists_result = Exists::<NonEmptyStringRule, _>::new(value.clone());
        assert!(exists_result.is_err());
        Ok(())
    }
}
