use crate::refined::Refined;
use crate::rule::NonEmptyRule;
use std::ops::Add;

/// This is a predicate type representing a non-empty string
pub type NonEmptyString = Refined<NonEmptyStringRule>;

impl Add for NonEmptyString {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Refined::new(format!("{}{}", self.into_value(), rhs.into_value()))
            .expect("This error is always unreachable")
    }
}

pub type NonEmptyStringRule = NonEmptyRule<String>;

#[cfg(test)]
mod test {
    use crate::rule::{NonEmptyString, NonEmptyStringRule, Rule};

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
}
