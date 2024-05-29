/// This macro generates a rule that checks if the length of the target is equal to `N`
/// # Example
/// ```rust
/// use refined_type::length_equal;
/// length_equal!(5);
///
/// let target = "12345";
/// let refined = LengthEqual5::new(target).unwrap();
/// assert_eq!(refined.into_value(), "12345");
///
/// let target = "1234";
/// let refined = LengthEqual5::new(target);
/// assert!(refined.is_err());
/// ```
#[macro_export]
macro_rules! length_equal {
    ($length:literal) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the LengthEqualN rule.
            #[allow(dead_code)]
            pub type [<LengthEqual $length>]<ITEM> = $crate::Refined<[<LengthEqualRule $length>]<ITEM>>;

            /// Rule where the length of the input value is equal to N
            #[allow(dead_code)]
            #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct [<LengthEqualRule $length>]<ITEM> {
                _phantom: ::std::marker::PhantomData<ITEM>,
            }

            impl<ITEM> $crate::rule::Rule for [<LengthEqualRule $length>]<ITEM> where ITEM: $crate::rule::LengthDefinition {
                type Item = ITEM;
                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    if target.length() == $length {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("target length is not equal to {}", $length)))
                    }
                }
            }
        }
    };
    ($length:literal, $($lengths:literal),+) => {
        length_equal!($length);
        length_equal!($($lengths),+);
    };
}

#[cfg(test)]
mod tests {
    use crate::result::Error;

    length_equal!(5, 10);

    #[test]
    fn test_length_equal_5() -> Result<(), Error> {
        let target = "12345";
        let refined = LengthEqual5::new(target)?;
        assert_eq!(refined.into_value(), "12345");
        Ok(())
    }

    #[test]
    fn test_length_equal_5_fail() {
        let target = "1234";
        let refined = LengthEqual5::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_equal_10() -> Result<(), Error> {
        let target = "1234567890";
        let refined = LengthEqual10::new(target)?;
        assert_eq!(refined.into_value(), "1234567890");
        Ok(())
    }

    #[test]
    fn test_length_equal_10_fail() {
        let target = "123456789";
        let refined = LengthEqual10::new(target);
        assert!(refined.is_err());
    }
}
