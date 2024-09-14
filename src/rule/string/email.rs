use crate::{declare_regex_rule, Refined};

/// A type that holds a value satisfying the `EmailRule`
///
/// # Example
/// ```rust
/// # use refined_type::rule::Email;
///
/// let valid = "sample@example.com".to_string();
/// assert!(Email::new(valid).is_ok());
///
/// let invalid = "example.com".to_string();
/// assert!(Email::new(invalid).is_err());
/// ```
pub type Email<STRING> = Refined<EmailRule<STRING>>;

declare_regex_rule![
    pub EmailRule,
    r"^[a-zA-Z0-9_.+-]+@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"
];

#[cfg(test)]
mod test {
    use crate::rule::string::email::EmailRule;
    use crate::rule::Rule;

    #[test]
    fn test_valid_email() {
        let valid = "sample@example.com".to_string();
        assert!(EmailRule::validate(&valid).is_ok())
    }

    #[test]
    fn test_invalid_email_1() {
        let invalid = "example.com".to_string();
        assert!(EmailRule::validate(&invalid).is_err())
    }

    #[test]
    fn test_invalid_email_2() {
        let invalid = "@".to_string();
        assert!(EmailRule::validate(&invalid).is_err())
    }
}
