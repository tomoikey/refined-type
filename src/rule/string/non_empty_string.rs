use crate::refined::Refined;
use crate::rule::composer::Not;
use crate::rule::empty::Empty;

/// This is a predicate type representing a non-empty string
pub type NonEmptyString<'a> = Refined<NonEmptyStringRule<'a>, String>;

pub type NonEmptyStringRule<'a> = Not<'a, String, Empty<String>>;

#[cfg(test)]
mod test {
    use crate::rule::{NonEmptyStringRule, Rule};

    fn test_non_empty_string() {
        let rule = NonEmptyStringRule::default();

        assert!(rule.validate("hello".to_string()).is_ok());
        assert!(rule.validate("".to_string()).is_err());
    }
}
