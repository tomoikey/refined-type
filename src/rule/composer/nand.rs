use crate::rule::composer::Not;
use crate::And;

/// This is a type that represents logical NAND in logic.
pub type Nand<RULE1, RULE2> = Not<And![RULE1, RULE2]>;

#[macro_export]
macro_rules! Nand {
    ($rule1:ty, $rule2:ty) => {
        $crate::rule::composer::Nand<$rule1, $rule2>
    };

    ($rule1:ty, $($rule2: ty), +) => {
        $crate::rule::composer::Nand<$rule1, $crate::And![$($rule2), +]>
    }
}

#[cfg(test)]
mod test_2 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Nand![ValidI8, ValidI8]; // ERR
    type Target2 = Nand![ValidI8, InvalidI8]; // PASS
    type Target3 = Nand![InvalidI8, ValidI8]; // PASS
    type Target4 = Nand![InvalidI8, InvalidI8]; // PASS

    #[test]
    fn test_rule_binder_ok() {
        assert!(Target2::validate(0).is_ok());
        assert!(Target3::validate(0).is_ok());
        assert!(Target4::validate(0).is_ok());
    }

    #[test]
    fn test_rule_binder_err() {
        assert!(Target1::validate(0).is_err());
    }
}

#[cfg(test)]
mod test_3 {
    use crate::rule::{Invalid, Rule, Valid};

    type ValidI8 = Valid<i8>;
    type InvalidI8 = Invalid<i8>;

    type Target1 = Nand![ValidI8, ValidI8, ValidI8]; // ERR
    type Target2 = Nand![ValidI8, ValidI8, InvalidI8]; // PASS
    type Target3 = Nand![ValidI8, InvalidI8, ValidI8]; // PASS
    type Target4 = Nand![ValidI8, InvalidI8, InvalidI8]; // PASS
    type Target5 = Nand![InvalidI8, ValidI8, ValidI8]; // PASS
    type Target6 = Nand![InvalidI8, ValidI8, InvalidI8]; // PASS
    type Target7 = Nand![InvalidI8, InvalidI8, ValidI8]; // PASS
    type Target8 = Nand![InvalidI8, InvalidI8, InvalidI8]; // PASS
    
    #[test]
    fn test_rule_binder_ok() {
        assert!(Target2::validate(0).is_ok());
        assert!(Target3::validate(0).is_ok());
        assert!(Target4::validate(0).is_ok());
        assert!(Target5::validate(0).is_ok());
        assert!(Target6::validate(0).is_ok());
        assert!(Target7::validate(0).is_ok());
        assert!(Target8::validate(0).is_ok());
    }
    
    #[test]
    fn test_rule_binder_err() {
        assert!(Target1::validate(0).is_err());
    }
}
