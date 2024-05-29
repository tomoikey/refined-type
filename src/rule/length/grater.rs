/// This macro generates a rule that checks if the length of the target is greater than `N`
/// # Example
/// ```rust
/// use refined_type::length_greater_than;
/// length_greater_than!(5);
///
/// let target = "123456";
/// let refined = LengthGreaterThan5::new(target).unwrap();
/// assert_eq!(refined.into_value(), "123456");
///
/// let target = "12345";
/// let refined = LengthGreaterThan5::new(target);
/// assert!(refined.is_err());
#[macro_export]
macro_rules! length_greater_than {
    ($length:literal) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the LengthGreaterThanN rule.
            #[allow(dead_code)]
            pub type [<LengthGreaterThan $length>]<ITEM> = $crate::Refined<[<LengthGreaterThanRule $length>]<ITEM>>;

            /// Rule where the length of the input value is greater than N
            #[allow(dead_code)]
            pub struct [<LengthGreaterThanRule $length>]<ITEM> {
                _phantom: ::std::marker::PhantomData<ITEM>,
            }

            impl<ITEM> $crate::rule::Rule for [<LengthGreaterThanRule $length>]<ITEM> where ITEM: $crate::rule::LengthDefinition {
                type Item = ITEM;
                fn validate(target: &Self::Item) -> Result<(), $crate::result::Error> {
                    if target.length() > $length {
                        Ok(())
                    } else {
                        Err($crate::result::Error::new(format!("target length is not greater than {}", $length)))
                    }
                }
            }
        }
    };
    ($length:literal, $($lengths:literal),+) => {
        length_greater_than!($length);
        length_greater_than!($($lengths),+);
    };
}

#[cfg(test)]
mod tests {
    use crate::result::Error;

    length_greater_than!(5, 10);

    #[test]
    fn test_length_greater_than_5() -> Result<(), Error> {
        let target = "123456";
        let refined = LengthGreaterThan5::new(target)?;
        assert_eq!(refined.into_value(), "123456");
        Ok(())
    }

    #[test]
    fn test_length_greater_than_5_fail() {
        let target = "1234";
        let refined = LengthGreaterThan5::new(target);
        assert!(refined.is_err());
    }

    #[test]
    fn test_length_greater_than_10() -> Result<(), Error> {
        let target = "12345678901";
        let refined = LengthGreaterThan10::new(target)?;
        assert_eq!(refined.into_value(), "12345678901");
        Ok(())
    }

    #[test]
    fn test_length_greater_than_10_fail() {
        let target = "123456789";
        let refined = LengthGreaterThan10::new(target);
        assert!(refined.is_err());
    }
}
