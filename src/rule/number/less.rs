macro_rules! define_less_rule {
    ($t: ty) => {
        $crate::paste::item! {
            pub type [<Less $t:camel>]<const THAN: $t> = $crate::Refined<[<LessRule $t:camel>]<THAN>>;

            #[derive(Debug, Clone, Copy)]
            pub struct [<LessRule $t:camel>]<const THAN: $t>;

            impl<const THAN: $t> $crate::rule::Rule for [<LessRule $t:camel>]<THAN> {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target < THAN {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(target, format!("{} is not less than {}", target, THAN)))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        define_less_rule!($t);
        define_less_rule!($($ts), +);
    };
}

define_less_rule!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod test {
    use crate::rule::LessI8;

    #[test]
    fn test_less_than_50i8_ok() {
        let less_result = LessI8::<50>::new(1);
        assert!(less_result.is_ok());
    }

    #[test]
    fn test_less_than_50i8_err() {
        let less_result = LessI8::<50>::new(50);
        assert!(less_result.is_err());
    }
}
