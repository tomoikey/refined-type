use crate::rule::{LastRule, Rule};
use std::collections::VecDeque;

impl<RULE, ITEM> Rule for LastRule<RULE, Vec<ITEM>>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = Vec<ITEM>;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        let mut target = target.into_iter().collect::<VecDeque<_>>();
        let last = target.pop_back();
        match last {
            Some(item) => match RULE::validate(item) {
                Ok(validated_item) => {
                    target.push_back(validated_item);
                    Ok(target.into_iter().collect())
                }
                Err(e) => {
                    target.push_back(e.into_value());
                    Err(crate::result::Error::new(
                        target.into_iter().collect(),
                        "Failed to validate the last item",
                    ))
                }
            },
            None => Err(crate::result::Error::new(
                target.into_iter().collect(),
                "Last item does not exist",
            )),
        }
    }
}

impl<RULE, ITEM> Rule for LastRule<RULE, VecDeque<ITEM>>
where
    RULE: Rule<Item = ITEM>,
{
    type Item = VecDeque<ITEM>;

    fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
        let mut target = target;
        let last = target.pop_back();
        match last {
            Some(item) => match RULE::validate(item) {
                Ok(validated_item) => {
                    target.push_back(validated_item);
                    Ok(target)
                }
                Err(err) => {
                    target.push_back(err.into_value());
                    Err(crate::result::Error::new(
                        target,
                        "Failed to validate the last item",
                    ))
                }
            },
            None => Err(crate::result::Error::new(
                target,
                "Last item does not exist",
            )),
        }
    }
}
