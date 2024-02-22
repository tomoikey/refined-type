use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Add;

pub type Empty<T> = Refined<EmptyRule<T>>;

impl<T> Add for Empty<T>
where
    T: EmptyDefinition,
{
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        // `T` implements `EmptyDefinition`. Therefore, this `expect` is safe.
        Refined::new(self.into_value().empty())
            .ok()
            .expect("This error is always unreachable")
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

pub trait EmptyDefinition: PartialEq {
    fn empty(&self) -> Self;
}

impl EmptyDefinition for String {
    fn empty(&self) -> Self {
        "".to_string()
    }
}

impl EmptyDefinition for &str {
    fn empty(&self) -> Self {
        ""
    }
}

impl<T> EmptyDefinition for Vec<T>
where
    T: PartialEq,
{
    fn empty(&self) -> Self {
        Vec::new()
    }
}

impl<T> EmptyDefinition for HashSet<T>
where
    T: PartialEq + Eq + Hash,
{
    fn empty(&self) -> Self {
        HashSet::new()
    }
}

impl<K, V> EmptyDefinition for HashMap<K, V>
where
    K: PartialEq + Eq + Hash,
    V: PartialEq,
{
    fn empty(&self) -> Self {
        HashMap::new()
    }
}

impl<T> EmptyDefinition for BTreeSet<T>
where
    T: PartialEq,
{
    fn empty(&self) -> Self {
        BTreeSet::new()
    }
}

impl<K, V> EmptyDefinition for BTreeMap<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn empty(&self) -> Self {
        BTreeMap::new()
    }
}

impl EmptyDefinition for u8 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for u16 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for u32 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for u64 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for u128 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for usize {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for i8 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for i16 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for i32 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for i64 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for i128 {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for isize {
    fn empty(&self) -> Self {
        0
    }
}

impl EmptyDefinition for f32 {
    fn empty(&self) -> Self {
        0f32
    }
}

impl EmptyDefinition for f64 {
    fn empty(&self) -> Self {
        0f64
    }
}

impl<T> Rule for EmptyRule<T>
where
    T: EmptyDefinition + PartialEq,
{
    type Item = T;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target == target.empty() {
            Ok(target)
        } else {
            Err(Error::new("The input value is not empty", target))
        }
    }
}
