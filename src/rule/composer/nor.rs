use crate::rule::composer::Not;
use crate::Or;

/// This is a type that represents logical NOR in logic.
pub type Nor<RULE1, RULE2> = Not<Or![RULE1, RULE2]>;

#[macro_export]
macro_rules! Nor {
    ($rule1:ty, $rule2:ty) => {
        $crate::rule::composer::Nor<$rule1, $rule2>
    };

    ($rule1:ty, $($rule2: ty), +) => {
        $crate::rule::composer::Nor<$rule1, $crate::Or![$($rule2), +]>
    }
}

#[cfg(test)]
mod test_2 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Nor![ValidI8, ValidI8]; // 2: ERR
    type Target2 = Nor![ValidI8, InvalidI8]; // 1: ERR
    type Target3 = Nor![InvalidI8, ValidI8]; // 1: ERR
    type Target4 = Nor![InvalidI8, InvalidI8]; // 0: PASS

    #[test]
    fn test_rule_binder_ok() {
        assert!(Target4::validate(0).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(Target1::validate(0).is_err());
        assert!(Target2::validate(0).is_err());
        assert!(Target3::validate(0).is_err());
    }
}

#[cfg(test)]
mod test_3 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Nor![ValidI8, ValidI8, ValidI8]; // 3: ERR
    type Target2 = Nor![ValidI8, ValidI8, InvalidI8]; // 2: ERR
    type Target3 = Nor![ValidI8, InvalidI8, ValidI8]; // 2: ERR
    type Target4 = Nor![ValidI8, InvalidI8, InvalidI8]; // 1: ERR
    type Target5 = Nor![InvalidI8, ValidI8, ValidI8]; // 2: ERR
    type Target6 = Nor![InvalidI8, ValidI8, InvalidI8]; // 1: ERR
    type Target7 = Nor![InvalidI8, InvalidI8, ValidI8]; // 1: ERR
    type Target8 = Nor![InvalidI8, InvalidI8, InvalidI8]; // 0: PASS

    #[test]
    fn test_rule_binder_ok() {
        assert!(Target8::validate(0).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(Target1::validate(0).is_err());
        assert!(Target2::validate(0).is_err());
        assert!(Target3::validate(0).is_err());
        assert!(Target4::validate(0).is_err());
        assert!(Target5::validate(0).is_err());
        assert!(Target6::validate(0).is_err());
        assert!(Target7::validate(0).is_err());
    }
}
