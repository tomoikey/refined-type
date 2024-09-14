use crate::{declare_regex_rule, Refined};

/// A type that holds a value satisfying the `AlphabetRule`
///
/// # Example
/// ```rust
/// # use refined_type::rule::Alphabet;
///
/// let alphabet_result = Alphabet::new("alphabet");
/// assert!(alphabet_result.is_ok());
///
/// let alphabet_result = Alphabet::new("I am the 1st".to_string());
/// assert!(alphabet_result.is_err());
/// ```
pub type Alphabet<STRING> = Refined<AlphabetRule<STRING>>;

declare_regex_rule![pub AlphabetRule, r"^[a-zA-Z]*$"];

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
