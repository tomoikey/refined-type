use crate::rule::{ReverseRule, Rule};

impl<RULE> Rule for ReverseRule<RULE, String>
where
    RULE: Rule<Item = String>,
{
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        match RULE::validate(target.chars().rev().collect()) {
            Ok(value) => Ok(value.chars().rev().collect()),
            Err(e) => {
                let message = e.to_string();
                Err(crate::result::Error::new(
                    e.into_value().chars().rev().collect(),
                    message,
                ))
            }
        }
    }
}
