use crate::rule::composer::Not;
use crate::Or;

/// This is a type that represents logical implication in logic.
/// By applying it to programming, you can make it function similarly to an “If-Then” statement.
/// # Example
/// ```rust
/// use refined_type::rule::composer::Imply;
/// use refined_type::rule::{EvenRuleI8, GreaterEqualRuleI8, Rule};
/// 
/// type IfGreaterOrEqual10ThenEven = Imply<GreaterEqualRuleI8<10>, EvenRuleI8>;
/// 
/// for value in vec![8, 9, 10, 12] {
///    assert!(IfGreaterOrEqual10ThenEven::validate(value).is_ok());
/// }
/// 
/// for value in vec![11, 13] {
///   assert!(IfGreaterOrEqual10ThenEven::validate(value).is_err());
/// }
pub type Imply<RULE1, RULE2> = Or![Not<RULE1>, RULE2];

#[cfg(test)]
mod test {
    use crate::rule::composer::Imply;
    use crate::rule::{EvenRuleI8, GreaterEqualRuleI8, Rule};

    type IfGreaterOrEqual10ThenEven = Imply<GreaterEqualRuleI8<10>, EvenRuleI8>;

    #[test]
    fn test_rule_binder_ok() {
        let table = vec![8, 9, 10, 12];

        for value in table {
            assert!(IfGreaterOrEqual10ThenEven::validate(value).is_ok());
        }
    }

    #[test]
    fn test_rule_binder_err() {
        let table = vec![11, 13];

        for value in table {
            assert!(IfGreaterOrEqual10ThenEven::validate(value).is_err());
        }
    }
}
