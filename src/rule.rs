mod number;
mod string;

use crate::error::Result;
use std::marker::PhantomData;
use std::ops::Deref;

pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type TARGET;
    fn validate(&self, target: Self::TARGET) -> Result<Self::TARGET>;
}

pub struct RuleBinder<'a, T, Rule1, Rule2> {
    bounden_rule: Box<dyn Fn(T) -> Result<T> + 'a>,
    _rule1: PhantomData<Rule1>,
    _rule2: PhantomData<Rule2>,
}

impl<'a, T, Rule1, Rule2> RuleBinder<'a, T, Rule1, Rule2>
where
    Rule1: Rule<TARGET = T> + 'a,
    Rule2: Rule<TARGET = T> + 'a,
{
    pub fn bind(rule1: Rule1, rule2: Rule2) -> Self {
        let bounded_rule = move |t: T| rule1.validate(t).and_then(|t| rule2.validate(t));
        Self {
            bounden_rule: Box::new(bounded_rule),
            _rule1: Default::default(),
            _rule2: Default::default(),
        }
    }
}

impl<T, Rule1, Rule2> Rule for RuleBinder<'_, T, Rule1, Rule2> {
    type TARGET = T;

    fn validate(&self, target: Self::TARGET) -> Result<Self::TARGET> {
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
