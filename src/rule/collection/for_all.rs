mod collection;
mod string;

use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;

use crate::rule::Rule;
use crate::Refined;

/// A type that holds a value satisfying the `ForAllRule`
pub type ForAll<RULE, ITERABLE> = Refined<ForAllRule<RULE, ITERABLE>>;

/// A type that holds a Vec value satisfying the `ForAllRule`
pub type ForAllVec<RULE> = ForAll<RULE, Vec<<RULE as Rule>::Item>>;

/// A type that holds a VecDeque value satisfying the `ForAllRule`
pub type ForAllVecDeque<RULE> = ForAll<RULE, VecDeque<<RULE as Rule>::Item>>;

/// A type that holds a HashSet value satisfying the `ForAllRule`
pub type ForAllHashSet<RULE> = ForAll<RULE, HashSet<<RULE as Rule>::Item>>;

/// A type that holds a HashMap value satisfying the `ForAllRule`
pub type ForAllHashMap<K, RULE> = ForAll<RULE, HashMap<K, <RULE as Rule>::Item>>;

/// A type that holds a String value satisfying the `ForAllRule`
pub type ForAllString<RULE> = ForAll<RULE, String>;

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
    use crate::rule::ForAll;
    use crate::rule::{ForAllString, ForAllVec, NonEmptyStringRule, Rule};

    #[test]
    fn for_all_1() -> anyhow::Result<()> {
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
        let for_all: ForAllString<CharRule> = ForAll::new(value.clone())?;
        assert_eq!(for_all.into_value(), value);
        Ok(())
    }
}
