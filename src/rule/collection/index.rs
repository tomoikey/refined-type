use std::collections::VecDeque;

use crate::rule::Rule;

#[macro_export]
macro_rules! define_index_refined {
    ($lit:literal) => {
        $crate::paste::item! {
            pub type [<Index $lit>]<RULE, ITERABLE> = $crate::Refined<[<Index $lit Rule>]<RULE, ITERABLE>>;

            pub type [<Index $lit Vec>]<RULE> = $crate::Refined<[<Index $lit VecRule>]<RULE>>;

            pub type [<Index $lit VecDeque>]<RULE> = $crate::Refined<[<Index $lit VecDequeRule>]<RULE>>;

            pub type [<Index $lit String>]<RULE> = $crate::Refined<[<Index $lit StringRule>]<RULE>>;
        }
    };
    ($lit:literal, $($lits:literal),*) => {
        define_index_refined!($lit);
        define_index_refined!($($lits),*);
    }
}

#[macro_export]
macro_rules! define_index_rule {
    ($lit:literal) => {
        $crate::paste::item! {
            pub struct [<Index $lit Rule>]<RULE, ITERABLE>
            where
                RULE: $crate::rule::Rule,
            {
                _phantom_data: ::std::marker::PhantomData<(RULE, ITERABLE)>,
            }

            pub type [<Index $lit VecRule>]<RULE> = [<Index $lit Rule>]<RULE, Vec<<RULE as Rule>::Item>>;
            impl <RULE, ITEM> $crate::rule::Rule for [<Index $lit Rule>]<RULE, Vec<ITEM>> where RULE: $crate::rule::Rule<Item = ITEM> {
                type Item = Vec<ITEM>;

                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    let item = target.get($lit).ok_or_else(|| $crate::result::Error::new(format!("index {} is out of bounds", $lit)))?;
                    if RULE::validate(item).is_ok() {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("the item at index {} does not satisfy the condition", $lit)))
                    }
                }
            }

            pub type [<Index $lit VecDequeRule>]<RULE> = [<Index $lit Rule>]<RULE, VecDeque<<RULE as Rule>::Item>>;
            impl <RULE, ITEM> $crate::rule::Rule for [<Index $lit Rule>]<RULE, ::std::collections::VecDeque<ITEM>> where RULE: $crate::rule::Rule<Item = ITEM> {
                type Item = ::std::collections::VecDeque<ITEM>;

                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    let item = target.get($lit).ok_or_else(|| $crate::result::Error::new(format!("index {} is out of bounds", $lit)))?;
                    if RULE::validate(item).is_ok() {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("the item at index {} does not satisfy the condition", $lit)))
                    }
                }
            }

            pub type [<Index $lit StringRule>]<RULE> = [<Index $lit Rule>]<RULE, String>;
            impl <RULE> $crate::rule::Rule for [<Index $lit Rule>]<RULE, String> where RULE: $crate::rule::Rule<Item = char> {
                type Item = String;

                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    let item = target.chars().nth($lit).ok_or_else(|| $crate::result::Error::new(format!("index {} is out of bounds", $lit)))?;
                    if RULE::validate(&item).is_ok() {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("the character at index {} does not satisfy the condition", $lit)))
                    }
                }
            }

            impl <'a, RULE> $crate::rule::Rule for [<Index $lit Rule>]<RULE, &'a str> where RULE: $crate::rule::Rule<Item = char> {
                type Item = &'a str;

                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    let item = target.chars().nth($lit).ok_or_else(|| $crate::result::Error::new(format!("index {} is out of bounds", $lit)))?;
                    if RULE::validate(&item).is_ok() {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("the character at index {} does not satisfy the condition", $lit)))
                    }
                }
            }
        }
    };
    ($lit:literal, $($lits:literal),*) => {
        define_index_rule!($lit);
        define_index_rule!($($lits),*);
    }
}

// define index refined type for 0 ~ 10 by default.
// if you want to define additional refined index types, you can add more using `define_index_refined`.
define_index_refined!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

// define index rules for 0 ~ 10 by default
// if you want to define additional index rules, you can add more using `define_index_rule`.
define_index_rule!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

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
