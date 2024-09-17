use crate::result::Error;
use crate::rule::{InitRule, Rule};

impl<RULE> Rule for InitRule<RULE, String>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        let length = target.len();
        let mut result = Ok(());

        for (i, c) in target.chars().enumerate() {
            if i == length - 1 {
                break;
            }
            match RULE::validate(&c) {
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

impl<'a, RULE> Rule for InitRule<RULE, &'a str>
where
    RULE: Rule<Item = char>,
{
    type Item = &'a str;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        let length = target.len();
        let mut result = Ok(());

        for (i, c) in target.chars().enumerate() {
            if i == length - 1 {
                break;
            }
            match RULE::validate(&c) {
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
