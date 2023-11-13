use crate::result::Error;
use crate::rule::composer::Not;
use crate::rule::Rule;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::marker::PhantomData;

pub type NonEmpty<'a, T> = Not<'a, T, Empty<T>>;

pub struct Empty<T> {
    _phantom_data: PhantomData<T>,
}

impl<T> Default for Empty<T>
where
    T: EmptyDefinition,
{
    fn default() -> Self {
        Self {
            _phantom_data: Default::default(),
        }
    }
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

impl<T> EmptyDefinition for HashSet<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> EmptyDefinition for HashMap<K, V> {
    fn empty(&self) -> bool {
        self.is_empty()
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

impl<T> Rule for Empty<T>
where
    T: EmptyDefinition,
{
    type Item = T;

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if !target.empty() {
            Ok(target)
        } else {
            Err(Error::new("The input value is not empty", target))
        }
    }
}
