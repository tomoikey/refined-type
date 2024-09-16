use crate::rule::{LastRule, Rule};
use std::collections::VecDeque;

impl<RULE, ITEM> Rule for LastRule<RULE, Vec<ITEM>>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = Vec<ITEM>;

    fn validate(target: &Self::Item) -> Result<(), crate::result::Error> {
        let item = target
            .last()
            .ok_or_else(|| crate::result::Error::new("the vector is empty"))?;
        if RULE::validate(item).is_ok() {
            Ok(())
        } else {
            Err(crate::result::Error::new(
                "the last item does not satisfy the condition",
            ))
        }
    }
}

impl<RULE, ITEM> Rule for LastRule<RULE, VecDeque<ITEM>>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = VecDeque<ITEM>;

    fn validate(target: &Self::Item) -> Result<(), crate::result::Error> {
        let item = target
            .back()
            .ok_or_else(|| crate::result::Error::new("the deque is empty"))?;
        if RULE::validate(item).is_ok() {
            Ok(())
        } else {
            Err(crate::result::Error::new(
                "the last item does not satisfy the condition",
            ))
        }
    }
}
