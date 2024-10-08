use crate::rule::Rule;

#[macro_export]
macro_rules! define_index_refined {
    (($vis:vis, $lit:expr)) => {
        $crate::paste::item! {
            $vis type [<Index $lit>]<RULE, ITERABLE> = $crate::Refined<[<Index $lit Rule>]<RULE, ITERABLE>>;

            $vis type [<Index $lit Vec>]<RULE> = $crate::Refined<[<Index $lit VecRule>]<RULE>>;

            $vis type [<Index $lit VecDeque>]<RULE> = $crate::Refined<[<Index $lit VecDequeRule>]<RULE>>;

            $vis type [<Index $lit String>]<RULE> = $crate::Refined<[<Index $lit StringRule>]<RULE>>;
        }
    };
    ($(($vis:vis, $lit:expr)),*) => {
        $(
            $crate::define_index_refined!(($vis, $lit));
        )*
    };
}

#[macro_export]
macro_rules! define_index_rule {
    (($vis:vis, $lit:expr)) => {
        $crate::paste::item! {
            $vis struct [<Index $lit Rule>]<RULE, ITERABLE>
            where
                RULE: $crate::rule::Rule,
            {
                _phantom_data: ::std::marker::PhantomData<(RULE, ITERABLE)>,
            }

            $vis type [<Index $lit VecRule>]<RULE> = [<Index $lit Rule>]<RULE, Vec<<RULE as Rule>::Item>>;
            impl <RULE, ITEM> $crate::rule::Rule for [<Index $lit Rule>]<RULE, Vec<ITEM>> where RULE: $crate::rule::Rule<Item = ITEM> {
                type Item = Vec<ITEM>;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if $lit >= target.len() {
                        return Err($crate::result::Error::new(target, format!("index {} is out of bounds", $lit)));
                    }
                    let mut target = target;
                    match RULE::validate(target.remove($lit)) {
                        Ok(validated_item) => {
                            let mut result = target;
                            result.insert($lit, validated_item);
                            Ok(result)
                        }
                        Err(err) => {
                            let mut result = target;
                            result.insert($lit, err.into_value());
                            Err($crate::result::Error::new(result, format!("the item at index {} does not satisfy the condition", $lit)))
                        }
                    }
                }
            }

            $vis type [<Index $lit VecDequeRule>]<RULE> = [<Index $lit Rule>]<RULE, ::std::collections::VecDeque<<RULE as Rule>::Item>>;
            impl <RULE, ITEM> $crate::rule::Rule for [<Index $lit Rule>]<RULE, ::std::collections::VecDeque<ITEM>> where RULE: $crate::rule::Rule<Item = ITEM> {
                type Item = ::std::collections::VecDeque<ITEM>;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if $lit >= target.len() {
                        return Err($crate::result::Error::new(target, format!("index {} is out of bounds", $lit)));
                    }
                    let mut target = target;
                    match RULE::validate(target.remove($lit).expect("unreachable")) {
                        Ok(validated_item) => {
                            target.insert($lit, validated_item);
                            Ok(target)
                        }
                        Err(err) => {
                            target.insert($lit, err.into_value());
                            Err($crate::result::Error::new(target, format!("the item at index {} does not satisfy the condition", $lit)))
                        }
                    }
                }
            }

            $vis type [<Index $lit StringRule>]<RULE> = [<Index $lit Rule>]<RULE, String>;
            impl <RULE> $crate::rule::Rule for [<Index $lit Rule>]<RULE, String> where RULE: $crate::rule::Rule<Item = char> {
                type Item = String;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if $lit >= target.len() {
                        return Err($crate::result::Error::new(target, format!("index {} is out of bounds", $lit)));
                    }
                    let mut target = target;
                    match RULE::validate(target.remove($lit)) {
                        Ok(validated_item) => {
                            target.insert($lit, validated_item);
                            Ok(target)
                        }
                        Err(err) => {
                            target.insert($lit, err.into_value());
                            Err($crate::result::Error::new(target, format!("the character at index {} does not satisfy the condition", $lit)))
                        }
                    }
                }
            }

            impl <'a, RULE> $crate::rule::Rule for [<Index $lit Rule>]<RULE, &'a str> where RULE: $crate::rule::Rule<Item = char> {
                type Item = &'a str;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    let item = target.chars().nth($lit).ok_or_else(|| $crate::result::Error::new(target, format!("index {} is out of bounds", $lit)))?;
                    if RULE::validate(item).is_ok() {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(target, format!("the character at index {} does not satisfy the condition", $lit)))
                    }
                }
            }
        }
    };
    ($(($vis:vis, $lit:expr)),*) => {
        $(
            $crate::define_index_rule!(($vis, $lit));
        )*
    };
}

// define index refined type for 0 ~ 10 by default.
// if you want to define additional refined index types, you can add more using `define_index_refined`.
define_index_refined!((pub, 0), (pub, 1), (pub, 2), (pub, 3), (pub, 4), (pub, 5), (pub, 6), (pub, 7), (pub, 8), (pub, 9), (pub, 10));
// define index rules for 0 ~ 10 by default
// if you want to define additional index rules, you can add more using `define_index_rule`.
define_index_rule!((pub, 0), (pub, 1), (pub, 2), (pub, 3), (pub, 4), (pub, 5), (pub, 6), (pub, 7), (pub, 8), (pub, 9), (pub, 10));

#[cfg(test)]
mod tests {
    use crate::rule::{Index0Vec, Index1Vec, Index2Vec, NonEmptyStringRule};

    #[test]
    fn test_index_0_non_empty_string() -> anyhow::Result<()> {
        let table = vec![
            (vec!["good morning".to_string(), "hello".to_string()], true),
            (vec!["good morning".to_string(), "".to_string()], true),
            (vec!["".to_string(), "hello".to_string()], false),
            (vec!["".to_string(), "".to_string()], false),
        ];

        for (value, expected) in table {
            let refined = Index0Vec::<NonEmptyStringRule>::new(value.clone());
            assert_eq!(refined.is_ok(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_index_1_non_empty_string() -> anyhow::Result<()> {
        let table = vec![
            (vec!["good morning".to_string(), "hello".to_string()], true),
            (vec!["good morning".to_string(), "".to_string()], false),
            (vec!["".to_string(), "hello".to_string()], true),
            (vec!["".to_string(), "".to_string()], false),
        ];

        for (value, expected) in table {
            let refined = Index1Vec::<NonEmptyStringRule>::new(value.clone());
            assert_eq!(refined.is_ok(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_index_2_non_empty_string_out_of_bounds() {
        let value = vec!["good morning".to_string(), "hello".to_string()];
        let refined = Index2Vec::<NonEmptyStringRule>::new(value);
        assert!(refined.is_err());
    }
}
