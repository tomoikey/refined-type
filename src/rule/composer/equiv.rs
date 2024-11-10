use crate::rule::composer::imply::Imply;
use crate::And;

/// This is a type that represents logical equivalence in logic.
///
/// # Example
/// ```rust
/// use refined_type::rule::composer::Equiv;
/// use refined_type::rule::{EvenRuleI8, GreaterEqualRuleI8, Rule};
///
/// type Target = Equiv<GreaterEqualRuleI8<10>, EvenRuleI8>;
///
/// for value in vec![1, 10] {
///    assert!(Target::validate(value).is_ok());
/// }
///     
/// for value in vec![2, 4] {
///    assert!(Target::validate(value).is_err());
/// }
/// ```
pub type Equiv<RULE1, RULE2> = And![Imply<RULE1, RULE2>, Imply<RULE2, RULE1>];

#[cfg(test)]
mod test {
    use crate::rule::composer::Equiv;
    use crate::rule::{EvenRuleI8, GreaterEqualRuleI8, Rule};

    type Target = Equiv<GreaterEqualRuleI8<10>, EvenRuleI8>;

    #[test]
    fn test_rule_binder_ok() {
        let table = vec![1, 10];

        for value in table {
            assert!(Target::validate(value).is_ok());
        }
    }

    #[test]
    fn test_rule_binder_err() {
        let table = vec![2, 4];

        for value in table {
            assert!(Target::validate(value).is_err());
        }
    }
}
