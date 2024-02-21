use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
///  use refined_type::rule::{AlphabetRule, NonEmptyStringRule, Rule};
///  use refined_type::rule::composer::And;
///
///  type NonEmptyAlphabetString<'a> = And<NonEmptyStringRule, AlphabetRule>;
///
///  let actual = NonEmptyAlphabetString::validate("Hello".to_string());
///  assert!(actual.is_ok_and(|n| n.as_str() == "Hello"));
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

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let bounded_rule = move |t: T| RULE1::validate(t).and_then(|t| RULE2::validate(t));
        bounded_rule(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::And;
    use crate::rule::{AlphabetRule, NonEmptyStringRule, Rule};

    type NonEmptyAlphabetString = And<NonEmptyStringRule, AlphabetRule>;

    #[test]
    fn test_rule_binder_ok() {
        assert!(NonEmptyAlphabetString::validate("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(NonEmptyAlphabetString::validate("Hello1".to_string()).is_err());
    }
}
