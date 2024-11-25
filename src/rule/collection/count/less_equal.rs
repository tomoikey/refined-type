use crate::rule::{CountEqualRule, CountLessRule};
use crate::{Or, Refined};

/// A type that holds a value satisfying the `LessEqualRule`
pub type CountLessEqual<const N: usize, RULE, ITERABLE> =
    Refined<CountLessEqualRule<N, RULE, ITERABLE>>;

/// A type that holds a `Vec` value satisfying the `CountLessEqualRule`
pub type CountLessEqualVec<const N: usize, RULE> = Refined<CountLessEqualVecRule<N, RULE>>;

/// A type that holds a `VecDeque` value satisfying the `CountLessEqualRule`
pub type CountLessEqualVecDeque<const N: usize, RULE> =
    Refined<CountLessEqualVecDequeRule<N, RULE>>;

/// A type that holds a `HashMap` value satisfying the `CountLessEqualRule`
pub type CountLessEqualHashMap<const N: usize, RULE, K> =
    Refined<CountLessEqualHashMapRule<N, RULE, K>>;

/// A type that holds a `HashSet` value satisfying the `CountLessEqualRule`
pub type CountLessEqualHashSet<const N: usize, RULE, K> =
    Refined<CountLessEqualHashSetRule<N, RULE, K>>;

/// A type that holds a `String` value satisfying the `CountLessEqualRule`
pub type CountLessEqualString<const N: usize, RULE> = Refined<CountLessEqualStringRule<N, RULE>>;

/// A type that holds a `&'a str` value satisfying the `CountLessEqualRule`
pub type CountLessEqualStr<'a, const N: usize, RULE> = Refined<CountLessEqualStrRule<'a, N, RULE>>;

/// Rule where the count of items in the collection that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualRule<const N: usize, RULE, ITERABLE> =
    Or![CountLessRule<N, RULE, ITERABLE>, CountEqualRule<N, RULE, ITERABLE>];

/// Rule where the count of items in the `Vec` that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualVecRule<const N: usize, RULE> =
    CountLessEqualRule<N, RULE, Vec<<RULE as crate::rule::Rule>::Item>>;

/// Rule where the count of items in the `VecDeque` that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualVecDequeRule<const N: usize, RULE> =
    CountLessEqualRule<N, RULE, std::collections::VecDeque<<RULE as crate::rule::Rule>::Item>>;

/// Rule where the count of items in the `HashMap` that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualHashMapRule<const N: usize, RULE, K> =
    CountLessEqualRule<N, RULE, std::collections::HashMap<K, <RULE as crate::rule::Rule>::Item>>;

/// Rule where the count of items in the `HashSet` that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualHashSetRule<const N: usize, RULE, K> =
    CountLessEqualRule<N, RULE, std::collections::HashSet<K>>;

/// Rule where the count of items in the `String` that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualStringRule<const N: usize, RULE> = CountLessEqualRule<N, RULE, String>;

/// Rule where the count of items in the `&'a str` that satisfy the condition is less than or equal to `N`.
pub type CountLessEqualStrRule<'a, const N: usize, RULE> = CountLessEqualRule<N, RULE, &'a str>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{CountLessEqualVec, NonEmptyStringRule};

    #[test]
    fn count_less_equal_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let count_less_equal = CountLessEqualVec::<2, NonEmptyStringRule>::new(value.clone())?;
        assert_eq!(count_less_equal.into_value(), value);
        Ok(())
    }

    #[test]
    fn count_less_equal_2() -> anyhow::Result<()> {
        let value = vec!["world".to_string(), "hello".to_string()];
        let count_less_equal_result =
            CountLessEqualVec::<1, NonEmptyStringRule>::new(value.clone());
        assert!(count_less_equal_result.is_err());
        Ok(())
    }
}
