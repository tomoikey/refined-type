macro_rules! define_greater_rule {
    ($t: ty) => {
        $crate::paste::item! {
            pub type [<Greater $t:camel>]<const THAN: $t> = $crate::Refined<[<GreaterRule $t:camel>]<THAN>>;

            #[derive(Debug, Clone, Copy)]
            pub struct [<GreaterRule $t:camel>]<const THAN: $t>;

            impl<const THAN: $t> $crate::rule::Rule for [<GreaterRule $t:camel>]<THAN> {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target > THAN {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(target, format!("{} is not greater than {}", target, THAN)))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        define_greater_rule!($t);
        define_greater_rule!($($ts), +);
    };
}

define_greater_rule!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod test {
    use crate::rule::GreaterI8;

    #[test]
    fn test_greater_than_50i8_ok() {
        let less_result = GreaterI8::<50>::new(100);
        assert!(less_result.is_ok());
    }

    #[test]
    fn test_greater_than_50i8_err() {
        let less_result = GreaterI8::<50>::new(50);
        assert!(less_result.is_err());
    }
}
