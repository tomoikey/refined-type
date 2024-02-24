macro_rules! odd_rule {
    ($t: ty) => {
        paste::item! {
            /// `Odd[TYPE]` is a type that represents that the number is odd.
            pub type [<Odd $t:upper>] = $crate::Refined<[<OddRule $t:upper>]>;
            pub struct [<OddRule $t:upper>];

            impl $crate::rule::Rule for [<OddRule $t:upper>] {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target % 2 == 1 {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(format!("{} is not odd number", target), target))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        odd_rule! {$t}
        odd_rule! {$($ts), +}
    };
}

odd_rule!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod test {
    use crate::rule::OddU8;

    #[test]
    fn test_odd_u8_ok() {
        let n = 7;
        let even_result = OddU8::new(n);
        assert!(even_result.is_ok())
    }

    #[test]
    fn test_odd_u8_err() {
        let n = 8;
        let even_result = OddU8::new(n);
        assert!(even_result.is_err())
    }
}
