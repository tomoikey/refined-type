use crate::result::Error;
use crate::rule::collection::skip::option::SkipOption;
use crate::rule::collection::skip::SkipRule;
use crate::rule::Rule;

impl<RULE, OPTION> Rule for SkipRule<RULE, String, OPTION>
where
    RULE: Rule<Item = char>,
    OPTION: SkipOption<Item = char>,
{
    type Item = String;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let chars = target.chars();
        let (mut is_valid, mut message) = (true, String::new());
        for (i, c) in chars.enumerate() {
            if OPTION::should_skip(i, &c) {
                continue;
            } else if let Err(e) = RULE::validate(c) {
                is_valid = false;
                message = format!(
                    "the character at index {} does not satisfy the condition: {}",
                    i, e
                );
            }
        }
        if is_valid {
            Ok(target)
        } else {
            Err(Error::new(target, message))
        }
    }
}
