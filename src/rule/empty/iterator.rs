use crate::rule::EmptyDefinition;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::iter::Map;

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
