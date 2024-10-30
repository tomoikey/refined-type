use crate::result::Error;
use crate::rule::Rule;
use std::marker::PhantomData;

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
        match RULE1::validate(target) {
            Ok(value) => RULE2::validate(value),
            Err(err) => {
                let rule1_error_message = err.to_string();
                let rule1_type_name = std::any::type_name::<RULE1>();
                match RULE2::validate(err.into_value()) {
                    Ok(value) => {
                        let message = format!("{rule1_error_message} ({rule1_type_name})");
                        Err(Error::new(value, message))
                    }
                    Err(err) => {
                        let rule2_type_name = std::any::type_name::<RULE2>();
                        let message = format!(
                            "{rule1_error_message} ({rule1_type_name}) & {err} ({rule2_type_name})",
                        );
                        Err(Error::new(err.into_value(), message))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::composer::And;
    use crate::rule::{AlphabetRule, EmailRule, EvenRuleU8, LessRuleU8, NonEmptyStringRule, Rule};

    type NonEmptyAlphabetString = And<NonEmptyStringRule, AlphabetRule<String>>;

    #[test]
    fn test_rule_binder_ok() {
        assert!(NonEmptyAlphabetString::validate("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        type Target = And![EvenRuleU8, LessRuleU8<10>];
        assert_eq!(Target::validate(11).unwrap_err().to_string(), "the value must be even, but received 11 (refined_type::rule::number::even::EvenRuleU8) & the value must be less than 10, but received 11 (refined_type::rule::number::less::LessRuleU8<10>)");
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
