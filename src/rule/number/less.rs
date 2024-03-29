#[macro_export]
macro_rules! less_rule {
    (($e: literal, $t: ty)) => {
        paste::item! {
            /// `Less[N][TYPE]` is a type that indicates that the number is less than `N`.
            #[allow(dead_code)]
            pub type [<Less $e $t>] = $crate::Refined<[<LessRule $e $t>]>;

            #[allow(dead_code)]
            pub struct [<LessRule $e $t>];

            impl $crate::rule::Rule for [<LessRule $e $t>] {
                type Item = $t;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target < $e {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(format!("{} is not less than {}", target, $e), target))
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
    less_rule!((50, i8));

    #[test]
    fn test_less_than_50i8_ok() {
        let less_result = Less50i8::new(1);
        assert!(less_result.is_ok());
    }

    #[test]
    fn test_less_than_50i8_err() {
        let less_result = Less50i8::new(50);
        assert!(less_result.is_err());
    }
}
