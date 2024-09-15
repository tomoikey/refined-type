/// This macro generates a rule that checks if the number is greater than `N`
#[macro_export]
macro_rules! greater_rule {
    (($e: literal, $t: ty)) => {
        $crate::paste::item! {
            /// `Greater[N][TYPE]` is a type that indicates that the number is greater than `N`.
            #[allow(dead_code)]
            pub type [<Greater $e $t>] = $crate::Refined<[<GreaterRule $e $t>]>;

            /// Rule where the number is greater than `N`
            #[allow(dead_code)]
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct [<GreaterRule $e $t>];

            impl $crate::rule::Rule for [<GreaterRule $e $t>] {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<$t, $crate::result::Error<$t>> {
                    if target > $e {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(target, format!("{} is not greater than {}", target, $e)))
                    }
                }
            }
        }
    };
    (($e: literal, $t: ty), $(($es: literal, $ts: ty)),+) => {
        greater_rule!(($e, $t));
        greater_rule!($(($es, $ts)), +);
    };
}

#[cfg(test)]
mod test {
    greater_rule!((50, i8));

    #[test]
    fn test_greater_than_50i8_ok() {
        let less_result = Greater50i8::new(100);
        assert!(less_result.is_ok());
    }

    #[test]
    fn test_greater_than_50i8_err() {
        let less_result = Greater50i8::new(50);
        assert!(less_result.is_err());
    }
}
