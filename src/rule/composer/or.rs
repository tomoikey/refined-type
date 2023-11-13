use crate::result::Result;
use crate::rule::Rule;
use std::marker::PhantomData;
use std::ops::Deref;

/// A binder that combines two rules to generate a new single `Rule`
/// # Example
/// ```rust
/// ```
pub struct Or<'a, T, RULE1, RULE2> {
    bounden_rule: Box<dyn Fn(T) -> Result<T, T> + 'a>,
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

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Self::Item> {
        self.bounden_rule.deref()(target)
    }
}
