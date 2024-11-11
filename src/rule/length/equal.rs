use crate::result::Error;
use crate::rule::{LengthDefinition, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LengthEqualRule`
pub type LengthEqual<const LENGTH: usize, ITEM> = Refined<LengthEqualRule<LENGTH, ITEM>>;

/// A type that holds a value satisfying the `LengthEqualVecRule`
pub type LengthEqualVec<const LENGTH: usize, ITEM> = Refined<LengthEqualVecRule<LENGTH, ITEM>>;

/// A type that holds a value satisfying the `LengthEqualVecDequeRule`
pub type LengthEqualVecDeque<const LENGTH: usize, ITEM> =
    Refined<LengthEqualVecDequeRule<LENGTH, ITEM>>;

/// A type that holds a value satisfying the `LengthEqualHashMapRule`
pub type LengthEqualHashMap<const LENGTH: usize, K, V> =
    Refined<LengthEqualHashMapRule<LENGTH, K, V>>;

/// A type that holds a value satisfying the `LengthEqualHashSetRule`
pub type LengthEqualHashSet<const LENGTH: usize, ITEM> =
    Refined<LengthEqualHashSetRule<LENGTH, ITEM>>;

/// A type that holds a value satisfying the `LengthEqualStringRule`
pub type LengthEqualString<const LENGTH: usize> = LengthEqual<LENGTH, String>;

/// A type that holds a value satisfying the `LengthEqualStrRule`
pub type LengthEqualStr<'a, const LENGTH: usize> = LengthEqual<LENGTH, &'a str>;

/// Rule where the input `ITEM` has a length equal to `LENGTH`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct LengthEqualRule<const LENGTH: usize, ITEM> {
    _phantom: std::marker::PhantomData<ITEM>,
}

/// Rule where the input `Vec` has a length equal to `LENGTH`
pub type LengthEqualVecRule<const LENGTH: usize, T> = LengthEqualRule<LENGTH, Vec<T>>;

/// Rule where the input `VecDeque` has a length equal to `LENGTH`
pub type LengthEqualVecDequeRule<const LENGTH: usize, T> =
    LengthEqualRule<LENGTH, std::collections::VecDeque<T>>;

/// Rule where the input `HashMap` has a length equal to `LENGTH`
pub type LengthEqualHashMapRule<const LENGTH: usize, K, V> =
    LengthEqualRule<LENGTH, std::collections::HashMap<K, V>>;

/// Rule where the input `HashSet` has a length equal to `LENGTH`
pub type LengthEqualHashSetRule<const LENGTH: usize, T> =
    LengthEqualRule<LENGTH, std::collections::HashSet<T>>;

/// Rule where the input `String` has a length equal to `LENGTH`
pub type LengthEqualStringRule<const LENGTH: usize> = LengthEqualRule<LENGTH, String>;

/// Rule where the input `&str` has a length equal to `LENGTH`
pub type LengthEqualStrRule<'a, const LENGTH: usize> = LengthEqualRule<LENGTH, &'a str>;

impl<const LENGTH: usize, ITEM: LengthDefinition> Rule for LengthEqualRule<LENGTH, ITEM> {
    type Item = ITEM;
    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.length() == LENGTH {
            Ok(target)
        } else {
            Err(Error::new(
                target,
                format!("target length is not equal to {}", LENGTH),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::length::equal::LengthEqual;

    #[test]
    fn test_length_equal_5() -> Result<(), Error<&'static str>> {
        let target = "12345";
        let refined = LengthEqual::<5, _>::new(target)?;
        assert_eq!(refined.into_value(), "12345");
        Ok(())
    }

    #[test]
    fn test_length_equal_5_fail() {
        let target = "1234";
        let refined = LengthEqual::<5, _>::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_equal_10() -> Result<(), Error<&'static str>> {
        let target = "1234567890";
        let refined = LengthEqual::<10, _>::new(target)?;
        assert_eq!(refined.into_value(), "1234567890");
        Ok(())
    }

    #[test]
    fn test_length_equal_10_fail() {
        let target = "123456789";
        let refined = LengthEqual::<10, _>::new(target);
        assert!(refined.is_err());
    }
}
