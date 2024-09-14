use crate::result::Error;
use crate::rule::{ForAllRule, Rule};

impl<RULE> Rule for ForAllRule<RULE, String>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.chars().all(|item| RULE::validate(&item).is_ok()) {
            Ok(())
        } else {
            Err(Error::new(format!(
                "{target} does not satisfy the condition"
            )))
        }
    }
}

impl<RULE> Rule for ForAllRule<RULE, &'static str>
where
    RULE: Rule<Item = char>,
{
    type Item = &'static str;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.chars().all(|item| RULE::validate(&item).is_ok()) {
            Ok(())
        } else {
            Err(Error::new(format!(
                "{target} does not satisfy the condition"
            )))
        }
    }
}
