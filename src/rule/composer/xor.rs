use crate::rule::composer::Not;
use crate::{And, Or};

/// This is a type that represents logical exclusive disjunction in logic.
/// # Example
/// ```rust
/// use refined_type::rule::composer::Xor;
/// use refined_type::rule::{Invalid, Rule, Valid};use refined_type::Xor;
///
/// type ValidI8 = Valid<i8>;
/// type InvalidI8 = Invalid<i8>;
///
/// type Target1 = Xor![ValidI8, ValidI8]; // 2: ERR
/// type Target2 = Xor![ValidI8, InvalidI8]; // 1: PASS
/// type Target3 = Xor![InvalidI8, ValidI8]; // 1: PASS
/// type Target4 = Xor![InvalidI8, InvalidI8]; // 0: ERR
///
/// assert!(Target2::validate(0).is_ok());
/// assert!(Target3::validate(0).is_ok());
///
/// assert!(Target1::validate(0).is_err());
/// assert!(Target4::validate(0).is_err());
/// ```
pub type Xor<RULE1, RULE2> = Or![And![RULE1, Not<RULE2>], And![Not<RULE1>, RULE2]];

#[macro_export]
macro_rules! Xor {
    ($rule1:ty, $rule2:ty) => {
        $crate::rule::composer::Xor<$rule1, $rule2>
    };

    ($rule1:ty, $($rule2: ty), +) => {
        $crate::rule::composer::Xor<$rule1, $crate::Xor![$($rule2), +]>
    }
}

#[cfg(test)]
mod test_2 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Xor![ValidI8, ValidI8]; // 2: ERR
    type Target2 = Xor![ValidI8, InvalidI8]; // 1: PASS
    type Target3 = Xor![InvalidI8, ValidI8]; // 1: PASS
    type Target4 = Xor![InvalidI8, InvalidI8]; // 0: ERR

    #[test]
    fn test_rule_binder_ok() {
        assert!(Target2::validate(0).is_ok());
        assert!(Target3::validate(0).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(Target1::validate(0).is_err());
        assert!(Target4::validate(0).is_err());
    }
}

#[cfg(test)]
mod test_3 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Xor![ValidI8, ValidI8, ValidI8]; // 3: PASS
    type Target2 = Xor![ValidI8, ValidI8, InvalidI8]; // 2: ERR
    type Target3 = Xor![ValidI8, InvalidI8, ValidI8]; // 2: ERR
    type Target4 = Xor![ValidI8, InvalidI8, InvalidI8]; // 1: PASS
    type Target5 = Xor![InvalidI8, ValidI8, ValidI8]; // 2: ERR
    type Target6 = Xor![InvalidI8, ValidI8, InvalidI8]; // 1: PASS
    type Target7 = Xor![InvalidI8, InvalidI8, ValidI8]; // 1: PASS
    type Target8 = Xor![InvalidI8, InvalidI8, InvalidI8]; // 0: ERR

    #[test]
    fn test_rule_binder_ok() {
        assert!(Target1::validate(0).is_ok());
        assert!(Target4::validate(0).is_ok());
        assert!(Target6::validate(0).is_ok());
        assert!(Target7::validate(0).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(Target2::validate(0).is_err());
        assert!(Target3::validate(0).is_err());
        assert!(Target5::validate(0).is_err());
        assert!(Target8::validate(0).is_err());
    }
}

#[cfg(test)]
mod test_4 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Xor![ValidI8, ValidI8, ValidI8, ValidI8]; // 4: ERR
    type Target2 = Xor![ValidI8, ValidI8, ValidI8, InvalidI8]; // 3: PASS
    type Target3 = Xor![ValidI8, ValidI8, InvalidI8, ValidI8]; // 3: PASS
    type Target4 = Xor![ValidI8, ValidI8, InvalidI8, InvalidI8]; // 2: ERR
    type Target5 = Xor![ValidI8, InvalidI8, ValidI8, ValidI8]; // 3: PASS
    type Target6 = Xor![ValidI8, InvalidI8, ValidI8, InvalidI8]; // 2: ERR
    type Target7 = Xor![ValidI8, InvalidI8, InvalidI8, ValidI8]; // 2: ERR
    type Target8 = Xor![ValidI8, InvalidI8, InvalidI8, InvalidI8]; // 1: PASS
    type Target9 = Xor![InvalidI8, ValidI8, ValidI8, ValidI8]; // 3: PASS
    type Target10 = Xor![InvalidI8, ValidI8, ValidI8, InvalidI8]; // 2: ERR
    type Target11 = Xor![InvalidI8, ValidI8, InvalidI8, ValidI8]; // 2: ERR
    type Target12 = Xor![InvalidI8, ValidI8, InvalidI8, InvalidI8]; // 1: PASS
    type Target13 = Xor![InvalidI8, InvalidI8, ValidI8, ValidI8]; // 2: ERR
    type Target14 = Xor![InvalidI8, InvalidI8, ValidI8, InvalidI8]; // 1: PASS
    type Target15 = Xor![InvalidI8, InvalidI8, InvalidI8, ValidI8]; // 1: PASS
    type Target16 = Xor![InvalidI8, InvalidI8, InvalidI8, InvalidI8]; // 0: ERR

    #[test]
    fn test_rule_binder_ok() {
        assert!(Target2::validate(0).is_ok());
        assert!(Target3::validate(0).is_ok());
        assert!(Target5::validate(0).is_ok());
        assert!(Target8::validate(0).is_ok());
        assert!(Target9::validate(0).is_ok());
        assert!(Target12::validate(0).is_ok());
        assert!(Target14::validate(0).is_ok());
        assert!(Target15::validate(0).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(Target1::validate(0).is_err());
        assert!(Target4::validate(0).is_err());
        assert!(Target6::validate(0).is_err());
        assert!(Target7::validate(0).is_err());
        assert!(Target10::validate(0).is_err());
        assert!(Target11::validate(0).is_err());
        assert!(Target13::validate(0).is_err());
        assert!(Target16::validate(0).is_err());
    }
}
