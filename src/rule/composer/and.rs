use std::marker::PhantomData;

use crate::rule::Rule;

/// A macro to generate a `Rule` that combines multiple rules
/// # Example
/// ```rust
/// use refined_type::rule::{NonEmptyStringRule, Rule, EmailRule};
/// use refined_type::And;
///
/// type NonEmptyAlphabetString = And![EmailRule<String>, NonEmptyStringRule, EmailRule<String>];
///
/// let actual = NonEmptyAlphabetString::validate("sample@example.com".to_string());
/// assert!(actual.is_ok());
/// ```
#[macro_export]
macro_rules! And {
    ($rule1:ty, $rule2:ty) => {
        $crate::rule::composer::And<$rule1, $rule2>
    };
    ($rule1:ty, $($rule2: ty), +) => {
        $crate::rule::composer::And<$rule1, And![$($rule2), +]>
    }
}

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
///  use refined_type::rule::{AlphabetRule, NonEmptyStringRule, Rule};
///  use refined_type::rule::composer::And;
///
///  type NonEmptyAlphabetString<'a> = And<NonEmptyStringRule, AlphabetRule<String>>;
///
///  let actual = NonEmptyAlphabetString::validate("Hello".to_string());
///  assert!(actual.is_ok());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct And<RULE1, RULE2> {
    _rule1: PhantomData<RULE1>,
    _rule2: PhantomData<RULE2>,
}

impl<'a, T, RULE1, RULE2> And<RULE1, RULE2>
where
    RULE1: Rule<Item = T> + 'a,
    RULE2: Rule<Item = T> + 'a,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<RULE1, RULE2> Default for And<RULE1, RULE2> {
    fn default() -> Self {
        Self {
            _rule1: Default::default(),
            _rule2: Default::default(),
        }
    }
}

impl<'a, T, RULE1, RULE2> Rule for And<RULE1, RULE2>
where
    RULE1: Rule<Item = T> + 'a,
    RULE2: Rule<Item = T> + 'a,
{
    type Item = T;

    fn validate(target: Self::Item) -> crate::Result<T> {
        let bounded_rule = |t: T| RULE1::validate(t).and_then(RULE2::validate);
        bounded_rule(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::And;
    use crate::rule::{AlphabetRule, EmailRule, NonEmptyStringRule, Rule};

    type NonEmptyAlphabetString = And<NonEmptyStringRule, AlphabetRule<String>>;

    #[test]
    fn test_rule_binder_ok() {
        assert!(NonEmptyAlphabetString::validate("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(NonEmptyAlphabetString::validate("Hello1".to_string()).is_err());
    }

    #[test]
    fn test_rule_binder_macro_ok() {
        type SampleRule = And![EmailRule<String>, NonEmptyStringRule, EmailRule<String>];
        assert!(SampleRule::validate("sample@example.com".to_string()).is_ok());
    }

    #[test]
    fn test_rule_binder_macro_err() {
        type SampleRule = And![AlphabetRule<String>, NonEmptyStringRule, EmailRule<String>];
        assert!(SampleRule::validate("Hello".to_string()).is_err());
    }
}
