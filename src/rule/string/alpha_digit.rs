use crate::{declare_regex_rule, Refined};

/// A type that holds a value satisfying the `AlphaDigitRule`
pub type AlphaDigit = Refined<AlphaDigitRule>;

declare_regex_rule![pub AlphaDigitRule, r"^[0-9a-zA-Z]*$"];

#[cfg(test)]
mod test {
    use crate::rule::AlphaDigit;

    #[test]
    fn test_alpha_digit_ok_1() {
        let alpha_digit = AlphaDigit::new("1234567890".to_string());
        assert!(alpha_digit.is_ok());
    }

    #[test]
    fn test_alpha_digit_ok_2() {
        let alpha_digit = AlphaDigit::new("".to_string());
        assert!(alpha_digit.is_ok());
    }

    #[test]
    fn test_alpha_digit_ok_3() {
        let alpha_digit = AlphaDigit::new("1234567890abc".to_string());
        assert!(alpha_digit.is_ok());
    }

    #[test]
    fn test_alpha_digit_err() {
        let alpha_digit = AlphaDigit::new("1234567890abcこんにちは".to_string());
        assert!(alpha_digit.is_err());
    }
}
