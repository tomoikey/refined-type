macro_rules! min_max_rule {
    ($t: ty) => {
        paste::item! {
            /// The MinMax type ensures that a given value falls within the specified range of minimum and maximum values
            pub type [<MinMax $t:upper>] = $crate::Refined<[<MinMax $t:upper Rule>], $t>;
            pub struct [<MinMax $t:upper Rule>] {
                min: $t,
                max: $t
            }
            impl [<MinMax $t:upper Rule>] {
                pub fn new(min: $t, max: $t) -> Option<Self> {
                    if min <= max {
                        Some(Self { min, max })
                    }
                    else {
                        None
                    }
                }
            }
            impl $crate::rule::Rule for [<MinMax $t:upper Rule>] {
                type Item = $t;

                fn validate(&self, target: Self::Item) -> $crate::result::Result<Self::Item> {
                    if self.min <= target && target <= self.max {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(format!("The input is not between {} and {}", self.min, self.max)))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        min_max_rule! {$t}
        min_max_rule! {$($ts), +}
    };
}

min_max_rule! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64
}

#[cfg(test)]
mod test {
    use crate::result::Result;
    use crate::rule::{MinMaxI8Rule, Rule};

    #[test]
    fn test_min_max_i8_new() {
        let min_max_rule_result = MinMaxI8Rule::new(5, -3);
        assert!(min_max_rule_result.is_none());
    }

    #[test]
    fn test_min_max_i8() -> Result<()> {
        let min_max_rule = MinMaxI8Rule::new(-3, 5).unwrap();
        assert!(min_max_rule.validate(-4).is_err());
        assert!(min_max_rule.validate(-3).is_ok());
        assert!(min_max_rule.validate(2).is_ok());
        assert!(min_max_rule.validate(5).is_ok());
        assert!(min_max_rule.validate(6).is_err());
        Ok(())
    }
}
