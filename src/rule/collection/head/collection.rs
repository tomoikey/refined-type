use crate::result::Error;
use crate::rule::collection::head::HeadRule;
use crate::rule::Rule;
use std::collections::VecDeque;

impl<RULE, ITEM> Rule for HeadRule<RULE, Vec<ITEM>>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = Vec<ITEM>;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if let Some(head) = target.first() {
            RULE::validate(head)
        } else {
            Err(Error::new("empty collection"))
        }
    }
}

impl<RULE, ITEM> Rule for HeadRule<RULE, VecDeque<ITEM>>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = VecDeque<ITEM>;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if let Some(head) = target.front() {
            RULE::validate(head)
        } else {
            Err(Error::new("empty collection"))
        }
    }
}
