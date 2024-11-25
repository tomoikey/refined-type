use crate::result::Error;
use crate::rule::{Iterable, Rule};
use crate::Refined;
use std::collections::VecDeque;

/// A type that holds a value satisfying the `CountEqualRule`
pub type CountEqual<const N: usize, RULE, ITERABLE> = Refined<CountEqualRule<N, RULE, ITERABLE>>;

/// A type that holds a `Vec` value satisfying the `CountEqualRule`
pub type CountEqualVec<const N: usize, RULE> = Refined<CountEqualVecRule<N, RULE>>;

/// A type that holds a `VecDeque` value satisfying the `CountEqualRule`
pub type CountEqualVecDeque<const N: usize, RULE> = Refined<CountEqualVecDequeRule<N, RULE>>;

/// A type that holds a `HashMap` value satisfying the `CountEqualRule`
pub type CountEqualHashMap<const N: usize, RULE, K> = Refined<CountEqualHashMapRule<N, RULE, K>>;

/// A type that holds a `HashSet` value satisfying the `CountEqualRule`
pub type CountEqualHashSet<const N: usize, RULE, K> = Refined<CountEqualHashSetRule<N, RULE, K>>;

/// A type that holds a `String` value satisfying the `CountEqualRule`
pub type CountEqualString<const N: usize, RULE> = Refined<CountEqualStringRule<N, RULE>>;

/// A type that holds a `&'a str` value satisfying the `CountEqualRule`
pub type CountEqualStr<'a, const N: usize, RULE> = Refined<CountEqualStrRule<'a, N, RULE>>;

/// Rule where the count of items that satisfy the condition is equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountEqualRule<const N: usize, RULE: Rule, ITERABLE: Iterable>
where
    ITERABLE: Iterable<Item = RULE::Item>,
{
    _phantom: std::marker::PhantomData<(RULE, ITERABLE)>,
}

impl<const N: usize, ITERABLE, RULE> Rule for CountEqualRule<N, RULE, ITERABLE>
where
    ITERABLE: Iterable<Item = RULE::Item> + FromIterator<ITERABLE::Item>,
    RULE: Rule,
{
    type Item = ITERABLE;
    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        let mut count = 0;
        let mut deque = VecDeque::new();
        for item in target.into_iterator() {
            match RULE::validate(item) {
                Ok(item) => {
                    deque.push_back(item);
                    count += 1
                }
                Err(e) => {
                    deque.push_back(e.into_value());
                }
            }
        }
        let target = ITERABLE::from_iter(deque);
        if count == N {
            Ok(target)
        } else {
            Err(Error::new(
                target,
                format!("count is not equal to {}, actual count is {}", N, count),
            ))
        }
    }
}

/// Rule where the count of items in the `Vec` that satisfy the condition is equal to `N`.
pub type CountEqualVecRule<const N: usize, RULE> =
    CountEqualRule<N, RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where the count of items in the `VecDeque` that satisfy the condition is equal to `N`.
pub type CountEqualVecDequeRule<const N: usize, RULE> =
    CountEqualRule<N, RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where the count of items in the `HashMap` that satisfy the condition is equal to `N`.
pub type CountEqualHashMapRule<const N: usize, RULE, K> =
    CountEqualRule<N, RULE, std::collections::HashMap<K, <RULE as Rule>::Item>>;

/// Rule where the count of items in the `HashSet` that satisfy the condition is equal to `N`.
pub type CountEqualHashSetRule<const N: usize, RULE, K> =
    CountEqualRule<N, RULE, std::collections::HashSet<K>>;

/// Rule where the count of items in the `String` that satisfy the condition is equal to `N`.
pub type CountEqualStringRule<const N: usize, RULE> = CountEqualRule<N, RULE, String>;

/// Rule where the count of items in the `&'a str` that satisfy the condition is equal to `N`.
pub type CountEqualStrRule<'a, const N: usize, RULE> = CountEqualRule<N, RULE, &'a str>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{CountEqualVec, NonEmptyStringRule};

    #[test]
    fn count_equal_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let count_equal = CountEqualVec::<2, NonEmptyStringRule>::new(value.clone())?;
        assert_eq!(count_equal.into_value(), value);
        Ok(())
    }
}
