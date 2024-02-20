use crate::refined::Refined;
use crate::rule::closed_algebraic::ClosedAlgebraic;
use crate::rule::NonEmpty;

/// This is a predicate type representing a non-empty string
pub type NonEmptyString = Refined<NonEmptyStringRule>;

pub type NonEmptyStringRule = NonEmpty<String>;

/// # Math Theory
/// NonEmpty + NonEmpty = NonEmpty
impl ClosedAlgebraic for NonEmptyString {
    fn plus(self, that: NonEmptyString) -> NonEmptyString {
        Refined::new(format!("{}{}", self.into_value(), that.into_value())).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::rule::{NonEmptyStringRule, Rule};

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyStringRule::validate("hello".to_string()).is_ok());
        assert!(NonEmptyStringRule::validate("".to_string()).is_err());
    }
}
