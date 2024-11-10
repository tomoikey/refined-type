pub use regex::Regex;

#[macro_export]
/// A macro to declare a rule that validates a `String` based on a regex pattern
/// ```rust
/// use refined_type::rule::Rule;
/// use refined_type::rule::Regex;
/// use refined_type::result::Error;
/// use refined_type::declare_regex_rule;
/// use refined_type::Refined;
///
/// declare_regex_rule![
///    EmailRule,
///    r"^[a-zA-Z0-9_.+-]+@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"
/// ];
///
/// let valid = String::from("sample@example.com");
/// assert!(EmailRule::validate(&valid).is_ok());
///
/// let invalid = String::from("example.com");
/// assert!(EmailRule::validate(&invalid).is_err());
/// ```
macro_rules! declare_regex_rule {
    ($vis:vis $rule:ident, $regex:literal) => {
        $crate::paste::item! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $vis struct $rule<STRING> {
                _phantom: std::marker::PhantomData<STRING>,
            }

            impl<STRING: AsRef<str>> $crate::rule::Rule for $rule<STRING> {
                type Item = STRING;

                fn validate(target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    let target_as_ref = target.as_ref();
                    let regex = $crate::rule::Regex::new($regex).expect("invalid regex pattern");
                    if regex.is_match(target_as_ref) {
                        Ok(target)
                    } else {
                        let message = format!("\"{target_as_ref}\" does not match the regex pattern {regex}");
                        Err($crate::result::Error::new(target, message))
                    }
                }
            }
        }
    };
    ($(($vis:vis $rule:ident, $regex:literal)),+) => {
        $(declare_regex_rule!($rule, $regex);)+
    };
}

#[cfg(test)]
mod tests {
    use crate::rule::Rule;

    declare_regex_rule![
        EmailRule,
        r"^[a-zA-Z0-9_.+-]+@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"
    ];

    declare_regex_rule![(FooRule, r"foo"), (BarRule, r"bar")];

    #[test]
    fn test_valid_email() {
        let valid = "sample@example.com";
        assert!(EmailRule::validate(&valid).is_ok())
    }

    #[test]
    fn test_invalid_email() {
        let invalid = "example.com";
        assert!(EmailRule::validate(&invalid).is_err())
    }

    #[test]
    fn test_valid_foo() {
        let valid = "foo";
        assert!(FooRule::validate(&valid).is_ok())
    }

    #[test]
    fn test_invalid_foo() {
        let invalid = String::from("bar");
        assert!(FooRule::validate(&invalid).is_err())
    }

    #[test]
    fn test_valid_bar() {
        let valid = String::from("bar");
        assert!(BarRule::validate(&valid).is_ok())
    }

    #[test]
    fn test_invalid_bar() {
        let invalid = String::from("foo");
        assert!(BarRule::validate(&invalid).is_err())
    }
}
