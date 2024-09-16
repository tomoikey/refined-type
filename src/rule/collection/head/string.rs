use crate::result::Error;
use crate::rule::collection::head::HeadRule;
use crate::rule::Rule;

impl<RULE> Rule for HeadRule<RULE, String>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if let Some(head) = target.chars().next() {
            RULE::validate(&head)
        } else {
            Err(Error::new("empty string"))
        }
    }
}

impl<'a, RULE> Rule for HeadRule<RULE, &'a str>
where
    RULE: Rule<Item = char>,
{
    type Item = &'a str;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if let Some(head) = target.chars().next() {
            RULE::validate(&head)
        } else {
            Err(Error::new("empty string"))
        }
    }
}
