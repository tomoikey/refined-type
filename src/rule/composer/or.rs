use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// //use refined_type::rule::{LessI8Rule, MoreI8Rule, Rule};
/// use refined_type::rule::composer::Or;
///
/// //let less_than_1 = LessI8Rule::new(1);
/// //let more_than_5 = MoreI8Rule::new(5);
/// //let rule = Or::new(less_than_1, more_than_5);
///
/// //assert!(rule.validate(0).is_ok());
/// //assert!(rule.validate(1).is_ok());
/// //assert!(rule.validate(2).is_err());
/// //assert!(rule.validate(4).is_err());
/// //assert!(rule.validate(5).is_ok());
/// //assert!(rule.validate(6).is_ok());
/// ```
pub struct Or<RULE1, RULE2> {
    _rule1: PhantomData<RULE1>,
    _rule2: PhantomData<RULE2>,
}

impl<'a, T, RULE1, RULE2> Rule for Or<RULE1, RULE2>
where
    RULE1: Rule<Item = T> + 'a,
    RULE2: Rule<Item = T> + 'a,
{
    type Item = T;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let bounded_rule = move |t: T| RULE1::validate(t).or_else(|e| RULE2::validate(e.target));
        bounded_rule(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::Or;
    use crate::rule::{AlphabetRule, NonEmptyStringRule, Rule};

    #[test]
    fn test_or() {
        type NonEmptyOrAlphabetString<'a> = Or<NonEmptyStringRule<'a>, AlphabetRule>;
        assert!(NonEmptyOrAlphabetString::validate("hello".to_string()).is_ok());
        assert!(NonEmptyOrAlphabetString::validate("12345".to_string()).is_ok());
        assert!(NonEmptyOrAlphabetString::validate("".to_string()).is_ok());
    }
}
