use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use regex::Regex;

/// The `Alphabet` type represents a type that can contain zero or more alphabetic characters.
///
/// # Example
/// ```rust
/// # use refined_type::rule::Alphabet;
///
/// let alphabet_result = Alphabet::new("alphabet".to_string());
/// assert!(alphabet_result.is_ok());
///
/// let alphabet_result = Alphabet::new("I am the 1st".to_string());
/// assert!(alphabet_result.is_err());
/// ```
pub type Alphabet = Refined<AlphabetRule>;

/// A string consisting entirely of alphabetic characters
pub struct AlphabetRule;

impl Rule for AlphabetRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let regex = Regex::new(r"[a-zA-Z]*").expect("Invalid regex");
        let is_valid = regex
            .find(target.as_str())
            .is_some_and(|matched| matched.as_str() == target.as_str());
        if is_valid {
            Ok(target)
        } else {
            Err(Error::new(
                "The input `String` have some non-alphabet characters",
                target,
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::Alphabet;

    #[test]
    fn test_alphabet_ok_empty() {
        let alphabet_result = Alphabet::new("".to_string());
        assert!(alphabet_result.is_ok());
    }

    #[test]
    fn test_alphabet_ok_non_empty() {
        let alphabet_result = Alphabet::new("alphabet".to_string());
        assert!(alphabet_result.is_ok());
    }

    #[test]
    fn test_alphabet_err() {
        let alphabet_result = Alphabet::new("I am the 1st".to_string());
        assert!(alphabet_result.is_err());
    }
}
