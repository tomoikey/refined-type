use crate::result::Error;
use crate::rule::Rule;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::marker::PhantomData;

/// Rule where the data is empty
/// ```rust
/// use refined_type::rule::{Empty, Rule};
///
/// assert!(Empty::<String>::validate("".to_string()).is_ok());
/// assert!(Empty::<String>::validate("non empty".to_string()).is_err());
///
/// assert!(Empty::<Vec<u8>>::validate(Vec::<u8>::new()).is_ok());
/// assert!(Empty::<Vec<u8>>::validate(vec![1, 2, 3]).is_err());
///
/// assert!(Empty::<u8>::validate(0).is_ok());
/// assert!(Empty::<u8>::validate(1).is_err());
/// ```
#[derive(Default)]
pub struct Empty<T> {
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

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.empty() {
            Ok(target)
        } else {
            Err(Error::new("The input value is not empty", target))
        }
    }
}
