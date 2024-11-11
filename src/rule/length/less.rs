use crate::result::Error;
use crate::rule::{LengthDefinition, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LengthLessRule`
pub type LengthLess<const THAN: usize, ITEM> = Refined<LengthLessRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthLessVecRule`
pub type LengthLessVec<const THAN: usize, ITEM> = Refined<LengthLessVecRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthLessVecDequeRule`
pub type LengthLessVecDeque<const THAN: usize, ITEM> = Refined<LengthLessVecDequeRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthLessHashMapRule`
pub type LengthLessHashMap<const THAN: usize, K, V> = Refined<LengthLessHashMapRule<THAN, K, V>>;

/// A type that holds a value satisfying the `LengthLessHashSetRule`
pub type LengthLessHashSet<const THAN: usize, ITEM> = Refined<LengthLessHashSetRule<THAN, ITEM>>;

/// A type that holds a value satisfying the `LengthLessStringRule`
pub type LengthLessString<const THAN: usize> = LengthLess<THAN, String>;

/// A type that holds a value satisfying the `LengthLessStrRule`
pub type LengthLessStr<'a, const THAN: usize> = LengthLess<THAN, &'a str>;

/// Rule where the input `ITEM` has a length less than `THAN`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct LengthLessRule<const THAN: usize, ITEM> {
    _phantom: std::marker::PhantomData<ITEM>,
}

/// Rule where the input `Vec` has a length less than `THAN`
pub type LengthLessVecRule<const THAN: usize, ITEM> = LengthLessRule<THAN, Vec<ITEM>>;

/// Rule where the input `VecDeque` has a length less than `THAN`
pub type LengthLessVecDequeRule<const THAN: usize, ITEM> =
    LengthLessRule<THAN, std::collections::VecDeque<ITEM>>;

/// Rule where the input `HashMap` has a length less than `THAN`
pub type LengthLessHashMapRule<const THAN: usize, K, V> =
    LengthLessRule<THAN, std::collections::HashMap<K, V>>;

/// Rule where the input `HashSet` has a length less than `THAN`
pub type LengthLessHashSetRule<const THAN: usize, ITEM> =
    LengthLessRule<THAN, std::collections::HashSet<ITEM>>;

/// Rule where the input `String` has a length less than `THAN`
pub type LengthLessStringRule<const THAN: usize> = LengthLessRule<THAN, String>;

/// Rule where the input `&str` has a length less than `THAN`
pub type LengthLessStrRule<'a, const THAN: usize> = LengthLessRule<THAN, &'a str>;

impl<const THAN: usize, ITEM: LengthDefinition> Rule for LengthLessRule<THAN, ITEM> {
    type Item = ITEM;
    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if target.length() < THAN {
            Ok(target)
        } else {
            Err(Error::new(
                target,
                format!("target length is not less than {}", THAN),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::LengthLess;

    #[test]
    fn test_length_less_than_5() -> Result<(), Error<&'static str>> {
        let target = "1234";
        let refined = LengthLess::<5, _>::new(target)?;
        assert_eq!(refined.into_value(), "1234");
        Ok(())
    }

    #[test]
    fn test_length_less_than_5_fail() {
        let target = "12345";
        let refined = LengthLess::<5, _>::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_less_than_10() -> Result<(), Error<&'static str>> {
        let target = "123456789";
        let refined = LengthLess::<10, _>::new(target)?;
        assert_eq!(refined.into_value(), "123456789");
        Ok(())
    }

    #[test]
    fn test_length_less_than_10_fail() {
        let target = "1234567890";
        let refined = LengthLess::<10, _>::new(target);
        assert!(refined.is_err());
    }
}
