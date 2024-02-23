#[macro_export]
macro_rules! equal_rule {
    (($e: literal, $t: ty)) => {
        paste::item! {
            pub type [<Equal $e $t>] = $crate::Refined<[<EqualRule $e $t>]>;
            pub struct [<EqualRule $e $t>];

            impl $crate::rule::Rule for [<EqualRule $e $t>] {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target == $e {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(format!("{} does not equal {}", target, $e), target))
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

#[cfg(test)]
mod test {
    equal_rule!((50, i8));

    #[test]
    fn test_equal_than_50i8_ok() {
        let less_result = Equal50i8::new(50);
        assert!(less_result.is_ok());
    }

    #[test]
    fn test_equal_than_50i8_err() {
        let less_result = Equal50i8::new(100);
        assert!(less_result.is_err());
    }
}
