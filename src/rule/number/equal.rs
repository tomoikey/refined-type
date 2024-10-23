macro_rules! define_equal_rule {
    ($t: ty) => {
        $crate::paste::item! {
            pub type [<Equal $t:camel>]<const EQUAL: $t> = $crate::Refined<[<EqualRule $t:camel>]<EQUAL>>;

            #[derive(Debug, Clone, Copy)]
            pub struct [<EqualRule $t:camel>]<const EQUAL: $t>;

            impl<const EQUAL: $t> $crate::rule::Rule for [<EqualRule $t:camel>]<EQUAL> {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target == EQUAL {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(target, format!("{} does not equal {}", target, EQUAL)))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        define_equal_rule!($t);
        define_equal_rule!($($ts), +);
    };
}

define_equal_rule!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod test {
    use crate::rule::number::equal::EqualI8;

    #[test]
    fn test_equal_than_50i8_ok() {
        let less_result = EqualI8::<50>::new(50);
        assert!(less_result.is_ok());
    }

    #[test]
    fn test_equal_than_50i8_err() {
        let less_result = EqualI8::<50>::new(100);
        assert!(less_result.is_err());
    }
}
