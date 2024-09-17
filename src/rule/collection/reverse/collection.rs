use std::collections::VecDeque;

use crate::result::Error;
use crate::rule::collection::reverse::ReverseRule;
use crate::rule::Rule;

macro_rules! impl_reverse {
    ($($t:ty),*) => {
        $(
            impl<RULE, ITEM> Rule for ReverseRule<RULE, $t>
            where
                RULE: Rule<Item = $t>,
            {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
                    match RULE::validate(target.into_iter().rev().collect()) {
                        Ok(value) => Ok(value.into_iter().rev().collect()),
                        Err(e) => {
                            let message = e.to_string();
                            Err(Error::new(e.into_value().into_iter().rev().collect(), message))
                        },
                    }
                }
            }
        )*
    };
    () => {};
}

impl_reverse![Vec<ITEM>, VecDeque<ITEM>];
