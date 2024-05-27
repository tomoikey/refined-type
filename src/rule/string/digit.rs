use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use regex::Regex;

/// A type that holds a value satisfying the `DigitRule`
pub type Digit = Refined<DigitRule>;
/// Rule where the input `String` contains only digit characters
pub struct DigitRule;

impl Rule for DigitRule {
    type Item = String;
    fn validate(target: &Self::Item) -> Result<(), Error> {
        let regex = Regex::new(r"[0-9]*").expect("Invalid regex");
        let is_valid = regex
            .find(target.as_str())
            .is_some_and(|matched| matched.as_str() == target.as_str());
        if is_valid {
            Ok(())
        } else {
            Err(Error::new("The input `String` have some digit characters"))
        }
    }
}

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
