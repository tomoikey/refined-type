mod equal;
mod even;
mod greater;
mod less;
mod odd;

pub use even::*;
pub use odd::*;

#[cfg(test)]
mod test {
    use crate::rule::composer::{And, Or};
    use crate::{equal_rule, greater_rule, less_rule, Refined};

    greater_rule!((18, u8));
    less_rule!((80, u8));
    equal_rule!((18, u8), (80, u8));

    type TargetAge = Refined<TargetAgeRule>;

    // 18 <= age
    type TargetAge18OrMore = Or<EqualRule18u8, GreaterRule18u8>;

    // age <= 80
    type TargetAge80OrLess = Or<EqualRule80u8, LessRule80u8>;

    // 18 <= age <= 80
    type TargetAgeRule = And<TargetAge18OrMore, TargetAge80OrLess>;

    #[test]
    fn test_age_0() -> anyhow::Result<()> {
        let age_result = TargetAge::new(0);
        assert!(age_result.is_err());
        Ok(())
    }

    #[test]
    fn test_age_18() -> anyhow::Result<()> {
        let age_result = TargetAge::new(18);
        assert!(age_result.is_ok());
        Ok(())
    }

    #[test]
    fn test_age_50() -> anyhow::Result<()> {
        let age_result = TargetAge::new(50);
        assert!(age_result.is_ok());
        Ok(())
    }

    #[test]
    fn test_age_80() -> anyhow::Result<()> {
        let age_result = TargetAge::new(80);
        assert!(age_result.is_ok());
        Ok(())
    }

    #[test]
    fn test_age_81() -> anyhow::Result<()> {
        let age_result = TargetAge::new(81);
        assert!(age_result.is_err());
        Ok(())
    }
}
