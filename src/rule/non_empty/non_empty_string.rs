use crate::refined::Refined;
use crate::rule::NonEmpty;

/// This is a predicate type representing a non-empty string
pub type NonEmptyString = Refined<NonEmptyStringRule>;

pub type NonEmptyStringRule = NonEmpty<String>;

#[cfg(test)]
mod test {
    use crate::rule::{NonEmptyStringRule, Rule};

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyStringRule::validate("hello".to_string()).is_ok());
        assert!(NonEmptyStringRule::validate("".to_string()).is_err());
    }
}
