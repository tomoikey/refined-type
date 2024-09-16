use crate::result::Error;
use crate::rule::{InitRule, Rule};

impl<RULE> Rule for InitRule<RULE, String>
where
    RULE: Rule<Item = char>,
{
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let mut remains = target.chars();
        let mut result = Vec::new();
        let mut failed = false;

        for (i, item) in remains.by_ref().enumerate() {
            if i < target.len() - 1 {
                match RULE::validate(item) {
                    Ok(validated_item) => result.push(validated_item),
                    Err(err) => {
                        result.push(err.into_value());
                        failed = true;
                        break;
                    }
                }
            } else {
                result.push(item);
            }
        }

        if failed {
            result.append(&mut remains.collect::<Vec<_>>());
            let result = result.into_iter().collect::<String>();
            Err(Error::new(result, "Failed to validate all items"))
        } else {
            Ok(result.into_iter().collect::<String>())
        }
    }
}

impl<'a, RULE> Rule for InitRule<RULE, &'a str>
where
    RULE: Rule<Item = char>,
{
    type Item = &'a str;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let length = target.len();
        let mut result = Ok(target);
        for (i, c) in target.chars().enumerate() {
            if i == length - 1 {
                break;
            }
            match RULE::validate(c) {
                Ok(_) => continue,
                Err(_) => {
                    result = Err(Error::new(target, "Failed to validate all items"));
                    break;
                }
            }
        }
        result
    }
}
