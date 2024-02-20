mod and;
mod not;
mod or;

pub use and::And;
pub use not::Not;
pub use or::Or;

#[cfg(test)]
mod test {
    // use crate::rule::composer::{And, Not, Or};
    // use crate::rule::{LessI8Rule, MoreI8Rule, Rule};
    //
    // #[test]
    // fn test_and_or_not() {
    //     let less_than_3 = LessI8Rule::new(3);
    //     let more_than_1 = MoreI8Rule::new(1);
    //
    //     // (1 <= x <= 3)
    //     let more_than_1_and_less_than_3 = And::new(less_than_3, more_than_1);
    //
    //     assert!(more_than_1_and_less_than_3.validate(0).is_err());
    //     assert!(more_than_1_and_less_than_3.validate(2).is_ok());
    //     assert!(more_than_1_and_less_than_3.validate(4).is_err());
    //
    //     let more_than_5 = MoreI8Rule::new(5);
    //
    //     // (1 <= x <= 3) or (5 <= x)
    //     let or_more_than_5 = Or::new(more_than_1_and_less_than_3, more_than_5);
    //
    //     assert!(or_more_than_5.validate(0).is_err());
    //     assert!(or_more_than_5.validate(2).is_ok());
    //     assert!(or_more_than_5.validate(4).is_err());
    //     assert!(or_more_than_5.validate(5).is_ok());
    //     assert!(or_more_than_5.validate(100).is_ok());
    //
    //     let more_than_7 = MoreI8Rule::new(7);
    //
    //     // ((1 <= x <= 3) or (5 <= x)) & (x < 7)
    //     let not_more_than_7 = And::new(or_more_than_5, Not::new(more_than_7));
    //
    //     assert!(not_more_than_7.validate(0).is_err());
    //     assert!(not_more_than_7.validate(2).is_ok());
    //     assert!(not_more_than_7.validate(4).is_err());
    //     assert!(not_more_than_7.validate(5).is_ok());
    //     assert!(not_more_than_7.validate(100).is_err());
    // }
}
