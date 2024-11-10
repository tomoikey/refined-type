use crate::rule::Rule;
use crate::Refined;
use std::collections::VecDeque;

pub type Index<const INDEX: usize, RULE, ITERABLE> = Refined<IndexRule<INDEX, RULE, ITERABLE>>;
pub type IndexVec<const INDEX: usize, RULE> = Refined<IndexRuleVec<INDEX, RULE>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexRule<const INDEX: usize, RULE, ITERABLE>
where
    RULE: Rule,
{
    _phantom_data: std::marker::PhantomData<(RULE, ITERABLE)>,
}

pub type IndexRuleVec<const INDEX: usize, RULE> = IndexRule<INDEX, RULE, Vec<<RULE as Rule>::Item>>;

impl<const INDEX: usize, RULE, ITEM> Rule for IndexRuleVec<INDEX, RULE>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = Vec<ITEM>;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        if INDEX >= target.len() {
            return Err(crate::result::Error::new(
                target,
                format!("index {} is out of bounds", INDEX),
            ));
        }
        let mut target = target;
        match RULE::validate(target.remove(INDEX)) {
            Ok(validated_item) => {
                target.insert(INDEX, validated_item);
                Ok(target)
            }
            Err(err) => {
                target.insert(INDEX, err.into_value());
                Err(crate::result::Error::new(
                    target,
                    format!("the item at index {} does not satisfy the condition", INDEX),
                ))
            }
        }
    }
}

pub type IndexRuleVecDeque<const INDEX: usize, RULE> =
    IndexRule<INDEX, RULE, VecDeque<<RULE as Rule>::Item>>;

impl<const INDEX: usize, RULE, ITEM> Rule for IndexRuleVecDeque<INDEX, RULE>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = VecDeque<ITEM>;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        if INDEX >= target.len() {
            return Err(crate::result::Error::new(
                target,
                format!("index {} is out of bounds", INDEX),
            ));
        }
        let mut target = target;
        match RULE::validate(
            target
                .remove(INDEX)
                .expect("This error is always unreachable"),
        ) {
            Ok(validated_item) => {
                target.insert(INDEX, validated_item);
                Ok(target)
            }
            Err(err) => {
                target.insert(INDEX, err.into_value());
                Err(crate::result::Error::new(
                    target,
                    format!("the item at index {} does not satisfy the condition", INDEX),
                ))
            }
        }
    }
}

pub type IndexRuleString<const INDEX: usize, RULE> = IndexRule<INDEX, RULE, String>;

impl<const INDEX: usize, RULE> Rule for IndexRuleString<INDEX, RULE>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        if INDEX >= target.len() {
            return Err(crate::result::Error::new(
                target,
                format!("index {} is out of bounds", INDEX),
            ));
        }
        let mut target = target;
        match RULE::validate(target.remove(INDEX)) {
            Ok(validated_item) => {
                target.insert(INDEX, validated_item);
                Ok(target)
            }
            Err(err) => {
                target.insert(INDEX, err.into_value());
                Err(crate::result::Error::new(
                    target,
                    format!("the item at index {} does not satisfy the condition", INDEX),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::{IndexVec, NonEmptyStringRule};

    #[test]
    fn test_index_0_non_empty_string() -> anyhow::Result<()> {
        let table = vec![
            (vec!["good morning".to_string(), "hello".to_string()], true),
            (vec!["good morning".to_string(), "".to_string()], true),
            (vec!["".to_string(), "hello".to_string()], false),
            (vec!["".to_string(), "".to_string()], false),
        ];

        for (value, expected) in table {
            let refined = IndexVec::<0, NonEmptyStringRule>::new(value.clone());
            assert_eq!(refined.is_ok(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_index_1_non_empty_string() -> anyhow::Result<()> {
        let table = vec![
            (vec!["good morning".to_string(), "hello".to_string()], true),
            (vec!["good morning".to_string(), "".to_string()], false),
            (vec!["".to_string(), "hello".to_string()], true),
            (vec!["".to_string(), "".to_string()], false),
        ];

        for (value, expected) in table {
            let refined = IndexVec::<1, NonEmptyStringRule>::new(value.clone());
            assert_eq!(refined.is_ok(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_index_2_non_empty_string_out_of_bounds() {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let refined = IndexVec::<2, NonEmptyStringRule>::new(value);
        assert!(refined.is_err());
    }
}
