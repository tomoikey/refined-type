use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// use refined_type::rule::composer::Or;
/// use refined_type::rule::{AlphabetRule, EmptyRule, Rule};
///
/// type EmptyOrAlphabetString = Or<EmptyRule<String>, AlphabetRule>;
///
/// assert!(EmptyOrAlphabetString::validate(&"".to_string()).is_ok());
/// assert!(EmptyOrAlphabetString::validate(&"alphabet".to_string()).is_ok());
/// assert!(EmptyOrAlphabetString::validate(&"1".to_string()).is_err());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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

    fn validate(target: &Self::Item) -> Result<(), Error> {
        let bounded_rule =
            |t: &T| RULE1::validate(t).or_else(|_| RULE2::validate(t));
        bounded_rule(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::Or;
    use crate::rule::{AlphabetRule, NonEmptyStringRule, Rule};

    #[test]
    fn test_or() {
        type NonEmptyOrAlphabetString = Or<NonEmptyStringRule, AlphabetRule>;
        assert!(NonEmptyOrAlphabetString::validate(&"hello".to_string()).is_ok());
        assert!(NonEmptyOrAlphabetString::validate(&"12345".to_string()).is_ok());
        assert!(NonEmptyOrAlphabetString::validate(&"".to_string()).is_ok());
    }
}
