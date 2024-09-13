use crate::{declare_regex_rule, Refined};

/// A type that holds a value satisfying the `DigitRule`
pub type Digit<T> = Refined<DigitRule<T>>;

declare_regex_rule![pub DigitRule,r"^[0-9]*$"];

#[cfg(test)]
mod test {
    use crate::rule::string::digit::Digit;

    #[test]
    fn test_digit_ok_1() {
        let digit = Digit::new("1234567890".to_string());
        assert!(digit.is_ok());
    }

    #[test]
    fn test_digit_ok_2() {
        let digit = Digit::new("".to_string());
        assert!(digit.is_ok());
    }

    #[test]
    fn test_digit_err() {
        let digit = Digit::new("1234567890abc".to_string());
        assert!(digit.is_err());
    }
}
