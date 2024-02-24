use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::iter::Map;
use std::marker::PhantomData;
use std::ops::Add;

pub type Empty<T> = Refined<EmptyRule<T>>;

impl<T> Add for Empty<T>
where
    T: EmptyDefinition,
{
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        self
    }
}

/// Rule where the data is empty
/// ```rust
/// use refined_type::rule::{EmptyRule, Rule};
///
/// assert!(EmptyRule::<String>::validate("".to_string()).is_ok());
/// assert!(EmptyRule::<String>::validate("non empty".to_string()).is_err());
///
/// assert!(EmptyRule::<Vec<u8>>::validate(Vec::<u8>::new()).is_ok());
/// assert!(EmptyRule::<Vec<u8>>::validate(vec![1, 2, 3]).is_err());
///
/// assert!(EmptyRule::<u8>::validate(0).is_ok());
/// assert!(EmptyRule::<u8>::validate(1).is_err());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct EmptyRule<T> {
    _phantom_data: PhantomData<T>,
}
pub trait EmptyDefinition {
    fn empty(&self) -> bool;
}

impl EmptyDefinition for String {
    fn empty(&self) -> bool {
        self == &"".to_string()
    }
}

impl EmptyDefinition for &str {
    fn empty(&self) -> bool {
        self == &""
    }
}

impl<T> EmptyDefinition for Vec<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> EmptyDefinition for std::vec::IntoIter<T> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> EmptyDefinition for std::slice::Iter<'a, T> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> EmptyDefinition for VecDeque<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> EmptyDefinition for std::collections::vec_deque::IntoIter<T> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> EmptyDefinition for std::collections::vec_deque::Iter<'a, T> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<F, B, I: ExactSizeIterator> EmptyDefinition for Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, S> EmptyDefinition for HashSet<T, S> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> EmptyDefinition for std::collections::hash_set::IntoIter<T> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> EmptyDefinition for std::collections::hash_set::Iter<'a, T> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V, S> EmptyDefinition for HashMap<K, V, S> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> EmptyDefinition for std::collections::hash_map::IntoIter<K, V> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, K, V> EmptyDefinition for std::collections::hash_map::Iter<'a, K, V> {
    fn empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> EmptyDefinition for BTreeSet<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> EmptyDefinition for BTreeMap<K, V> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl EmptyDefinition for u8 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for u16 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for u32 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for u64 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for u128 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for usize {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for i8 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for i16 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for i32 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for i64 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for i128 {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for isize {
    fn empty(&self) -> bool {
        *self == 0
    }
}

impl EmptyDefinition for f32 {
    fn empty(&self) -> bool {
        *self == 0f32
    }
}

impl EmptyDefinition for f64 {
    fn empty(&self) -> bool {
        *self == 0f64
    }
}

impl<T> Rule for EmptyRule<T>
where
    T: EmptyDefinition,
{
    type Item = T;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.empty() {
            Ok(target)
        } else {
            Err(Error::new("The input value is not empty", target))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::Empty;

    #[test]
    fn test_add_empty() -> anyhow::Result<()> {
        let empty_1 = Empty::new(0)?;
        let empty_2 = Empty::new(0)?;
        let empty = empty_1 + empty_2;
        assert_eq!(empty.into_value(), 0);
        Ok(())
    }
}
