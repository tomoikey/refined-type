use crate::rule::{NonEmpty, NonEmptyRule};
use crate::Refined;
use std::borrow::Borrow;
use std::collections::hash_set::Difference;

use std::collections::HashSet;
use std::hash::{BuildHasher, Hash, RandomState};

pub type NonEmptyHashSet<T, S = RandomState> = Refined<NonEmptyRule<HashSet<T, S>>>;
pub type NonEmptyHashSetRule<T, S = RandomState> = NonEmptyRule<HashSet<T, S>>;

impl<T, S> NonEmptyHashSet<T, S> {
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> NonEmpty<std::collections::hash_set::IntoIter<T>> {
        Refined::new(self.into_value().into_iter())
            .ok()
            .expect("This error is always unreachable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn iter(&self) -> NonEmpty<std::collections::hash_set::Iter<T>> {
        Refined::new(self.value().iter())
            .ok()
            .expect("This error is always unreachable")
    }

    pub fn len(&self) -> usize {
        self.value().len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn capacity(&self) -> usize {
        self.value().capacity()
    }

    pub fn hasher(&self) -> &S {
        self.value().hasher()
    }
}

impl<T, S> NonEmptyHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    pub fn insert(self, value: T) -> Self {
        let mut result = self.into_value();
        result.insert(value);
        Refined::new(result)
            .ok()
            .expect("This error is always unreachable")
    }

    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.value().get(value)
    }

    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.value().contains(value)
    }

    pub fn difference<'a>(&'a self, other: &'a HashSet<T, S>) -> Difference<'a, T, S> {
        self.value().difference(other)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::NonEmptyHashSet;
    use std::collections::HashSet;

    #[test]
    fn test_err() -> anyhow::Result<()> {
        let set_result = NonEmptyHashSet::new(HashSet::<u8>::new());
        assert!(set_result.is_err());
        Ok(())
    }

    #[test]
    fn test_len() -> anyhow::Result<()> {
        let mut set = HashSet::new();
        set.insert(1);
        let set = NonEmptyHashSet::new(set)?;
        assert_eq!(set.len(), 1);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> anyhow::Result<()> {
        let mut set = HashSet::new();
        set.insert(1);
        let set = NonEmptyHashSet::new(set)?;
        assert!(!set.is_empty());
        Ok(())
    }

    #[test]
    fn test_insert() -> anyhow::Result<()> {
        let mut set_origin = HashSet::new();
        set_origin.insert(1);

        let set = NonEmptyHashSet::new(set_origin.clone())?.insert(2);

        set_origin.insert(2);
        assert_eq!(set.into_value(), set_origin);
        Ok(())
    }

    #[test]
    fn test_is_get() -> anyhow::Result<()> {
        let mut set = HashSet::new();
        set.insert(1);
        let set = NonEmptyHashSet::new(set)?;
        assert_eq!(set.get(&1), Some(&1));
        Ok(())
    }

    #[test]
    fn test_is_contains() -> anyhow::Result<()> {
        let mut set_origin = HashSet::new();
        set_origin.insert(1);
        let set = NonEmptyHashSet::new(set_origin.clone())?.insert(2);
        assert!(set.contains(&1));
        Ok(())
    }

    #[test]
    fn test_is_difference() -> anyhow::Result<()> {
        let mut set_origin = HashSet::new();
        set_origin.insert(1);
        let set = NonEmptyHashSet::new(set_origin.clone())?.insert(2);
        let difference = set.difference(&set_origin);
        assert_eq!(difference.count(), 1);
        assert_eq!(set.difference(&set_origin).next(), Some(&2));
        Ok(())
    }
}
