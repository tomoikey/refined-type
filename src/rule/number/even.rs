macro_rules! even_rule {
    ($t: ty) => {
        $crate::paste::item! {
            /// `Even[TYPE]` is a type that represents that the number is even.
            #[allow(dead_code)]
            pub type [<Even $t:upper>] = $crate::Refined<[<EvenRule $t:upper>]>;

            /// Rule where the number is even
            #[allow(dead_code)]
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct [<EvenRule $t:upper>];

            impl $crate::rule::Rule for [<EvenRule $t:upper>] {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target % 2 == 0 {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(target, format!("the value must be even, but received {target}")))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        even_rule! {$t}
        even_rule! {$($ts), +}
    };
}

even_rule!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod test {
    use crate::rule::EvenU8;

    #[test]
    fn test_even_u8_ok() {
        let n = 8;
        let even_result = EvenU8::new(n);
        assert!(even_result.is_ok())
    }

    #[test]
    fn test_even_u8_err() {
        let n = 7;
        let even_result = EvenU8::new(n);
        assert!(even_result.is_err())
    }
}
