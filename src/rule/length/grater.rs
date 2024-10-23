use crate::result::Error;
use crate::rule::{LengthDefinition, Rule};
use crate::Refined;

/// A type that holds a value satisfying the `LengthGreaterRule`
pub type LengthGreater<const THAN: usize, ITEM> = Refined<LengthGreaterRule<THAN, ITEM>>;

/// Rule where the input `ITEM` has a length greater than `THAN`
pub struct LengthGreaterRule<const THAN: usize, ITEM> {
    _phantom: std::marker::PhantomData<ITEM>,
}

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
