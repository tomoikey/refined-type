use crate::result::Error;
use crate::rule::{LengthDefinition, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LengthEqualRule`
pub type LengthEqual<const LENGTH: usize, ITEM> = Refined<LengthEqualRule<LENGTH, ITEM>>;

/// Rule where the input `ITEM` has a length equal to `LENGTH`
pub struct LengthEqualRule<const LENGTH: usize, ITEM> {
    _phantom: std::marker::PhantomData<ITEM>,
}

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
