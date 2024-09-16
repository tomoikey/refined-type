use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;

/// `Not` reverses the definition of a certain `Rule`.
/// # Example
/// ```rust
/// use refined_type::rule::composer::Not;
/// use refined_type::rule::{EmptyRule, Rule};
///
/// type NonEmptyString = Not<EmptyRule<String>>;
///
/// assert!(NonEmptyString::validate("non empty".to_string()).is_ok());
/// assert!(NonEmptyString::validate("".to_string()).is_err());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Not<RULE> {
    _rule: PhantomData<RULE>,
}

impl<'a, T, RULE> Rule for Not<RULE>
where
    RULE: Rule<Item = T> + 'a,
{
    type Item = T;

    fn validate(target: Self::Item) -> crate::Result<T> {
        let bounded_rule = |t: T| match RULE::validate(t) {
            Ok(value) => Err(Error::new(value, "Target satisfies the `Not` rule")),
            Err(err) => Ok(err.into_value()),
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
