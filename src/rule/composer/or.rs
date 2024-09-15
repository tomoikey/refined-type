use std::marker::PhantomData;

use crate::result::Error;
use crate::rule::Rule;

/// A macro to generate a `Rule` that combines multiple rules
/// # Example
/// ```rust
/// use refined_type::rule::{NonEmptyStringRule, Rule, EmailRule};
/// use refined_type::O;
///
/// type NewRule = O![EmailRule<String>, NonEmptyStringRule, EmailRule<String>];
///
/// let actual = NewRule::validate("sample@example.com".to_string());
/// assert!(actual.is_ok());
#[macro_export]
macro_rules! O {
    ($rule1:ty, $rule2:ty) => {
        $crate::rule::composer::Or<$rule1, $rule2>
    };
    ($rule1:ty, $($rule2: ty), +) => {
        $crate::rule::composer::Or<$rule1, O![$($rule2), +]>
    }
}

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// use refined_type::rule::composer::Or;
/// use refined_type::rule::{AlphabetRule, EmptyRule, Rule};
///
/// type EmptyOrAlphabetString = Or<EmptyRule<String>, AlphabetRule<String>>;
///
/// assert!(EmptyOrAlphabetString::validate("".to_string()).is_ok());
/// assert!(EmptyOrAlphabetString::validate("alphabet".to_string()).is_ok());
/// assert!(EmptyOrAlphabetString::validate("1".to_string()).is_err());
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

    fn validate(target: Self::Item) -> Result<T, Error<T>> {
        let bounded_rule = |t: T| match RULE1::validate(t) {
            Ok(value) => Ok(value),
            Err(err) => RULE2::validate(err.into_value()),
        };
        bounded_rule(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::Or;
    use crate::rule::{AlphabetRule, EmailRule, NonEmptyStringRule, Rule};

    #[test]
    fn test_or() {
        type NonEmptyOrAlphabetString = Or<NonEmptyStringRule, AlphabetRule<String>>;
        assert!(NonEmptyOrAlphabetString::validate("hello".to_string()).is_ok());
        assert!(NonEmptyOrAlphabetString::validate("12345".to_string()).is_ok());
        assert!(NonEmptyOrAlphabetString::validate("".to_string()).is_ok());
    }

    #[test]
    fn test_rule_binder_macro_ok() {
        type SampleRule = O![EmailRule<String>, NonEmptyStringRule, EmailRule<String>];
        assert!(SampleRule::validate("hoge".to_string()).is_ok());
    }

    #[test]
    fn test_rule_binder_macro_err() {
        type SampleRule = O![EmailRule<String>, NonEmptyStringRule, EmailRule<String>];
        assert!(SampleRule::validate("".to_string()).is_err());
    }
}
