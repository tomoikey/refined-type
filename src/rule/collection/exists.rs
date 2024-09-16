use crate::rule::composer::Not;
use crate::rule::{ForAllRule, Rule};
use crate::Refined;
use std::collections::{HashMap, HashSet, VecDeque};

/// A type that holds a value satisfying the `ExistsRule`
pub type Exists<RULE, ITERABLE> = Refined<ExistsRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `ExistsRule`
pub type ExistsVec<RULE> = Exists<RULE, Vec<<RULE as Rule>::Item>>;

/// A type that holds a VecDeque value satisfying the `ExistsRule`
pub type ExistsVecDeque<RULE> = Exists<RULE, VecDeque<<RULE as Rule>::Item>>;

/// A type that holds a HashSet value satisfying the `ExistsRule`
pub type ExistsHashSet<RULE> = Exists<RULE, HashSet<<RULE as Rule>::Item>>;

/// A type that holds a HashMap value satisfying the `ExistsRule`
pub type ExistsHashMap<K, RULE> = Exists<RULE, HashMap<K, <RULE as Rule>::Item>>;

/// A type that holds a String value satisfying the `ExistsRule`
pub type ExistsString<RULE> = Exists<RULE, String>;

/// Rule where at least one data in the collection satisfies the condition
pub type ExistsRule<RULE, ITERABLE> = Not<ForAllRule<Not<RULE>, ITERABLE>>;

#[cfg(test)]
mod tests {
    use crate::rule::{Exists, NonEmptyStringRule};

    #[test]
    fn exists_1() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let exists: Exists<NonEmptyStringRule, Vec<_>> = Exists::new(value.clone())?;
        assert_eq!(exists.into_value(), value);
        Ok(())
    }

    #[test]
    fn exists_2() -> anyhow::Result<()> {
        let value = vec!["".to_string(), "".to_string()];
        let exists_result = Exists::<NonEmptyStringRule, Vec<_>>::new(value.clone());
        assert!(exists_result.is_err());
        Ok(())
    }
}
