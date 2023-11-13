use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;
use std::ops::Deref;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// use refined_type::rule::{LessI8Rule, MoreI8Rule, Rule};
/// use refined_type::rule::composer::Or;
///
/// let less_than_1 = LessI8Rule::new(1);
/// let more_than_5 = MoreI8Rule::new(5);
/// let rule = Or::new(less_than_1, more_than_5);
///
/// assert!(rule.validate(0).is_ok());
/// assert!(rule.validate(1).is_ok());
/// assert!(rule.validate(2).is_err());
/// assert!(rule.validate(4).is_err());
/// assert!(rule.validate(5).is_ok());
/// assert!(rule.validate(6).is_ok());
/// ```
pub struct Or<'a, T, RULE1, RULE2> {
    bounden_rule: Box<dyn Fn(T) -> Result<T, Error<T>> + 'a>,
    _rule1: PhantomData<RULE1>,
    _rule2: PhantomData<RULE2>,
}

impl<'a, T, RULE1, RULE2> Or<'a, T, RULE1, RULE2>
where
    RULE1: Rule<Item = T> + 'a,
    RULE2: Rule<Item = T> + 'a,
{
    pub fn new(rule1: RULE1, rule2: RULE2) -> Self {
        let bounded_rule = move |t: T| rule1.validate(t).or_else(|e| rule2.validate(e.target));
        Self {
            bounden_rule: Box::new(bounded_rule),
            _rule1: Default::default(),
            _rule2: Default::default(),
        }
    }
}

impl<T, RULE1, RULE2> Rule for Or<'_, T, RULE1, RULE2> {
    type Item = T;

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        self.bounden_rule.deref()(target)
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::Or;
    use crate::rule::{AlphabetRule, NonEmptyStringRule, Rule};

    #[test]
    fn test_or() {
        let alphabet_or_non_empty = Or::new(NonEmptyStringRule::default(), AlphabetRule);
        assert!(alphabet_or_non_empty.validate("hello".to_string()).is_ok());
        assert!(alphabet_or_non_empty.validate("12345".to_string()).is_ok());
        assert!(alphabet_or_non_empty.validate("".to_string()).is_ok());
    }
}
