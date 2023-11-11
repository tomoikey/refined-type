use crate::rule::Rule;
use anyhow::{anyhow, Result};

pub struct NonEmptyStringRule;

impl Rule for NonEmptyStringRule {
    type TARGET = String;

    fn validate(target: Self::TARGET) -> Result<Self::TARGET> {
        if target.is_empty() {
            Err(anyhow!("The input `String` is empty"))
        } else {
            Ok(target)
        }
    }
}
