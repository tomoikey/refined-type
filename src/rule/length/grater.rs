use crate::result::Error;
use crate::rule::{LengthDefinition, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LengthGreaterRule`
pub type LengthGreater<const THAN: usize, ITEM> = Refined<LengthGreaterRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthGreaterVecRule`
pub type LengthGreaterVec<const THAN: usize, ITEM> = Refined<LengthGreaterVecRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthGreaterVecDequeRule`
pub type LengthGreaterVecDeque<const THAN: usize, ITEM> =
    Refined<LengthGreaterVecDequeRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthGreaterHashMapRule`
pub type LengthGreaterHashMap<const THAN: usize, K, V> =
    Refined<LengthGreaterHashMapRule<THAN, K, V>>;

/// A type that holds a value satisfying the `LengthGreaterHashSetRule`
pub type LengthGreaterHashSet<const THAN: usize, ITEM> =
    Refined<LengthGreaterHashSetRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthGreaterStringRule`
pub type LengthGreaterString<const THAN: usize> = LengthGreater<THAN, String>;

/// A type that holds a value satisfying the `LengthGreaterStrRule`
pub type LengthGreaterStr<'a, const THAN: usize> = LengthGreater<THAN, &'a str>;

/// Rule where the input `ITEM` has a length greater than `THAN`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct LengthGreaterRule<const THAN: usize, ITEM> {
    _phantom: std::marker::PhantomData<ITEM>,
}

/// Rule where the input `Vec` has a length greater than `THAN`
pub type LengthGreaterVecRule<const THAN: usize, ITEM> = LengthGreaterRule<THAN, Vec<ITEM>>;

/// Rule where the input `VecDeque` has a length greater than `THAN`
pub type LengthGreaterVecDequeRule<const THAN: usize, ITEM> =
    LengthGreaterRule<THAN, std::collections::VecDeque<ITEM>>;

/// Rule where the input `HashMap` has a length greater than `THAN`
pub type LengthGreaterHashMapRule<const THAN: usize, K, V> =
    LengthGreaterRule<THAN, std::collections::HashMap<K, V>>;

/// Rule where the input `HashSet` has a length greater than `THAN`
pub type LengthGreaterHashSetRule<const THAN: usize, ITEM> =
    LengthGreaterRule<THAN, std::collections::HashSet<ITEM>>;

/// Rule where the input `String` has a length greater than `THAN`
pub type LengthGreaterStringRule<const THAN: usize> = LengthGreaterRule<THAN, String>;

/// Rule where the input `&str` has a length greater than `THAN`
pub type LengthGreaterStrRule<'a, const THAN: usize> = LengthGreaterRule<THAN, &'a str>;

impl<const THAN: usize, ITEM: LengthDefinition> Rule for LengthGreaterRule<THAN, ITEM> {
    type Item = ITEM;
    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.length() > THAN {
            Ok(target)
        } else {
            Err(Error::new(
                target,
                format!("target length is not greater than {}", THAN),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::length::grater::LengthGreater;

    #[test]
    fn test_length_greater_than_5() -> Result<(), Error<&'static str>> {
        let target = "123456";
        let refined = LengthGreater::<5, _>::new(target)?;
        assert_eq!(refined.into_value(), "123456");
        Ok(())
    }

    #[test]
    fn test_length_greater_than_5_fail() {
        let target = "1234";
        let refined = LengthGreater::<5, _>::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_greater_than_10() -> Result<(), Error<&'static str>> {
        let target = "12345678901";
        let refined = LengthGreater::<10, _>::new(target)?;
        assert_eq!(refined.into_value(), "12345678901");
        Ok(())
    }

    #[test]
    fn test_length_greater_than_10_fail() {
        let target = "123456789";
        let refined = LengthGreater::<10, _>::new(target);
        assert!(refined.is_err());
    }
}
