use crate::result::Error;
use crate::rule::Rule;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

/// Refined is a versatile type in ensuring that `T` satisfies the conditions of `RULE` (predicate type)
/// # Example
/// ```rust
/// use refined_type::rule::{NonEmptyString, NonEmptyStringRule};
/// use refined_type::Refined;
///
/// let non_empty_string_result = Refined::<NonEmptyStringRule>::new("Hello World".to_string());
/// assert_eq!(non_empty_string_result.unwrap().into_value(), "Hello World");
///
/// let empty_string_result = Refined::<NonEmptyStringRule>::new("".to_string());
/// assert!(empty_string_result.is_err())
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Refined<RULE>
where
    RULE: Rule,
{
    value: RULE::Item,
    _rule: PhantomData<RULE>,
}

impl<RULE, T> Serialize for Refined<RULE>
where
    RULE: Rule<Item = T>,
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, RULE, T> Deserialize<'de> for Refined<RULE>
where
    RULE: Rule<Item = T>,
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let item: T = Deserialize::deserialize(deserializer)?;
        let refined = Refined::new(item).map_err(|e| Error::custom(e.to_string()))?;
        Ok(refined)
    }
}

impl<RULE, T> Refined<RULE>
where
    RULE: Rule<Item = T>,
{
    pub fn new(value: T) -> Result<Self, Error> {
        RULE::validate(&value).map_err(|e| Error::new(e.to_string()))?;
        Ok(Self {
            value,
            _rule: Default::default(),
        })
    }

    pub fn unsafe_new(value: T) -> Self {
        RULE::validate(&value).expect("initialization by `unsafe_new` failed");
        Self {
            value,
            _rule: Default::default(),
        }
    }

    pub fn value(&self) -> &RULE::Item {
        &self.value
    }

    pub fn into_value(self) -> RULE::Item {
        self.value
    }
}

impl<RULE, T> Display for Refined<RULE>
where
    RULE: Rule<Item = T>,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::refined::Refined;
    use crate::result::Error;
    use crate::rule::{NonEmptyString, NonEmptyStringRule, NonEmptyVec};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[test]
    fn test_unsafe_new_success() {
        let non_empty_string = Refined::<NonEmptyStringRule>::unsafe_new("Hello".to_string());
        assert_eq!(non_empty_string.value, "Hello");
    }

    #[test]
    #[should_panic(expected = "initialization by `unsafe_new` failed")]
    fn test_unsafe_new_panic() {
        let non_empty_string = Refined::<NonEmptyStringRule>::unsafe_new("".to_string());
        assert_eq!(non_empty_string.value, ""); // unreachable
    }

    #[test]
    fn test_refined_non_empty_string_ok() -> Result<(), Error> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string())?;
        assert_eq!(non_empty_string.value, "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_non_empty_string_err() -> Result<(), String> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("".to_string());
        assert!(non_empty_string.is_err());
        Ok(())
    }

    #[test]
    fn test_refined_display() -> Result<(), Error> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string())?;
        assert_eq!(format!("{}", non_empty_string), "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_serialize_json_string() -> anyhow::Result<()> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("hello".to_string())?;

        let actual = json!(non_empty_string);
        let expected = json!("hello");
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_refined_serialize_json_struct() -> anyhow::Result<()> {
        type NonEmptyString = Refined<NonEmptyStringRule>;
        #[derive(Serialize)]
        struct Human {
            name: NonEmptyString,
            age: u8,
        }

        let john = Human {
            name: NonEmptyString::new("john".to_string())?,
            age: 8,
        };

        let actual = json!(john);
        let expected = json! {{
            "name": "john",
            "age": 8
        }};
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_refined_deserialize_json_ok_string() -> anyhow::Result<()> {
        let json = json!("hello").to_string();
        let non_empty_string: Refined<NonEmptyStringRule> = serde_json::from_str(&json)?;

        let actual = non_empty_string.into_value();
        let expected = "hello";
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_refined_deserialize_json_ok_1() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, Deserialize)]
        struct Human {
            name: NonEmptyString,
            friends: NonEmptyVec<String>,
            age: u8,
        }
        let json = json! {{
            "name": "john",
            "friends": ["tom", "taro"],
            "age": 8
        }}
        .to_string();

        let actual = serde_json::from_str::<Human>(&json)?;

        let expected = Human {
            name: NonEmptyString::new("john".to_string())?,
            friends: NonEmptyVec::new(vec!["tom".to_string(), "taro".to_string()])?,
            age: 8,
        };
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_refined_deserialize_json_err_1() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, Deserialize)]
        struct Human {
            name: NonEmptyString,
            friends: NonEmptyVec<String>,
            age: u8,
        }
        let json = json! {{
            "name": "john",
            "friends": [],
            "age": 8
        }}
        .to_string();

        // because `friends` is empty vec
        assert!(serde_json::from_str::<Human>(&json).is_err());
        Ok(())
    }

    #[test]
    fn test_refined_deserialize_json_err_2() -> anyhow::Result<()> {
        #[derive(Debug, Eq, PartialEq, Deserialize)]
        struct Human {
            name: NonEmptyString,
            friends: NonEmptyVec<String>,
            age: u8,
        }
        let json = json! {{
            "name": "",
            "friends": ["tom", "taro"],
            "age": 8
        }}
        .to_string();

        // because `name` is empty string
        assert!(serde_json::from_str::<Human>(&json).is_err());
        Ok(())
    }

    #[test]
    fn test_refined_deserialize_json_err_3() -> anyhow::Result<()> {
        let json = json!("").to_string();
        let result = serde_json::from_str::<Refined<NonEmptyStringRule>>(&json);
        assert!(result.is_err());
        Ok(())
    }
}
