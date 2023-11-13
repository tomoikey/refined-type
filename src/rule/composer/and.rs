use crate::result::Result;
use crate::rule::Rule;
use std::marker::PhantomData;
use std::ops::Deref;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
///  use refined_type::rule::{AlphabetRule, NonEmptyStringRule, Rule};
///  use refined_type::rule::composer::And;
///
///  let non_empty_alphabet_rule = And::new(NonEmptyStringRule, AlphabetRule);
///  let actual = non_empty_alphabet_rule.validate("Hello".to_string());
///  assert!(actual.is_ok_and(|n| n.as_str() == "Hello"));
/// ```
pub struct And<'a, T, RULE1, RULE2> {
    bounden_rule: Box<dyn Fn(T) -> Result<T, T> + 'a>,
    _rule1: PhantomData<RULE1>,
    _rule2: PhantomData<RULE2>,
}

impl<'a, T, RULE1, RULE2> And<'a, T, RULE1, RULE2>
where
    RULE1: Rule<Item = T> + 'a,
    RULE2: Rule<Item = T> + 'a,
{
    pub fn new(rule1: RULE1, rule2: RULE2) -> Self {
        let bounded_rule = move |t: T| rule1.validate(t).and_then(|t| rule2.validate(t));
        Self {
            bounden_rule: Box::new(bounded_rule),
            _rule1: Default::default(),
            _rule2: Default::default(),
        }
    }
}

impl<T, RULE1, RULE2> Rule for And<'_, T, RULE1, RULE2> {
    type Item = T;

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Self::Item> {
        self.bounden_rule.deref()(target)
    }
}

#[cfg(test)]
mod test {
    use crate::result::Result;
    use crate::rule::composer::And;
    use crate::rule::{AlphabetRule, NonEmptyStringRule, Rule};

    #[test]
    fn test_rule_binder_ok() -> Result<(), String> {
        let non_empty_alphabet_rule = And::new(NonEmptyStringRule, AlphabetRule);
        let actual = non_empty_alphabet_rule.validate("Hello".to_string())?;
        assert_eq!(actual, "Hello");
        Ok(())
    }

    #[test]
    fn test_rule_binder_err() -> Result<(), String> {
        let non_empty_alphabet_rule = And::new(NonEmptyStringRule, AlphabetRule);
        let actual = non_empty_alphabet_rule.validate("Hello1".to_string());
        assert!(actual.is_err());
        Ok(())
    }
}
