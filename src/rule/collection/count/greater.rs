use crate::result::Error;
use crate::rule::{Iterable, Rule};
use crate::Refined;
use std::collections::VecDeque;

/// A type that holds a value where the count of items in the collection that satisfy the condition is greater than `N`.
pub type CountGreater<const N: usize, RULE, ITERABLE> =
    Refined<CountGreaterRule<N, RULE, ITERABLE>>;

/// A type that holds a `Vec` value where the count of items that satisfy the condition is greater than `N`.
pub type CountGreaterVec<const N: usize, RULE> = CountGreater<N, RULE, Vec<<RULE as Rule>::Item>>;

/// A type that holds a `VecDeque` value where the count of items that satisfy the condition is greater than `N`.
pub type CountGreaterVecDeque<const N: usize, RULE> =
    CountGreater<N, RULE, VecDeque<<RULE as Rule>::Item>>;

/// A type that holds a `HashMap` value where the count of items that satisfy the condition is greater than `N`.
pub type CountGreaterHashMap<const N: usize, RULE, K> =
    CountGreater<N, RULE, std::collections::HashMap<K, <RULE as Rule>::Item>>;

/// A type that holds a `HashSet` value where the count of items that satisfy the condition is greater than `N`.
pub type CountGreaterHashSet<const N: usize, RULE, K> =
    CountGreater<N, RULE, std::collections::HashSet<K>>;

/// A type that holds a `String` value where the count of items that satisfy the condition is greater than `N`.
pub type CountGreaterString<const N: usize, RULE> = CountGreater<N, RULE, String>;

/// A type that holds a `&'a str` value where the count of items that satisfy the condition is greater than `N`.
pub type CountGreaterStr<'a, const N: usize, RULE> = CountGreater<N, RULE, &'a str>;

/// Rule where the count of items in the collection that satisfy the condition is greater than `N`.
pub struct CountGreaterRule<const N: usize, RULE: Rule, ITERABLE: Iterable>
where
    ITERABLE: Iterable<Item = RULE::Item>,
{
    _phantom: std::marker::PhantomData<(RULE, ITERABLE)>,
}

impl<const N: usize, ITERABLE, RULE> Rule for CountGreaterRule<N, RULE, ITERABLE>
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
        if count > N {
            Ok(target)
        } else {
            Err(Error::new(
                target,
                format!("count is not greater than {}, actual count is {}", N, count),
            ))
        }
    }
}

/// Rule where the count of items in the `Vec` that satisfy the condition is greater than `N`.
pub type CountGreaterVecRule<const N: usize, RULE> =
    CountGreaterRule<N, RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where the count of items in the `VecDeque` that satisfy the condition is greater than `N`.
pub type CountGreaterVecDequeRule<const N: usize, RULE> =
    CountGreaterRule<N, RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where the count of items in the `HashMap` that satisfy the condition is greater than `N`.
pub type CountGreaterHashMapRule<const N: usize, RULE, K> =
    CountGreaterRule<N, RULE, std::collections::HashMap<K, <RULE as Rule>::Item>>;

/// Rule where the count of items in the `HashSet` that satisfy the condition is greater than `N`.
pub type CountGreaterHashSetRule<const N: usize, RULE, K> =
    CountGreaterRule<N, RULE, std::collections::HashSet<K>>;

/// Rule where the count of items in the `String` that satisfy the condition is greater than `N`.
pub type CountGreaterStringRule<const N: usize, RULE> = CountGreaterRule<N, RULE, String>;

/// Rule where the count of items in the `&'a str` that satisfy the condition is greater than `N`.
pub type CountGreaterStrRule<'a, const N: usize, RULE> = CountGreaterRule<N, RULE, &'a str>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{CountGreater, NonEmptyStringRule};

    #[test]
    fn count_greater_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let count_greater: CountGreater<1, NonEmptyStringRule, Vec<_>> =
            CountGreater::new(value.clone())?;
        assert_eq!(count_greater.into_value(), value);
        Ok(())
    }

    #[test]
    fn count_greater_2() -> anyhow::Result<()> {
        let value = vec!["".to_string(), "".to_string()];
        let count_greater_result =
            CountGreater::<1, NonEmptyStringRule, Vec<_>>::new(value.clone());
        assert!(count_greater_result.is_err());
        Ok(())
    }
}
