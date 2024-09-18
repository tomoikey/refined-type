use std::collections::VecDeque;

macro_rules! impl_skip_rule {
    ($($t:ty),*) => {
        $(
            impl<RULE, ITEM, OPTION> $crate::rule::Rule for $crate::rule::SkipRule<RULE, $t, OPTION>
            where
                RULE: $crate::rule::Rule<Item = ITEM>,
                OPTION: $crate::rule::SkipOption<Item = ITEM>,
            {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, crate::result::Error<Self::Item>> {
                    let mut remains = target.into_iter();
                    let mut result = VecDeque::new();
                    let (mut is_valid, mut message) = (true, String::new());
                    for (i, item) in remains.by_ref().enumerate() {
                        if OPTION::should_skip(i, &item) {
                            result.push_back(item);
                            continue;
                        }
                        match RULE::validate(item) {
                            Ok(validated_item) => result.push_back(validated_item),
                            Err(err) => {
                                is_valid = false;
                                message = format!(
                                    "the item at index {} does not satisfy the condition: {}",
                                    i, err
                                );
                                result.push_back(err.into_value());
                            }
                        }
                    }

                    if is_valid {
                        Ok(result.into_iter().collect())
                    } else {
                        result.append(&mut remains.collect::<VecDeque<_>>());
                        Err(crate::result::Error::new(result.into_iter().collect(), message))
                    }
                }
            }
        )*
    };
}

impl_skip_rule![Vec<RULE::Item>, VecDeque<RULE::Item>];
