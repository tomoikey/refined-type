use std::collections::{HashSet, VecDeque};

use crate::result::Error;
use crate::rule::ForAllRule;
use crate::rule::Rule;

macro_rules! impl_for_all {
    ($($t:ty),*) => {
        $(
            impl<RULE> Rule for ForAllRule<RULE, $t>
            where
                RULE: Rule,
            {
                type Item = $t;

                fn validate(target: &Self::Item) -> Result<(), Error> {
                    if target.iter().all(|item| RULE::validate(item).is_ok()) {
                        Ok(())
                    } else {
                        Err(Error::new("not all items satisfy the condition"))
                    }
                }
            }
        )*
    };
}

impl_for_all![Vec<RULE::Item>, VecDeque<RULE::Item>, HashSet<RULE::Item>];
