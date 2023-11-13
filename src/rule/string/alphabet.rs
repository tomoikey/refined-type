use crate::result::{Error, Result};
use crate::rule::Rule;
use crate::Refined;
use regex::Regex;

pub type Alphabet = Refined<AlphabetRule, String>;

/// A string consisting entirely of alphabetic characters
pub struct AlphabetRule;

impl Rule for AlphabetRule {
    type Item = String;

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Self::Item> {
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
