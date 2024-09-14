use crate::rule::composer::Not;
use crate::rule::ForAllRule;
use crate::Refined;

/// A type that holds a value satisfying the `ExistsRule`
pub type Exists<RULE, T> = Refined<ExistsRule<RULE, T>>;

/// Rule where at least one data in the collection satisfies the condition
pub type ExistsRule<RULE, T> = Not<ForAllRule<Not<RULE>, T>>;

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
