use crate::rule::{LengthEqualRule, LengthGreaterRule, LengthLessRule};
use crate::{And, Or, Refined};

pub type LengthMinMax<const MIN: usize, const MAX: usize, ITEM> =
    Refined<LengthMinMaxRule<MIN, MAX, ITEM>>;

pub type LengthMinMaxRule<const MIN: usize, const MAX: usize, ITEM> = And![
    Or![LengthEqualRule<MIN, ITEM>, LengthGreaterRule<MIN, ITEM>],
    Or![LengthEqualRule<MAX, ITEM>, LengthLessRule<MAX, ITEM>]
];

#[cfg(test)]
mod tests {
    use crate::result::Error;
    use crate::rule::length::min_max::LengthMinMax;

    #[test]
    fn test_length_min_max_5_10() -> Result<(), Error<&'static str>> {
        let target = "123456";
        let refined = LengthMinMax::<5, 10, _>::new(target)?;
        assert_eq!(refined.into_value(), "123456");
        Ok(())
    }

    #[test]
    fn test_length_min_max_5_10_fail() {
        let target = "1234";
        let refined = LengthMinMax::<5, 10, _>::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_min_max_5_10_fail_2() {
        let target = "12345678901";
        let refined = LengthMinMax::<5, 10, _>::new(target);
        assert!(refined.is_err());
    }
}
