use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;
use std::ops::Deref;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// ```
pub struct Not<'a, T, RULE> {
    bounden_rule: Box<dyn Fn(T) -> Result<T, Error<T>> + 'a>,
    _rule: PhantomData<RULE>,
}

impl<'a, T, RULE> Not<'a, T, RULE>
where
    RULE: Rule<Item = T> + 'a,
{
    pub fn new(rule: RULE) -> Self {
        let bounded_rule = move |t: T| match rule.validate(t) {
            Ok(t) => Err(Error::new("Target satisfies the rule", t)),
            Err(e) => Ok(e.target),
        };
        Self {
            bounden_rule: Box::new(bounded_rule),
            _rule: Default::default(),
        }
    }
}

impl<T, RULE> Rule for Not<'_, T, RULE> {
    type Item = T;

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        self.bounden_rule.deref()(target)
    }
}

impl<'a, T, RULE> Default for Not<'a, T, RULE>
where
    RULE: Rule<Item = T> + Default + 'a,
{
    fn default() -> Self {
        Self {
            bounden_rule: Box::new(|t| RULE::default().validate(t)),
            _rule: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::Not;
    use crate::rule::{NonEmptyStringRule, Rule};

    #[test]
    fn test_not() {
        let non_non_empty_string = Not::new(NonEmptyStringRule::default());
        assert!(non_non_empty_string.validate("".to_string()).is_ok());
        assert!(non_non_empty_string.validate("Hello".to_string()).is_err())
    }
}
