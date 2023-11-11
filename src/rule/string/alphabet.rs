use crate::{Error, Refined, Result, Rule};
use regex::Regex;

pub type Alphabet = Refined<AlphabetRule, String>;

/// A string consisting entirely of alphabetic characters
pub struct AlphabetRule;

impl Rule for AlphabetRule {
    type TARGET = String;

    fn validate(&self, target: Self::TARGET) -> Result<Self::TARGET> {
        let regex = Regex::new(r"[a-zA-Z]*").expect("Invalid regex");
        let is_valid = regex
            .find(target.as_str())
            .is_some_and(|matched| matched.as_str() == target.as_str());
        if is_valid {
            Ok(target)
        } else {
            Err(Error::new(
                "The input `String` have some non-alphabet characters",
            ))
        }
    }
}
