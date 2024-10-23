use crate::result::Error;
use crate::rule::{LengthDefinition, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LengthLessRule`
pub type LengthLess<const THAN: usize, ITEM> = Refined<LengthLessRule<THAN, ITEM>>;

/// Rule where the input `ITEM` has a length less than `THAN`
pub struct LengthLessRule<const THAN: usize, ITEM> {
    _phantom: std::marker::PhantomData<ITEM>,
}

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
