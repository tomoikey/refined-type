use crate::error::{Error, Result};
use crate::refined::Refined;
use crate::rule::Rule;

/// This is a predicate type representing a non-empty string
pub type NonEmptyString = Refined<NonEmptyStringRule, String>;

pub struct NonEmptyStringRule;

impl Rule for NonEmptyStringRule {
    type TARGET = String;

    fn validate(target: Self::TARGET) -> Result<Self::TARGET> {
        if target.is_empty() {
            Err(Error::new("The input `String` is empty"))
        } else {
            Ok(target)
        }
    }
}
