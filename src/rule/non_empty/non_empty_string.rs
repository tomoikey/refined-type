use crate::refined::Refined;
use crate::result::Error;
use crate::rule::NonEmptyRule;
use std::ops::Add;
use std::str::FromStr;

/// This is a predicate type representing a non-empty string
/// # Example
/// ```rust
/// # use refined_type::rule::NonEmptyString;
///
/// let non_empty_string_1 = NonEmptyString::new("Hello".to_string()).unwrap();
/// let non_empty_string_2 = NonEmptyString::new("World".to_string()).unwrap();
/// let non_empty_string = non_empty_string_1 + non_empty_string_2;
///
/// assert_eq!(non_empty_string.into_value(), "HelloWorld");
/// ```
pub type NonEmptyString = Refined<NonEmptyStringRule>;
pub type NonEmptyStringRule = NonEmptyRule<String>;

impl FromStr for NonEmptyString {
    type Err = Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Refined::new(s.to_string())
    }
}

impl Add for NonEmptyString {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Refined::new(format!("{}{}", self.into_value(), rhs.into_value()))
            .expect("This error is always unreachable")
    }
}

#[cfg(test)]
mod test {
    use crate::rule::{NonEmptyString, NonEmptyStringRule, Rule};
    use std::str::FromStr;

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyStringRule::validate("hello".to_string()).is_ok());
        assert!(NonEmptyStringRule::validate("".to_string()).is_err());
    }

    #[test]
    fn test_add_string() -> anyhow::Result<()> {
        let non_empty_string_1 = NonEmptyString::new("Hello".to_string())?;
        let non_empty_string_2 = NonEmptyString::new("World".to_string())?;
        let non_empty_string = non_empty_string_1 + non_empty_string_2;

        assert_eq!(non_empty_string.into_value(), "HelloWorld");
        Ok(())
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        let non_empty_string = NonEmptyString::from_str("Hello")?;
        assert_eq!(non_empty_string.into_value(), "Hello");
        Ok(())
    }
}
