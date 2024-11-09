use std::collections::{HashMap, HashSet, VecDeque};

use crate::rule::composer::Not;
use crate::rule::{ForAllRule, Iterable, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `ExistsRule`
pub type Exists<RULE, ITERABLE> = Refined<ExistsRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `ExistsRule`
pub type ExistsVec<RULE> = Refined<ExistsVecRule<RULE>>;

/// A type that holds a VecDeque value satisfying the `ExistsRule`
pub type ExistsVecDeque<RULE> = Refined<ExistsVecDequeRule<RULE>>;

/// A type that holds a HashSet value satisfying the `ExistsRule`
pub type ExistsHashSet<RULE> = Refined<ExistsHashSetRule<RULE>>;

/// A type that holds a HashMap value satisfying the `ExistsRule`
pub type ExistsHashMap<K, RULE> = Refined<ExistsHashMapRule<K, RULE>>;

/// A type that holds a String value satisfying the `ExistsRule`
pub type ExistsString<RULE> = Refined<ExistsStringRule<RULE>>;

/// Rule where at least one data in the collection satisfies the condition
pub type ExistsRule<RULE, ITERABLE> =
    Not<ForAllRule<Not<RULE>, ITERABLE, <ITERABLE as Iterable>::Item>>;

/// Rule where at least one data in the `Vec` satisfies the condition
pub type ExistsVecRule<RULE> = ExistsRule<RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where at least one data in the `VecDeque` satisfies the condition
pub type ExistsVecDequeRule<RULE> = ExistsRule<RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where at least one data in the `HashSet` satisfies the condition
pub type ExistsHashSetRule<RULE> = ExistsRule<RULE, HashSet<<RULE as Rule>::Item>>;

/// Rule where at least one data in the `HashMap` satisfies the condition
pub type ExistsHashMapRule<K, RULE> = ExistsRule<RULE, HashMap<K, <RULE as Rule>::Item>>;

/// Rule where at least one data in the `String` satisfies the condition
pub type ExistsStringRule<RULE> = ExistsRule<RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{Exists, NonEmptyStringRule};

    #[test]
    fn exists_1() -> Result<(), Error<Vec<String>>> {
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
