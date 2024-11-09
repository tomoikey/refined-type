use std::collections::{HashMap, HashSet, VecDeque};

use crate::rule::{Iterable, NoSkip, Rule, SkipRule};
use crate::Refined;

/// A type that holds a value satisfying the `ForAllRule`
pub type ForAll<RULE, ITERABLE> = Refined<ForAllRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `ForAllRule`
pub type ForAllVec<RULE> = Refined<ForAllVecRule<RULE>>;

/// A type that holds a VecDeque value satisfying the `ForAllRule`
pub type ForAllVecDeque<RULE> = Refined<ForAllVecDequeRule<RULE>>;

/// A type that holds a HashSet value satisfying the `ForAllRule`
pub type ForAllHashSet<RULE> = Refined<ForAllHashSetRule<RULE>>;

/// A type that holds a HashMap value satisfying the `ForAllRule`
pub type ForAllHashMap<K, RULE> = Refined<ForAllHashMapRule<K, RULE>>;

/// A type that holds a String value satisfying the `ForAllRule`
pub type ForAllString<RULE> = Refined<ForAllStringRule<RULE>>;

/// Rule where all the data in the collection satisfies the condition
pub type ForAllRule<RULE, ITERABLE> =
    SkipRule<RULE, ITERABLE, NoSkip<<ITERABLE as Iterable>::Item>>;

/// Rule where all the data in the `Vec` satisfies the condition
pub type ForAllVecRule<RULE> = ForAllRule<RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where all the data in the `VecDeque` satisfies the condition
pub type ForAllVecDequeRule<RULE> = ForAllRule<RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where all the data in the `HashSet` satisfies the condition
pub type ForAllHashSetRule<RULE> = ForAllRule<RULE, HashSet<<RULE as Rule>::Item>>;

/// Rule where all the data in the `HashMap` satisfies the condition
pub type ForAllHashMapRule<K, RULE> = ForAllRule<RULE, HashMap<K, <RULE as Rule>::Item>>;

/// Rule where all the data in the `String` satisfies the condition
pub type ForAllStringRule<RULE> = ForAllRule<RULE, String>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::ForAll;
    use crate::rule::{ForAllString, ForAllVec, NonEmptyStringRule, Rule};

    #[test]
    fn for_all_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let for_all: ForAllVec<NonEmptyStringRule> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }

    #[test]
    fn for_all_2() -> anyhow::Result<()> {
        let value = vec!["good morning".to_string(), "".to_string()];
        let for_all_result = ForAllVec::<NonEmptyStringRule>::new(value.clone());
        assert!(for_all_result.is_err());
        Ok(())
    }

    #[test]
    fn for_all_3() -> Result<(), Error<String>> {
        struct CharRule;
        impl Rule for CharRule {
            type Item = char;

            fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
                if target.is_alphabetic() {
                    Ok(target)
                } else {
                    Err(Error::new(target, format!("{} is not an alphabet", target)))
                }
            }
        }

        let value = "hello".to_string();
        let for_all: ForAllString<CharRule> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }
}
