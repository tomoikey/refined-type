mod number;
mod string;

use crate::error::Result;
use std::marker::PhantomData;
use std::ops::Deref;

pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(&self, target: Self::Item) -> Result<Self::Item>;
}

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
///  use refined_type::{AlphabetRule, NonEmptyStringRule, Rule, RuleBinder};
///
///  let non_empty_alphabet_rule = RuleBinder::bind(NonEmptyStringRule, AlphabetRule);
///  let actual = non_empty_alphabet_rule.validate("Hello".to_string())?;
///  assert_eq!(actual, "Hello");
/// ```
pub struct RuleBinder<'a, T, RULE1, RULE2> {
    bounden_rule: Box<dyn Fn(T) -> Result<T> + 'a>,
    _rule1: PhantomData<RULE1>,
    _rule2: PhantomData<RULE2>,
}

impl<'a, T, RULE1, RULE2> RuleBinder<'a, T, RULE1, RULE2>
where
    RULE1: Rule<Item = T> + 'a,
    RULE2: Rule<Item = T> + 'a,
{
    pub fn bind(rule1: RULE1, rule2: RULE2) -> Self {
        let bounded_rule = move |t: T| rule1.validate(t).and_then(|t| rule2.validate(t));
        Self {
            bounden_rule: Box::new(bounded_rule),
            _rule1: Default::default(),
            _rule2: Default::default(),
        }
    }
}

impl<T, RULE1, RULE2> Rule for RuleBinder<'_, T, RULE1, RULE2> {
    type Item = T;

    fn validate(&self, target: Self::Item) -> Result<Self::Item> {
        self.bounden_rule.deref()(target)
    }
}

#[cfg(test)]
mod test {
    use crate::error::Result;
    use crate::{AlphabetRule, NonEmptyStringRule, Rule, RuleBinder};

    #[test]
    fn test_rule_binder_ok() -> Result<()> {
        let non_empty_alphabet_rule = RuleBinder::bind(NonEmptyStringRule, AlphabetRule);
        let actual = non_empty_alphabet_rule.validate("Hello".to_string())?;
        assert_eq!(actual, "Hello");
        Ok(())
    }

    #[test]
    fn test_rule_binder_err() -> Result<()> {
        let non_empty_alphabet_rule = RuleBinder::bind(NonEmptyStringRule, AlphabetRule);
        let actual = non_empty_alphabet_rule.validate("Hello1".to_string());
        assert!(actual.is_err());
        Ok(())
    }
}
