use crate::rule::composer::Not;
use crate::rule::{ForAllRule, Rule};
use crate::Refined;
use std::collections::{HashMap, HashSet, VecDeque};

/// A type that holds a value satisfying the `NothingRule`
pub type Nothing<RULE, ITERABLE> = Refined<NothingRule<RULE, ITERABLE>>;

/// A type that holds a `Vec` value satisfying the `NothingRule`
pub type NothingVec<RULE> = Refined<NothingVecRule<RULE>>;

/// A type that holds a `VecDeque` value satisfying the `NothingRule`
pub type NothingVecDeque<RULE> = Refined<NothingVecDequeRule<RULE>>;

/// A type that holds a `HashSet` value satisfying the `NothingRule`
pub type NothingHashSet<RULE> = Refined<NothingHashSetRule<RULE>>;

/// A type that holds a `HashMap` value satisfying the `NothingRule`
pub type NothingHashMap<K, RULE> = Refined<NothingHashMapRule<K, RULE>>;

/// A type that holds a `String` value satisfying the `NothingRule`
pub type NothingString<RULE> = Refined<NothingStringRule<RULE>>;

/// Rule where no data in the collection satisfies the condition
pub type NothingRule<RULE, ITERABLE> = ForAllRule<Not<RULE>, ITERABLE>;

/// Rule where no data in the `Vec` satisfies the condition
pub type NothingVecRule<RULE> = NothingRule<RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where no data in the `VecDeque` satisfies the condition
pub type NothingVecDequeRule<RULE> = NothingRule<RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where no data in the `HashSet` satisfies the condition
pub type NothingHashSetRule<RULE> = NothingRule<RULE, HashSet<<RULE as Rule>::Item>>;

/// Rule where no data in the `HashMap` satisfies the condition
pub type NothingHashMapRule<K, RULE> = NothingRule<RULE, HashMap<K, <RULE as Rule>::Item>>;

/// Rule where no data in the `String` satisfies the condition
pub type NothingStringRule<RULE> = NothingRule<RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{NonEmptyStringRule, NothingVec};

    #[test]
    fn nothing_valid() -> Result<(), Error<Vec<String>>> {
        let table = vec![vec![], vec!["".to_string()]];

        for value in table {
            let nothing = NothingVec::<NonEmptyStringRule>::new(value.clone())?;
            assert_eq!(nothing.into_value(), value);
        }

        Ok(())
    }

    #[test]
    fn nothing_invalid() -> anyhow::Result<()> {
        let table = vec![
            vec!["good morning".to_string(), "hello".to_string()],
            vec!["good morning".to_string()],
        ];

        for value in table {
            let nothing_result = NothingVec::<NonEmptyStringRule>::new(value.clone());
            assert!(nothing_result.is_err());
        }

        Ok(())
    }
}
