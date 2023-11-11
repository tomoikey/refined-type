use crate::error::{Error, Result};
use crate::refined::Refined;
use crate::rule::Rule;

/// This is a predicate type representing a non-empty string
pub type NonEmptyString = Refined<NonEmptyStringRule, String>;

pub struct NonEmptyStringRule;

impl Rule for NonEmptyStringRule {
    type Item = String;

    fn validate(&self, target: Self::Item) -> Result<Self::Item> {
        if target.is_empty() {
            Err(Error::new("The input `String` is empty"))
        } else {
            Ok(target)
        }
    }
}
