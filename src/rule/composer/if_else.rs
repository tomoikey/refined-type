use crate::rule::composer::Not;
use crate::{And, Or};

/// This is a type that represents logical if-else in logic.
/// # Example
/// ```rust
/// use refined_type::rule::composer::IfElse;
///
/// use refined_type::rule::{EvenRuleI8, GreaterEqualRuleI8, OddRuleI8, Rule};
///
/// type Target = IfElse<GreaterEqualRuleI8<10>, EvenRuleI8, OddRuleI8>;
///
/// for value in vec![1, 10] {
///   assert!(Target::validate(value).is_ok());
/// }
///
/// for value in vec![2, 11] {
///  assert!(Target::validate(value).is_err());
/// }
/// ```
pub type IfElse<CONDITION, THEN, ELSE> = Or![And![CONDITION, THEN], And![Not<CONDITION>, ELSE]];

#[cfg(test)]
mod test {
    use crate::rule::composer::IfElse;
    use crate::rule::{EvenRuleI8, GreaterEqualRuleI8, OddRuleI8, Rule};

    type Target = IfElse<GreaterEqualRuleI8<10>, EvenRuleI8, OddRuleI8>;

    #[test]
    fn test_rule_binder_ok() {
        let table = vec![1, 10];

        for value in table {
            assert!(Target::validate(value).is_ok());
        }
    }

    #[test]
    fn test_rule_binder_err() {
        let table = vec![2, 11];

        for value in table {
            assert!(Target::validate(value).is_err());
        }
    }
}
