use serde::{Deserialize, Serialize};
use serde_json::json;

use refined_type::result::Error;
use refined_type::rule::composer::{And, Not, Or};
use refined_type::rule::{
    Exists, ForAll, LengthDefinition, NonEmptyRule, NonEmptyString, NonEmptyStringRule,
    NonEmptyVec, NonEmptyVecDeque, Rule,
};
use refined_type::{
    equal_rule, greater_rule, length_equal, length_greater_than, length_less_than, less_rule,
    Refined, A, O,
};

// define the constraints you expect by combining 'Refined' and 'Rule'.
type MyNonEmptyString = Refined<NonEmptyRule<String>>;
type MyNonEmptyVec<T> = Refined<NonEmptyRule<Vec<T>>>;

// define a struct for converting from JSON.
#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Human {
    name: MyNonEmptyString,
    friends: MyNonEmptyVec<String>,
}

#[test]
fn example_1() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "friends": ["tom", "taro"]
    }}
    .to_string();

    let actual = serde_json::from_str::<Human>(&json)?;
    let expected = Human {
        name: MyNonEmptyString::new("john".to_string())?,
        friends: MyNonEmptyVec::new(vec!["tom".to_string(), "taro".to_string()])?,
    };
    assert_eq!(actual, expected);
    Ok(())
}

// In the second example, while `friends` meets the rule, `name` does not, causing the conversion from JSON to fail
#[test]
fn example_2() -> anyhow::Result<()> {
    let json = json! {{
        "name": "",
        "friends": ["tom", "taro"]
    }}
    .to_string();

    // because `name` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

// In the third example, while `name` satisfies the rule, `friends` does not, causing the conversion from JSON to fail.
#[test]
fn example_3() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "friends": []
    }}
    .to_string();

    // because `friends` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

#[test]
fn example_4() -> anyhow::Result<()> {
    let non_empty_string_result = Refined::<NonEmptyStringRule>::new("Hello World".to_string())?;
    assert_eq!(non_empty_string_result.into_value(), "Hello World");

    let empty_string_result = Refined::<NonEmptyStringRule>::new("".to_string());
    assert!(empty_string_result.is_err());
    Ok(())
}

struct ContainsHelloRule;
struct ContainsCommaRule;
struct ContainsWorldRule;

impl Rule for ContainsHelloRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.contains("Hello") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not contain `Hello`", target)))
        }
    }
}

impl Rule for ContainsCommaRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.contains(",") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not contain `,`", target)))
        }
    }
}

impl Rule for ContainsWorldRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.contains("World") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not contain `World`", target)))
        }
    }
}

#[test]
fn example_5() {
    type HelloAndWorldRule = And<ContainsHelloRule, ContainsWorldRule>;

    let rule_ok = Refined::<HelloAndWorldRule>::new("Hello! World!".to_string());
    assert!(rule_ok.is_ok());

    let rule_err = Refined::<HelloAndWorldRule>::new("Hello, world!".to_string());
    assert!(rule_err.is_err());
}

#[test]
fn example_6() {
    type HelloOrWorldRule = Or<ContainsHelloRule, ContainsWorldRule>;

    let rule_ok_1 = Refined::<HelloOrWorldRule>::new("Hello! World!".to_string());
    assert!(rule_ok_1.is_ok());

    let rule_ok_2 = Refined::<HelloOrWorldRule>::new("hello World!".to_string());
    assert!(rule_ok_2.is_ok());

    let rule_err = Refined::<HelloOrWorldRule>::new("hello, world!".to_string());
    assert!(rule_err.is_err());
}

#[test]
fn example_7() {
    type NotHelloRule = Not<ContainsHelloRule>;

    let rule_ok = Refined::<NotHelloRule>::new("hello! World!".to_string());
    assert!(rule_ok.is_ok());

    let rule_err = Refined::<NotHelloRule>::new("Hello, World!".to_string());
    assert!(rule_err.is_err());
}

struct StartsWithHelloRule;
struct StartsWithByeRule;
struct EndsWithJohnRule;

impl Rule for StartsWithHelloRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.starts_with("Hello") {
            Ok(())
        } else {
            Err(Error::new(format!(
                "{} does not start with `Hello`",
                target
            )))
        }
    }
}

impl Rule for StartsWithByeRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.starts_with("Bye") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not start with `Bye`", target)))
        }
    }
}

impl Rule for EndsWithJohnRule {
    type Item = String;

    fn validate(target: &Self::Item) -> Result<(), Error> {
        if target.ends_with("John") {
            Ok(())
        } else {
            Err(Error::new(format!("{} does not end with `John`", target)))
        }
    }
}

#[test]
fn example_8() {
    type GreetingRule = And<Or<StartsWithHelloRule, StartsWithByeRule>, EndsWithJohnRule>;

    assert!(GreetingRule::validate(&"Hello! Nice to meet you John".to_string()).is_ok());
    assert!(GreetingRule::validate(&"Bye! Have a good day John".to_string()).is_ok());
    assert!(GreetingRule::validate(&"How are you? Have a good day John".to_string()).is_err());
    assert!(GreetingRule::validate(&"Bye! Have a good day Tom".to_string()).is_err());
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
struct Human2 {
    name: NonEmptyString,
    age: u8,
}

#[test]
fn example_9() -> anyhow::Result<()> {
    let john = Human2 {
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
fn example_10() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 8
    }}
    .to_string();

    let actual = serde_json::from_str::<Human2>(&json)?;

    let expected = Human2 {
        name: NonEmptyString::new("john".to_string())?,
        age: 8,
    };
    assert_eq!(actual, expected);
    Ok(())
}

greater_rule!((18, u8));
less_rule!((80, u8));
equal_rule!((18, u8), (80, u8));

#[allow(dead_code)]
type Age = Refined<TargetAgeRule>;

// 18 <= age
#[allow(dead_code)]
type TargetAge18OrMore = Or<EqualRule18u8, GreaterRule18u8>;

// age <= 80
#[allow(dead_code)]
type TargetAge80OrLess = Or<EqualRule80u8, LessRule80u8>;

// 18 <= age <= 80
#[allow(dead_code)]
type TargetAgeRule = And<TargetAge18OrMore, TargetAge80OrLess>;

#[test]
fn example_11() -> anyhow::Result<()> {
    let vec = vec!["Hello".to_string(), "World".to_string()];
    let for_all_ok = ForAll::<NonEmptyStringRule>::new(vec.clone())?;
    assert_eq!(vec, for_all_ok.into_value());

    let vec = vec!["Hello".to_string(), "".to_string()];
    let for_all_err = ForAll::<NonEmptyStringRule>::new(vec.clone());
    assert!(for_all_err.is_err());
    Ok(())
}

#[test]
fn example_12() -> anyhow::Result<()> {
    let vec = vec!["Hello".to_string(), "".to_string()];
    let exists_ok = Exists::<NonEmptyStringRule>::new(vec.clone())?;
    assert_eq!(vec, exists_ok.into_value());

    let vec = vec!["".to_string(), "".to_string()];
    let exists_err = Exists::<NonEmptyStringRule>::new(vec.clone());
    assert!(exists_err.is_err());
    Ok(())
}

#[test]
fn example_13() -> anyhow::Result<()> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec: NonEmptyVec<i32> = ne_vec.into_iter().map(|n| n * 2).map(|n| n * 3).collect();
    assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
    Ok(())
}

#[test]
fn example_14() -> anyhow::Result<()> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec: NonEmptyVec<i32> = ne_vec.iter().map(|n| n * 2).map(|n| n * 3).collect();
    assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
    Ok(())
}

#[test]
fn example_15() -> anyhow::Result<()> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec_deque: NonEmptyVecDeque<i32> = ne_vec.into_iter().collect();
    assert_eq!(ne_vec_deque.into_value(), vec![1, 2, 3]);
    Ok(())
}

#[test]
fn example_16() -> Result<(), Error> {
    length_greater_than!(5);
    length_equal!(5, 10);
    length_less_than!(10);

    type Password = Refined<From5To10Rule<String>>;

    type From5To10Rule<T> = And<
        Or<LengthEqualRule5<T>, LengthGreaterThanRule5<T>>,
        Or<LengthLessThanRule10<T>, LengthEqualRule10<T>>,
    >;

    // length is 8. so, this is valid
    let raw_password = "password";
    let password = Password::new(raw_password.to_string())?;
    assert_eq!(password.into_value(), "password");

    // length is 4. so, this is invalid
    let raw_password = "pswd";
    let password = Password::new(raw_password.to_string());
    assert!(password.is_err());

    // length is 17. so, this is invalid
    let raw_password = "password password";
    let password = Password::new(raw_password.to_string());
    assert!(password.is_err());

    Ok(())
}

#[test]
fn example_17() -> anyhow::Result<()> {
    length_greater_than!(5);
    length_equal!(5, 10);
    length_less_than!(10);

    type Friends = Refined<From5To10Rule<Vec<String>>>;

    type From5To10Rule<T> = And<
        Or<LengthEqualRule5<T>, LengthGreaterThanRule5<T>>,
        Or<LengthLessThanRule10<T>, LengthEqualRule10<T>>,
    >;

    // length is 6. so, this is valid
    let raw_friends = vec![
        "Tom".to_string(),
        "Taro".to_string(),
        "Jiro".to_string(),
        "Hanako".to_string(),
        "Sachiko".to_string(),
        "Yoshiko".to_string(),
    ];
    let friends = Friends::new(raw_friends.clone())?;
    assert_eq!(friends.into_value(), raw_friends);

    // length is 2. so, this is invalid
    let raw_friends = vec!["Tom".to_string(), "Taro".to_string()];
    let friends = Friends::new(raw_friends.clone());
    assert!(friends.is_err());

    // length is 11. so, this is invalid
    let raw_friends = vec![
        "Tom".to_string(),
        "Taro".to_string(),
        "Jiro".to_string(),
        "Hanako".to_string(),
        "Sachiko".to_string(),
        "Yuiko".to_string(),
        "Taiko".to_string(),
        "John".to_string(),
        "Jane".to_string(),
        "Jack".to_string(),
        "Jill".to_string(),
    ];
    let friends = Friends::new(raw_friends.clone());
    assert!(friends.is_err());

    Ok(())
}

#[test]
fn example_18() -> anyhow::Result<()> {
    #[derive(Debug, PartialEq)]
    struct Hello;
    impl LengthDefinition for Hello {
        fn length(&self) -> usize {
            5
        }
    }

    length_equal!(5);
    let hello = Refined::<LengthEqualRule5<Hello>>::new(Hello)?;
    assert_eq!(hello.into_value(), Hello);
    Ok(())
}

type ContainsHelloAndWorldRule = And<ContainsHelloRule, ContainsWorldRule>;

#[allow(dead_code)]
type ContainsHelloAndWorld = Refined<ContainsHelloAndWorldRule>;

#[test]
fn example_19() -> anyhow::Result<()> {
    type Sample = Refined<A![ContainsHelloRule, ContainsCommaRule, ContainsHelloRule]>;
    let sample = Sample::new("Hello, World!".to_string())?;
    assert_eq!(sample.into_value(), "Hello, World!");

    let sample = Sample::new("Hello World!".to_string());
    assert!(sample.is_err());
    Ok(())
}

#[test]
fn example_20() -> anyhow::Result<()> {
    type Sample = Refined<O![ContainsHelloRule, ContainsCommaRule, ContainsWorldRule]>;
    let sample = Sample::new("Foo! World!".to_string())?;
    assert_eq!(sample.into_value(), "Foo! World!");

    let sample = Sample::new("Hello World!".to_string())?;
    assert_eq!(sample.into_value(), "Hello World!");

    let sample = Sample::new("World!".to_string())?;
    assert_eq!(sample.into_value(), "World!");

    let sample = Sample::new("".to_string());
    assert!(sample.is_err());
    Ok(())
}
