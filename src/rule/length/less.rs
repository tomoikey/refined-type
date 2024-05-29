/// This macro generates a rule that checks if the length of the target is less than `N`
/// # Example
/// ```rust
/// use refined_type::length_less_than;
/// length_less_than!(5);
///
/// let target = "1234";
/// let refined = LengthLessThan5::new(target).unwrap();
/// assert_eq!(refined.into_value(), "1234");
///
/// let target = "12345";
/// let refined = LengthLessThan5::new(target);
/// assert!(refined.is_err());
/// ```
#[macro_export]
macro_rules! length_less_than {
    ($length:literal) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the LengthLessThanN rule.
            #[allow(dead_code)]
            pub type [<LengthLessThan $length>]<ITEM> = $crate::Refined<[<LengthLessThanRule $length>]<ITEM>>;

            /// Rule where the length of the input value is less than N
            #[allow(dead_code)]
            pub struct [<LengthLessThanRule $length>]<ITEM> {
                _phantom: ::std::marker::PhantomData<ITEM>,
            }

            impl<ITEM> $crate::rule::Rule for [<LengthLessThanRule $length>]<ITEM> where ITEM: $crate::rule::LengthDefinition {
                type Item = ITEM;
                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    if target.length() < $length {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("target length is not less than {}", $length)))
                    }
                }
            }
        }
    };
    ($length:literal, $($lengths:literal),+) => {
        length_less_than!($length);
        length_less_than!($($lengths),+);
    };
}

#[cfg(test)]
mod tests {
    use crate::result::Error;

    length_less_than!(5, 10);

    #[test]
    fn test_length_less_than_5() -> Result<(), Error> {
        let target = "1234";
        let refined = LengthLessThan5::new(target)?;
        assert_eq!(refined.into_value(), "1234");
        Ok(())
    }

    #[test]
    fn test_length_less_than_5_fail() {
        let target = "12345";
        let refined = LengthLessThan5::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_less_than_10() -> Result<(), Error> {
        let target = "123456789";
        let refined = LengthLessThan10::new(target)?;
        assert_eq!(refined.into_value(), "123456789");
        Ok(())
    }

    #[test]
    fn test_length_less_than_10_fail() {
        let target = "1234567890";
        let refined = LengthLessThan10::new(target);
        assert!(refined.is_err());
    }
}
