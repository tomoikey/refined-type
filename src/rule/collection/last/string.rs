use crate::rule::{LastRule, Rule};

impl<RULE> Rule for LastRule<RULE, String>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), crate::result::Error> {
        let item = target
            .chars()
            .last()
            .ok_or_else(|| crate::result::Error::new("the string is empty"))?;
        if RULE::validate(&item).is_ok() {
            Ok(())
        } else {
            Err(crate::result::Error::new(
                "the last character does not satisfy the condition",
            ))
        }
    }
}

impl<'a, RULE> Rule for LastRule<RULE, &'a str>
where
    RULE: Rule<Item = char>,
{
    type Item = &'a str;

    fn validate(target: &Self::Item) -> Result<(), crate::result::Error> {
        let item = target
            .chars()
            .last()
            .ok_or_else(|| crate::result::Error::new("the string is empty"))?;
        if RULE::validate(&item).is_ok() {
            Ok(())
        } else {
            Err(crate::result::Error::new(
                "the last character does not satisfy the condition",
            ))
        }
    }
}
