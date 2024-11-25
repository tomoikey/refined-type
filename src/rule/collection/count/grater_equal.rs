use crate::rule::{CountEqualRule, CountGreaterRule};
use crate::{Or, Refined};

/// A type that holds a value satisfying the `GreaterEqualRule`
pub type CountGreaterEqual<const N: usize, RULE, ITERABLE> =
    Refined<CountGreaterEqualRule<N, RULE, ITERABLE>>;

/// A type that holds a `Vec` value satisfying the `CountGreaterEqualRule`
pub type CountGreaterEqualVec<const N: usize, RULE> = Refined<CountGreaterEqualVecRule<N, RULE>>;

/// A type that holds a `VecDeque` value satisfying the `CountGreaterEqualRule`
pub type CountGreaterEqualVecDeque<const N: usize, RULE> =
    Refined<CountGreaterEqualVecDequeRule<N, RULE>>;

/// A type that holds a `HashMap` value satisfying the `CountGreaterEqualRule`
pub type CountGreaterEqualHashMap<const N: usize, RULE, K> =
    Refined<CountGreaterEqualHashMapRule<N, RULE, K>>;

/// A type that holds a `HashSet` value satisfying the `CountGreaterEqualRule`
pub type CountGreaterEqualHashSet<const N: usize, RULE, K> =
    Refined<CountGreaterEqualHashSetRule<N, RULE, K>>;

/// A type that holds a `String` value satisfying the `CountGreaterEqualRule`
pub type CountGreaterEqualString<const N: usize, RULE> =
    Refined<CountGreaterEqualStringRule<N, RULE>>;

/// A type that holds a `&'a str` value satisfying the `CountGreaterEqualRule`
pub type CountGreaterEqualStr<'a, const N: usize, RULE> =
    Refined<CountGreaterEqualStrRule<'a, N, RULE>>;

/// Rule where the count of items in the collection that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualRule<const N: usize, RULE, ITERABLE> =
    Or![CountGreaterRule<N, RULE, ITERABLE>, CountEqualRule<N, RULE, ITERABLE>];

/// Rule where the count of items in the `Vec` that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualVecRule<const N: usize, RULE> =
    CountGreaterEqualRule<N, RULE, Vec<<RULE as crate::rule::Rule>::Item>>;

/// Rule where the count of items in the `VecDeque` that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualVecDequeRule<const N: usize, RULE> =
    CountGreaterEqualRule<N, RULE, std::collections::VecDeque<<RULE as crate::rule::Rule>::Item>>;

/// Rule where the count of items in the `HashMap` that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualHashMapRule<const N: usize, RULE, K> =
    CountGreaterEqualRule<N, RULE, std::collections::HashMap<K, <RULE as crate::rule::Rule>::Item>>;

/// Rule where the count of items in the `HashSet` that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualHashSetRule<const N: usize, RULE, K> =
    CountGreaterEqualRule<N, RULE, std::collections::HashSet<K>>;

/// Rule where the count of items in the `String` that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualStringRule<const N: usize, RULE> = CountGreaterEqualRule<N, RULE, String>;

/// Rule where the count of items in the `&'a str` that satisfy the condition is greater than or equal to `N`.
pub type CountGreaterEqualStrRule<'a, const N: usize, RULE> =
    CountGreaterEqualRule<N, RULE, &'a str>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{CountGreaterEqualVec, NonEmptyStringRule};

    #[test]
    fn count_greater_equal_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let count_greater_equal: CountGreaterEqualVec<2, NonEmptyStringRule> =
            CountGreaterEqualVec::new(value.clone())?;
        assert_eq!(count_greater_equal.into_value(), value);
        Ok(())
    }

    #[test]
    fn count_greater_equal_2() -> anyhow::Result<()> {
        let value = vec!["".to_string(), "".to_string()];
        let count_greater_equal_result =
            CountGreaterEqualVec::<2, NonEmptyStringRule>::new(value.clone());
        assert!(count_greater_equal_result.is_err());
        Ok(())
    }
}
