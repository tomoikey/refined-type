use crate::result::Error;
use crate::rule::{Iterable, Rule};
use crate::Refined;
use std::collections::VecDeque;

/// A type that holds a value where the count of items in the collection that satisfy the condition is less than `N`.
pub type CountLess<const N: usize, RULE, ITERABLE> = Refined<CountLessRule<N, RULE, ITERABLE>>;

/// A type that holds a `Vec` value where the count of items that satisfy the condition is less than `N`.
pub type CountLessVec<const N: usize, RULE> = CountLess<N, RULE, Vec<<RULE as Rule>::Item>>;

/// A type that holds a `VecDeque` value where the count of items that satisfy the condition is less than `N`.
pub type CountLessVecDeque<const N: usize, RULE> =
    CountLess<N, RULE, VecDeque<<RULE as Rule>::Item>>;

/// A type that holds a `HashMap` value where the count of items that satisfy the condition is less than `N`.
pub type CountLessHashMap<const N: usize, RULE, K> =
    CountLess<N, RULE, std::collections::HashMap<K, <RULE as Rule>::Item>>;

/// A type that holds a `HashSet` value where the count of items that satisfy the condition is less than `N`.
pub type CountLessHashSet<const N: usize, RULE, K> =
    CountLess<N, RULE, std::collections::HashSet<K>>;

/// A type that holds a `String` value where the count of items that satisfy the condition is less than `N`.
pub type CountLessString<const N: usize, RULE> = CountLess<N, RULE, String>;

/// A type that holds a `&'a str` value where the count of items that satisfy the condition is less than `N`.
pub type CountLessStr<'a, const N: usize, RULE> = CountLess<N, RULE, &'a str>;

/// Rule where the count of items in the collection that satisfy the condition is less than `N`.
pub struct CountLessRule<const N: usize, RULE: Rule, ITERABLE: Iterable>
where
    ITERABLE: Iterable<Item = RULE::Item>,
{
    _phantom: std::marker::PhantomData<(RULE, ITERABLE)>,
}

impl<const N: usize, ITERABLE, RULE> Rule for CountLessRule<N, RULE, ITERABLE>
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
        if count < N {
            Ok(target)
        } else {
            Err(Error::new(
                target,
                format!("count is not less than {}, actual count is {}", N, count),
            ))
        }
    }
}

/// Rule where the count of items in the `Vec` that satisfy the condition is less than `N`.
pub type CountLessVecRule<const N: usize, RULE> = CountLessRule<N, RULE, Vec<<RULE as Rule>::Item>>;

/// Rule where the count of items in the `VecDeque` that satisfy the condition is less than `N`.
pub type CountLessVecDequeRule<const N: usize, RULE> =
    CountLessRule<N, RULE, VecDeque<<RULE as Rule>::Item>>;

/// Rule where the count of items in the `HashMap` that satisfy the condition is less than `N`.
pub type CountLessHashMapRule<const N: usize, RULE, K> =
    CountLessRule<N, RULE, std::collections::HashMap<K, <RULE as Rule>::Item>>;

/// Rule where the count of items in the `HashSet` that satisfy the condition is less than `N`.
pub type CountLessHashSetRule<const N: usize, RULE, K> =
    CountLessRule<N, RULE, std::collections::HashSet<K>>;

/// Rule where the count of items in the `String` that satisfy the condition is less than `N`.
pub type CountLessStringRule<const N: usize, RULE> = CountLessRule<N, RULE, String>;

/// Rule where the count of items in the `&'a str` that satisfy the condition is less than `N`.
pub type CountLessStrRule<'a, const N: usize, RULE> = CountLessRule<N, RULE, &'a str>;

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::{CountLess, NonEmptyStringRule};

    #[test]
    fn count_less_1() -> Result<(), Error<Vec<String>>> {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let count_less: CountLess<3, NonEmptyStringRule, Vec<_>> = CountLess::new(value.clone())?;
        assert_eq!(count_less.into_value(), value);
        Ok(())
    }

    #[test]
    fn count_less_2() -> anyhow::Result<()> {
        let value = vec!["".to_string(), "hello".to_string()];
        let count_less_result = CountLess::<1, NonEmptyStringRule, Vec<_>>::new(value.clone());
        assert!(count_less_result.is_err());
        Ok(())
    }
}
