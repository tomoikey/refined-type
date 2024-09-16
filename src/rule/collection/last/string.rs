use crate::rule::{LastRule, Rule};

impl<RULE> Rule for LastRule<RULE, String>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        match target.chars().last() {
            Some(item) => match RULE::validate(item) {
                Ok(_) => Ok(target),
                Err(_) => Err(crate::result::Error::new(
                    target,
                    "Failed to validate the last item",
                )),
            },
            None => Err(crate::result::Error::new(
                target,
                "Last item does not exist",
            )),
        }
    }
}

impl<'a, RULE> Rule for LastRule<RULE, &'a str>
where
    RULE: Rule<Item = char>,
{
    type Item = &'a str;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        match target.chars().last() {
            Some(item) => match RULE::validate(item) {
                Ok(_) => Ok(target),
                Err(_) => Err(crate::result::Error::new(
                    target,
                    "Failed to validate the last item",
                )),
            },
            None => Err(crate::result::Error::new(
                target,
                "Last item does not exist",
            )),
        }
    }
}
