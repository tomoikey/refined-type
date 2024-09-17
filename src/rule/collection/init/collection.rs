use std::collections::{HashMap, HashSet, VecDeque};

use crate::result::Error;
use crate::rule::Rule;
use crate::rule::{ForAllRule, InitRule};

macro_rules! impl_init {
    ($($t:ty),*) => {
        $(
            impl<RULE> Rule for InitRule<RULE, $t>
            where
                RULE: Rule,
            {
                type Item = $t;

                fn validate(target: &Self::Item) -> Result<(), Error> {
                    let length = target.len();
                    let mut result = Ok(());

                    for (i, item) in target.iter().enumerate() {
                        if i == length - 1 {
                            break;
                        }
                        match RULE::validate(item) {
                            Ok(_) => continue,
                            Err(e) => {
                                result = Err(e);
                                break;
                            }
                        }
                    }
                    result
                }
            }
        )*
    };
}

impl_init![Vec<RULE::Item>, VecDeque<RULE::Item>, HashSet<RULE::Item>];

impl<RULE, K, V> Rule for ForAllRule<RULE, HashMap<K, V>>
where
    RULE: Rule<Item = V>,
{
    type Item = HashMap<K, V>;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.values().all(|item| RULE::validate(item).is_ok()) {
            Ok(())
        } else {
            Err(Error::new("not all items satisfy the condition"))
        }
    }
}
