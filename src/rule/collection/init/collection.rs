use std::collections::{HashMap, VecDeque};

use crate::result::Error;
use crate::rule::InitRule;
use crate::rule::Rule;

macro_rules! impl_init {
    ($($t:ty),*) => {
        $(
            impl<RULE> Rule for InitRule<RULE, $t>
            where
                RULE: Rule,
            {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
                    let length = target.len();
                    let mut remains = target.into_iter();
                    let mut result = VecDeque::new();
                    let mut failed = false;

                    for (i, item) in remains.by_ref().enumerate() {
                        if i < length - 1 {
                            match RULE::validate(item) {
                                Ok(validated_item) => result.push_back(validated_item),
                                Err(err) => {
                                    result.push_back(err.into_value());
                                    failed = true;
                                    break;
                                }
                            }
                        } else {
                            result.push_back(item);
                        }
                    }

                    if failed {
                        result.append(&mut remains.collect::<VecDeque<_>>());
                        let result = result.into_iter().collect::<$t>();
                        Err(Error::new(
                            result,
                            "Failed to validate all items",
                        ))
                    } else {
                        Ok(result.into_iter().collect::<$t>())
                    }
                }
            }
        )*
    };
}

impl_init![Vec<RULE::Item>, VecDeque<RULE::Item>];

impl<RULE, K, V> Rule for InitRule<RULE, HashMap<K, V>>
where
    RULE: Rule<Item = V>,
    K: Eq + std::hash::Hash,
{
    type Item = HashMap<K, V>;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let length = target.len();
        let mut remains = target.into_iter();
        let mut result = VecDeque::new();
        let mut failed = false;

        for (i, (key, value)) in remains.by_ref().enumerate() {
            if i < length - 1 {
                match RULE::validate(value) {
                    Ok(validated_item) => result.push_back((key, validated_item)),
                    Err(err) => {
                        result.push_back((key, err.into_value()));
                        failed = true;
                        break;
                    }
                }
            } else {
                result.push_back((key, value));
            }
        }

        if failed {
            result.append(&mut remains.collect::<VecDeque<_>>());
            let result = result.into_iter().collect::<HashMap<K, V>>();
            Err(Error::new(result, "Failed to validate all items"))
        } else {
            Ok(result.into_iter().collect::<HashMap<K, V>>())
        }
    }
}
