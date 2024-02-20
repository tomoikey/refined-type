use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// use refined_type::rule::composer::Not;
/// use refined_type::rule::{Empty, Rule};
///
/// type NonEmptyString = Not<Empty<String>>;
///
/// assert!(NonEmptyString::validate("non empty".to_string()).is_ok());
/// assert!(NonEmptyString::validate("".to_string()).is_err());
/// ```
pub struct Not<RULE> {
    _rule: PhantomData<RULE>,
}

impl<'a, T, RULE> Rule for Not<RULE>
where
    RULE: Rule<Item = T> + 'a,
{
    type Item = T;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let bounded_rule = move |t: T| match RULE::validate(t) {
            Ok(t) => Err(Error::new("Target satisfies the rule", t)),
            Err(e) => Ok(e.into_target()),
        };
        bounded_rule(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::Not;
    use crate::rule::{NonEmptyStringRule, Rule};

    #[test]
    fn test_not() {
        type NonNonEmptyString = Not<NonEmptyStringRule>;
        assert!(NonNonEmptyString::validate("".to_string()).is_ok());
        assert!(NonNonEmptyString::validate("Hello".to_string()).is_err())
    }
}
