use crate::rule::Rule;
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Refined<RULE, T> {
    value: T,
    _phantom_data: PhantomData<RULE>,
}

impl<RULE, T> Display for Refined<RULE, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<RULE, T> TryFrom<Vec<T>> for Refined<RULE, Vec<T>>
where
    RULE: Rule<TARGET = T>,
{
    type Error = anyhow::Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let mut result = Vec::new();
        for i in value {
            result.push(RULE::validate(i)?)
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> TryFrom<VecDeque<T>> for Refined<RULE, VecDeque<T>>
where
    RULE: Rule<TARGET = T>,
{
    type Error = anyhow::Error;

    fn try_from(value: VecDeque<T>) -> std::result::Result<Self, Self::Error> {
        let mut result = VecDeque::new();
        for i in value {
            result.push_back(RULE::validate(i)?)
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> TryFrom<LinkedList<T>> for Refined<RULE, LinkedList<T>>
where
    RULE: Rule<TARGET = T>,
{
    type Error = anyhow::Error;

    fn try_from(value: LinkedList<T>) -> std::result::Result<Self, Self::Error> {
        let mut result = LinkedList::new();
        for i in value {
            result.push_back(RULE::validate(i)?)
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, K, V> TryFrom<BTreeMap<K, V>> for Refined<RULE, BTreeMap<K, V>>
where
    RULE: Rule<TARGET = V>,
    K: Ord,
{
    type Error = anyhow::Error;

    fn try_from(value: BTreeMap<K, V>) -> std::result::Result<Self, Self::Error> {
        let mut result = BTreeMap::new();
        for (k, v) in value {
            result.insert(k, RULE::validate(v)?);
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, K, V> TryFrom<HashMap<K, V>> for Refined<RULE, HashMap<K, V>>
where
    RULE: Rule<TARGET = V>,
    K: Eq + Hash,
{
    type Error = anyhow::Error;

    fn try_from(value: HashMap<K, V>) -> Result<Self, Self::Error> {
        let mut result = HashMap::new();
        for (k, v) in value {
            result.insert(k, RULE::validate(v)?);
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> TryFrom<HashSet<T>> for Refined<RULE, HashSet<T>>
where
    RULE: Rule<TARGET = T>,
    T: Eq + Hash,
{
    type Error = anyhow::Error;

    fn try_from(value: HashSet<T>) -> Result<Self, Self::Error> {
        let mut result = HashSet::new();
        for i in value {
            result.insert(RULE::validate(i)?);
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> TryFrom<BTreeSet<T>> for Refined<RULE, BTreeSet<T>>
where
    RULE: Rule<TARGET = T>,
    T: Ord,
{
    type Error = anyhow::Error;

    fn try_from(value: BTreeSet<T>) -> Result<Self, Self::Error> {
        let mut result = BTreeSet::new();
        for i in value {
            result.insert(RULE::validate(i)?);
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> TryFrom<BinaryHeap<T>> for Refined<RULE, BinaryHeap<T>>
where
    RULE: Rule<TARGET = T>,
    T: Ord,
{
    type Error = anyhow::Error;

    fn try_from(value: BinaryHeap<T>) -> std::result::Result<Self, Self::Error> {
        let mut result = BinaryHeap::new();
        for i in value {
            result.push(RULE::validate(i)?);
        }
        Ok(Self {
            value: result,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> Refined<RULE, T>
where
    RULE: Rule<TARGET = T>,
{
    pub fn new(value: T) -> Result<Self> {
        Ok(Self {
            value: RULE::validate(value)?,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> Deref for Refined<RULE, T>
where
    RULE: Rule<TARGET = T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod test {
    use crate::refined::Refined;
    use crate::rule::{NonEmptyString, NonEmptyStringRule};
    use anyhow::Result;

    #[test]
    fn test_non_empty_string_ok() -> Result<()> {
        let non_empty_string: NonEmptyString = Refined::new("Hello".to_string())?;
        assert_eq!(non_empty_string.value, "Hello");
        Ok(())
    }

    #[test]
    fn test_non_empty_string_err() -> Result<()> {
        let non_empty_string: Result<NonEmptyString> = Refined::new("".to_string());
        assert!(non_empty_string.is_err());
        Ok(())
    }

    #[test]
    fn test_array_of_non_empty_string_ok() -> Result<()> {
        let strings = vec![
            "Good Morning".to_string(),
            "Hello".to_string(),
            "Good Evening".to_string(),
        ];
        let array_non_empty_string: Refined<NonEmptyStringRule, Vec<String>> =
            Refined::try_from(strings.clone())?;
        assert_eq!(array_non_empty_string.value, strings);
        Ok(())
    }

    #[test]
    fn test_array_of_non_empty_string_err() -> Result<()> {
        let strings = vec![
            "Good Morning".to_string(),
            "".to_string(),
            "Good Evening".to_string(),
        ];
        let array_non_empty_string_result: Result<Refined<NonEmptyStringRule, Vec<String>>> =
            Refined::try_from(strings.clone());
        assert!(array_non_empty_string_result.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_display() -> Result<()> {
        let non_empty_string: NonEmptyString = Refined::new("Hello".to_string())?;
        assert_eq!(format!("{}", non_empty_string), "Hello");
        Ok(())
    }
}
