use crate::rule::{NonEmpty, NonEmptyRule};
use crate::Refined;
use std::borrow::Borrow;
use std::collections::hash_map::{IntoKeys, IntoValues, Keys, Values};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, RandomState};

pub type NonEmptyHashMap<K, V, S = RandomState> = Refined<NonEmptyHashMapRule<K, V, S>>;
pub type NonEmptyHashMapRule<K, V, S = RandomState> = NonEmptyRule<HashMap<K, V, S>>;

impl<K, V, S> NonEmptyHashMap<K, V, S> {
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> NonEmpty<std::collections::hash_map::IntoIter<K, V>> {
        Refined::new(self.into_value().into_iter())
            .ok()
            .expect("This error is always unreachable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn iter(&self) -> NonEmpty<std::collections::hash_map::Iter<K, V>> {
        Refined::new(self.value().iter())
            .ok()
            .expect("This error is always unreachable")
    }

    pub fn hasher(&self) -> &S {
        self.value().hasher()
    }

    pub fn capacity(&self) -> usize {
        self.value().capacity()
    }

    pub fn len(&self) -> usize {
        self.value().len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn keys(&self) -> Keys<K, V> {
        self.value().keys()
    }

    pub fn into_keys(self) -> IntoKeys<K, V> {
        self.into_value().into_keys()
    }

    pub fn values(&self) -> Values<K, V> {
        self.value().values()
    }

    pub fn into_values(self) -> IntoValues<K, V> {
        self.into_value().into_values()
    }
}

impl<K, V, S> NonEmptyHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.value().get(k)
    }

    pub fn insert(self, k: K, v: V) -> Self {
        let mut result = self.into_value();
        result.insert(k, v);
        Refined::new(result)
            .ok()
            .expect("This error is always unreachable")
    }
}

#[cfg(test)]
mod test {
    use crate::rule::{NonEmptyHashMap, NonEmptyVec};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_map_len() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        let map = NonEmptyHashMap::new(map)?;
        assert_eq!(map.len(), 1);
        Ok(())
    }

    #[test]
    fn test_map_is_empty() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        let map = NonEmptyHashMap::new(map)?;
        assert!(!map.is_empty());
        Ok(())
    }

    #[test]
    fn test_map_keys() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        let map = NonEmptyHashMap::new(map)?;
        assert_eq!(
            map.keys().collect::<HashSet<_>>(),
            vec![&"1", &"2"].into_iter().collect()
        );
        Ok(())
    }

    #[test]
    fn test_map_into_keys() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        let map = NonEmptyHashMap::new(map)?;
        assert_eq!(
            map.into_keys().collect::<HashSet<_>>(),
            vec!["1", "2"].into_iter().collect()
        );
        Ok(())
    }

    #[test]
    fn test_map_values() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        let map = NonEmptyHashMap::new(map)?;
        assert_eq!(
            map.values().collect::<HashSet<_>>(),
            vec![&1, &2].into_iter().collect()
        );
        Ok(())
    }

    #[test]
    fn test_map_into_values() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        let map = NonEmptyHashMap::new(map)?;
        assert_eq!(
            map.into_values().collect::<HashSet<_>>(),
            vec![1, 2].into_iter().collect()
        );
        Ok(())
    }

    #[test]
    fn test_map_get() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        let map = NonEmptyHashMap::new(map)?;
        assert_eq!(map.get("1"), Some(&1));
        Ok(())
    }

    #[test]
    fn test_map_insert() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        let map = NonEmptyHashMap::new(map)?.insert("3", 3);
        let vec: NonEmptyVec<(&str, i32)> = map.into_iter().collect();
        assert_eq!(
            vec.into_value().into_iter().collect::<HashSet<_>>(),
            vec![("1", 1), ("2", 2), ("3", 3)].into_iter().collect()
        );
        Ok(())
    }
}
