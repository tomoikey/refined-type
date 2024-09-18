use crate::refined::Refined;
use crate::result::Error;
use crate::rule::{NonEmptyRule, NonEmptyVec};
use std::ops::Add;
use std::str::FromStr;

/// A type that holds a value satisfying the `NonEmptyStringRule`
///
/// # Example
/// ```rust
/// # use refined_type::rule::NonEmptyString;
///
/// let non_empty_string_1 = NonEmptyString::new("Hello".to_string()).unwrap();
/// let non_empty_string_2 = NonEmptyString::new("World".to_string()).unwrap();
/// let non_empty_string = non_empty_string_1 + non_empty_string_2;
///
/// assert_eq!(non_empty_string.into_value(), "HelloWorld");
/// ```
pub type NonEmptyString = Refined<NonEmptyStringRule>;

/// Rule where the input `String` is not empty
pub type NonEmptyStringRule = NonEmptyRule<String>;

impl NonEmptyString {
    pub fn insert(self, idx: usize, ch: char) -> Self {
        let mut result = self.into_value();
        result.insert(idx, ch);
        NonEmptyString::unsafe_new(result)
    }

    pub fn push(self, ch: char) -> Self {
        let mut result = self.into_value();
        result.push(ch);
        NonEmptyString::unsafe_new(result)
    }

    pub fn push_str(self, string: &str) -> Self {
        let mut result = self.into_value();
        result.push_str(string);
        NonEmptyString::unsafe_new(result)
    }

    pub fn as_bytes(&self) -> NonEmptyVec<u8> {
        NonEmptyVec::unsafe_new(self.value().as_bytes().to_vec())
    }

    pub fn repeat(&self, n: usize) -> Self {
        NonEmptyString::unsafe_new(self.value().repeat(n))
    }

    pub fn to_ascii_lowercase(&self) -> Self {
        NonEmptyString::unsafe_new(self.value().to_ascii_lowercase())
    }

    pub fn to_lowercase(&self) -> Self {
        NonEmptyString::unsafe_new(self.value().to_lowercase())
    }

    pub fn to_ascii_uppercase(&self) -> Self {
        NonEmptyString::unsafe_new(self.value().to_ascii_uppercase())
    }

    pub fn to_uppercase(&self) -> Self {
        NonEmptyString::unsafe_new(self.value().to_uppercase())
    }

    pub fn capacity(&self) -> usize {
        self.value().capacity()
    }

    pub fn len(&self) -> usize {
        self.value().len()
    }
}

impl FromStr for NonEmptyString {
    type Err = Error<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Refined::new(s.to_string())
    }
}

impl Add for NonEmptyString {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Refined::new(format!("{}{}", self.into_value(), rhs.into_value()))
            .expect("This error is always unreachable")
    }
}

#[cfg(test)]
mod test {
    use crate::result::Error;
    use crate::rule::{NonEmptyString, NonEmptyStringRule, Rule};
    use std::str::FromStr;

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyStringRule::validate("hello".to_string()).is_ok());
        assert!(NonEmptyStringRule::validate("".to_string()).is_err());
    }

    #[test]
    fn test_add_string() -> Result<(), Error<String>> {
        let non_empty_string_1 = NonEmptyString::new("Hello".to_string())?;
        let non_empty_string_2 = NonEmptyString::new("World".to_string())?;
        let non_empty_string = non_empty_string_1 + non_empty_string_2;

        assert_eq!(non_empty_string.into_value(), "HelloWorld");
        Ok(())
    }

    #[test]
    fn test_from_str() -> Result<(), Error<String>> {
        let non_empty_string = NonEmptyString::from_str("Hello")?;
        assert_eq!(non_empty_string.into_value(), "Hello");
        Ok(())
    }
}
