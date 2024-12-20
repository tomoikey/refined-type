use crate::result::Error;
use crate::rule::Rule;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};

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
    /// Creates a new `Refined` instance if the provided value satisfies the rule.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be refined.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error<T>>` - A `Refined` instance if the value satisfies the rule, otherwise an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use refined_type::rule::NonEmptyStringRule;
    /// use refined_type::Refined;
    ///
    /// let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string());
    /// assert!(non_empty_string.is_ok());
    ///
    /// let empty_string = Refined::<NonEmptyStringRule>::new("".to_string());
    /// assert!(empty_string.is_err());
    /// ```
    pub fn new(value: T) -> Result<Self, Error<T>> {
        let value = RULE::validate(value).map_err(|e| {
            let message = e.to_string();
            Error::new(e.into_value(), message)
        })?;
        Ok(Self { value })
    }

    /// Creates a new `Refined` instance if the provided value satisfies the rule.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be refined.
    ///
    /// # Panics
    ///
    /// This function will panic if the value does not satisfy the rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use refined_type::rule::NonEmptyStringRule;
    /// use refined_type::Refined;
    ///
    /// let non_empty_string = Refined::<NonEmptyStringRule>::unsafe_new("Hello".to_string());
    /// assert_eq!(non_empty_string.into_value(), "Hello");
    ///
    /// // This will panic
    /// // let empty_string = Refined::<NonEmptyStringRule>::unsafe_new("".to_string());
    /// ```
    pub fn unsafe_new(value: T) -> Self
    where
        T: Debug,
    {
        let value = RULE::validate(value).expect("initialization by `unsafe_new` failed");
        Self { value }
    }

    pub(crate) fn new_unchecked(value: T) -> Self {
        Self { value }
    }

    /// Mutates the value inside the `Refined` type using the provided function.
    ///
    /// This method takes ownership of the current `Refined` instance, applies the
    /// provided function to its inner value, and attempts to create a new `Refined`
    /// instance with the mutated value. If the mutated value does not satisfy the
    /// rule, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes the inner value and returns a new value.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Error<T>>` - A new `Refined` instance with the mutated value
    ///   if the value satisfies the rule, otherwise an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use refined_type::rule::NonEmptyString;
    /// use refined_type::Refined;
    ///
    /// let value = NonEmptyString::new("h".to_string())
    ///     .unwrap()
    ///     .mutate(|n| n + "e")
    ///     .unwrap()
    ///     .mutate(|n| n + "l")
    ///     .unwrap()
    ///     .mutate(|n| n + "l")
    ///     .unwrap()
    ///     .mutate(|n| n + "o")
    ///     .unwrap();
    /// assert_eq!(value.into_value(), "hello");
    /// ```
    pub fn mutate<F>(self, f: F) -> Result<Self, Error<T>>
    where
        F: FnOnce(T) -> T,
    {
        Refined::new(f(self.into_value()))
    }

    /// Returns a reference to the value inside the `Refined` type.
    ///
    /// # Returns
    ///
    /// * `&RULE::Item` - A reference to the value inside the `Refined` type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use refined_type::rule::NonEmptyStringRule;
    /// use refined_type::Refined;
    ///
    /// let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string()).unwrap();
    /// assert_eq!(non_empty_string.value(), "Hello");
    /// ```
    pub fn value(&self) -> &RULE::Item {
        &self.value
    }

    /// Consumes the `Refined` instance and returns the inner value.
    ///
    /// # Returns
    ///
    /// * `RULE::Item` - The value inside the `Refined` type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use refined_type::rule::NonEmptyStringRule;
    /// use refined_type::Refined;
    ///
    /// let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string()).unwrap();
    /// assert_eq!(non_empty_string.into_value(), "Hello");
    /// ```
    pub fn into_value(self) -> RULE::Item {
        self.value
    }
}

macro_rules! impl_try_from {
    ($t: ty) => {
        impl<RULE: Rule<Item = $t>> TryFrom<$t> for Refined<RULE> {
            type Error = Error<$t>;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                Refined::new(value)
            }
        }
    };
    ($($ts: ty), +) => {
        $(impl_try_from!($ts);)+
    };
}

impl_try_from![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64];
impl_try_from![String, char];

impl<'a, RULE: Rule<Item = String>> TryFrom<&'a str> for Refined<RULE> {
    type Error = Error<String>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Refined::new(value.into())
    }
}

impl<T, RULE: Rule<Item = Vec<T>>> TryFrom<Vec<T>> for Refined<RULE> {
    type Error = Error<Vec<T>>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Refined::new(value)
    }
}

impl<T, RULE: Rule<Item = VecDeque<T>>> TryFrom<VecDeque<T>> for Refined<RULE> {
    type Error = Error<VecDeque<T>>;

    fn try_from(value: VecDeque<T>) -> Result<Self, Self::Error> {
        Refined::new(value)
    }
}

impl<T, RULE: Rule<Item = HashSet<T>>> TryFrom<HashSet<T>> for Refined<RULE> {
    type Error = Error<HashSet<T>>;

    fn try_from(value: HashSet<T>) -> Result<Self, Self::Error> {
        Refined::new(value)
    }
}

impl<K, V, RULE: Rule<Item = HashMap<K, V>>> TryFrom<HashMap<K, V>> for Refined<RULE> {
    type Error = Error<HashMap<K, V>>;

    fn try_from(value: HashMap<K, V>) -> Result<Self, Self::Error> {
        Refined::new(value)
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
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::refined::Refined;
    use crate::result::Error;
    use crate::rule::{
        EqualI128, EqualI16, EqualI32, EqualI64, EqualI8, EqualIsize, EqualU128, EqualU16,
        EqualU32, EqualU64, EqualU8, EqualUsize, NonEmptyHashMap, NonEmptyHashSet, NonEmptyString,
        NonEmptyStringRule, NonEmptyVec, NonEmptyVecDeque,
    };

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
    fn test_refined_non_empty_string_ok() -> Result<(), Error<String>> {
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
    fn test_refined_display() -> Result<(), Error<String>> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("Hello".to_string())?;
        assert_eq!(format!("{}", non_empty_string), "Hello");
        Ok(())
    }

    #[test]
    fn test_refined_serialize_json_string() -> Result<(), Error<String>> {
        let non_empty_string = Refined::<NonEmptyStringRule>::new("hello".to_string())?;

        let actual = json!(non_empty_string);
        let expected = json!("hello");
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_refined_serialize_json_struct() -> Result<(), Error<String>> {
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
            name: NonEmptyString::unsafe_new("john".to_string()),
            friends: NonEmptyVec::unsafe_new(vec!["tom".to_string(), "taro".to_string()]),
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

    #[test]
    fn test_try_from() -> anyhow::Result<()> {
        let value = NonEmptyString::try_from("hello")?;
        assert_eq!(value.into_value(), "hello");

        let value = NonEmptyString::try_from("hello".to_string())?;
        assert_eq!(value.into_value(), "hello");

        let value = EqualU8::<8>::try_from(8)?;
        assert_eq!(value.into_value(), 8);

        let value = EqualU16::<16>::try_from(16)?;
        assert_eq!(value.into_value(), 16);

        let value = EqualU32::<32>::try_from(32)?;
        assert_eq!(value.into_value(), 32);

        let value = EqualU64::<64>::try_from(64)?;
        assert_eq!(value.into_value(), 64);

        let value = EqualU128::<128>::try_from(128)?;
        assert_eq!(value.into_value(), 128);

        let value = EqualUsize::<1>::try_from(1)?;
        assert_eq!(value.into_value(), 1);

        let value = EqualI8::<8>::try_from(8)?;
        assert_eq!(value.into_value(), 8);

        let value = EqualI16::<16>::try_from(16)?;
        assert_eq!(value.into_value(), 16);

        let value = EqualI32::<32>::try_from(32)?;
        assert_eq!(value.into_value(), 32);

        let value = EqualI64::<64>::try_from(64)?;
        assert_eq!(value.into_value(), 64);

        let value = EqualI128::<128>::try_from(128)?;
        assert_eq!(value.into_value(), 128);

        let value = EqualIsize::<1>::try_from(1)?;
        assert_eq!(value.into_value(), 1);

        let value = NonEmptyVec::try_from(vec!["hello".to_string()])?;
        assert_eq!(value.into_value(), vec!["hello".to_string()]);

        let value = NonEmptyVecDeque::try_from(
            vec!["hello".to_string()]
                .into_iter()
                .collect::<VecDeque<_>>(),
        )?;
        assert_eq!(value.into_value(), vec!["hello".to_string()]);

        let value = NonEmptyHashSet::try_from(
            vec!["hello".to_string()]
                .into_iter()
                .collect::<HashSet<_>>(),
        )?;
        assert_eq!(
            value.into_value(),
            vec!["hello".to_string()].into_iter().collect()
        );

        let value = NonEmptyHashMap::try_from(
            vec![("hello".to_string(), "world".to_string())]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        )?;
        assert_eq!(
            value.into_value(),
            vec![("hello".to_string(), "world".to_string())]
                .into_iter()
                .collect()
        );

        let value: NonEmptyVec<NonEmptyString> =
            NonEmptyVec::try_from(vec!["hello".to_string().try_into()?])?;
        assert_eq!(value.into_value(), vec!["hello".to_string().try_into()?]);
        Ok(())
    }

    #[test]
    fn test_mutate() -> anyhow::Result<()> {
        let value = NonEmptyString::try_from("h")?
            .mutate(|n| n + "e")?
            .mutate(|n| n + "l")?
            .mutate(|n| n + "l")?
            .mutate(|n| n + "o")?;
        assert_eq!(value.into_value(), "hello");
        Ok(())
    }
}
