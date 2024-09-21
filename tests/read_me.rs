use serde::{Deserialize, Serialize};
use serde_json::json;

use refined_type::result::Error;
use refined_type::rule::composer::Not;
use refined_type::rule::{
    ExistsVec, ForAllVec, HeadVec, Index0VecRule, Index1Vec, InitVec, LastVec, LengthDefinition,
    NonEmptyRule, NonEmptyString, NonEmptyStringRule, NonEmptyVec, NonEmptyVecDeque, Reverse, Rule,
    SkipFirst, SkipVec, TailVec,
};
use refined_type::{
    equal_rule, greater_rule, length_equal, length_greater_than, length_less_than, less_rule, And,
    Or, Refined,
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
        name: MyNonEmptyString::unsafe_new("john".to_string()),
        friends: MyNonEmptyVec::unsafe_new(vec!["tom".to_string(), "taro".to_string()]),
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

struct ContainsHelloRule;
struct ContainsCommaRule;
struct ContainsWorldRule;

impl Rule for ContainsHelloRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<String, Error<String>> {
        if target.contains("Hello") {
            Ok(target)
        } else {
            let message = format!("{} does not contain `Hello`", target);
            Err(Error::new(target, message))
        }
    }
}

impl Rule for ContainsCommaRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<String, Error<String>> {
        if target.contains(",") {
            Ok(target)
        } else {
            let message = format!("{} does not contain `,`", target);
            Err(Error::new(target, message))
        }
    }
}

impl Rule for ContainsWorldRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<String, Error<String>> {
        if target.contains("World") {
            Ok(target)
        } else {
            let message = format!("{} does not contain `World`", target);
            Err(Error::new(target, message))
        }
    }
}

#[test]
fn example_5() {
    type HelloAndWorldRule = And![ContainsHelloRule, ContainsWorldRule];

    let rule_ok = Refined::<HelloAndWorldRule>::new("Hello! World!".to_string());
    assert!(rule_ok.is_ok());

    let rule_err = Refined::<HelloAndWorldRule>::new("Hello, world!".to_string());
    assert!(rule_err.is_err());
}

#[test]
fn example_6() {
    type HelloOrWorldRule = Or![ContainsHelloRule, ContainsWorldRule];

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

    fn validate(target: Self::Item) -> Result<String, Error<String>> {
        if target.starts_with("Hello") {
            Ok(target)
        } else {
            let message = format!("{} does not start with `Hello`", target);
            Err(Error::new(target, message))
        }
    }
}

impl Rule for StartsWithByeRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<String, Error<String>> {
        if target.starts_with("Bye") {
            Ok(target)
        } else {
            let message = format!("{} does not start with `Bye`", target);
            Err(Error::new(target, message))
        }
    }
}

impl Rule for EndsWithJohnRule {
    type Item = String;

    fn validate(target: Self::Item) -> Result<String, Error<String>> {
        if target.ends_with("John") {
            Ok(target)
        } else {
            let message = format!("{} does not end with `John`", target);
            Err(Error::new(target, message))
        }
    }
}

#[test]
fn example_8() {
    type GreetingRule = And![
        Or![StartsWithHelloRule, StartsWithByeRule],
        EndsWithJohnRule
    ];

    assert!(GreetingRule::validate("Hello! Nice to meet you John".to_string()).is_ok());
    assert!(GreetingRule::validate("Bye! Have a good day John".to_string()).is_ok());
    assert!(GreetingRule::validate("How are you? Have a good day John".to_string()).is_err());
    assert!(GreetingRule::validate("Bye! Have a good day Tom".to_string()).is_err());
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
struct Human2 {
    name: NonEmptyString,
    age: u8,
}

#[test]
fn example_9() -> anyhow::Result<()> {
    let john = Human2 {
        name: NonEmptyString::unsafe_new("john".to_string()),
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
        name: NonEmptyString::unsafe_new("john".to_string()),
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
type TargetAge18OrMore = Or![EqualRule18u8, GreaterRule18u8];

// age <= 80
#[allow(dead_code)]
type TargetAge80OrLess = Or![EqualRule80u8, LessRule80u8];

// 18 <= age <= 80
#[allow(dead_code)]
type TargetAgeRule = And![TargetAge18OrMore, TargetAge80OrLess];

#[test]
fn example_11() -> Result<(), Error<Vec<String>>> {
    let vec = vec!["Hello".to_string(), "World".to_string()];
    let for_all_ok = ForAllVec::<NonEmptyStringRule>::new(vec.clone())?;
    assert_eq!(vec, for_all_ok.into_value());

    let vec = vec!["Hello".to_string(), "".to_string()];
    let for_all_err = ForAllVec::<NonEmptyStringRule>::new(vec.clone());
    assert!(for_all_err.is_err());
    Ok(())
}

#[test]
fn example_12() -> Result<(), Error<Vec<String>>> {
    let vec = vec!["Hello".to_string(), "".to_string()];
    let exists_ok = ExistsVec::<NonEmptyStringRule>::new(vec.clone())?;
    assert_eq!(vec, exists_ok.into_value());

    let vec = vec!["".to_string(), "".to_string()];
    let exists_err = ExistsVec::<NonEmptyStringRule>::new(vec.clone());
    assert!(exists_err.is_err());
    Ok(())
}

#[test]
fn example_13() -> anyhow::Result<()> {
    let table = vec![
        (vec!["good morning".to_string(), "".to_string()], true), // PASS
        (vec!["hello".to_string(), "hello".to_string()], true),   // PASS
        (vec![], false),                                          // FAIL
        (vec!["".to_string()], false),                            // FAIL
        (vec!["".to_string(), "hello".to_string()], false),       // FAIL
    ];

    for (value, ok) in table {
        let head = HeadVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(head.is_ok(), ok);
    }

    Ok(())
}

#[test]
fn example_14() -> anyhow::Result<()> {
    let table = vec![
        (vec!["".to_string(), "hello".to_string()], true), // PASS
        (vec!["good morning".to_string(), "hello".to_string()], true), // PASS
        (vec![], false),                                   // FAIL
        (vec!["".to_string()], false),                     // FAIL
        (vec!["hello".to_string(), "".to_string()], false), // FAIL
    ];

    for (value, ok) in table {
        let last = LastVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(last.is_ok(), ok);
    }

    Ok(())
}

#[test]
fn example_15() -> anyhow::Result<()> {
    let table = vec![
        (
            vec!["hey".to_string(), "hello".to_string(), "world".to_string()],
            true,
        ),
        (
            vec!["hey".to_string(), "hello".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["hey".to_string(), "".to_string(), "world".to_string()],
            false,
        ),
        (
            vec!["hey".to_string(), "".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "hello".to_string(), "world".to_string()],
            true,
        ),
        (
            vec!["".to_string(), "hello".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "".to_string(), "world".to_string()],
            false,
        ),
        (vec!["".to_string(), "".to_string(), "".to_string()], false),
    ];

    for (value, ok) in table {
        let tail = TailVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(tail.is_ok(), ok);
    }

    Ok(())
}

#[test]
fn example_16() -> anyhow::Result<()> {
    let table = vec![
        (
            vec!["hey".to_string(), "hello".to_string(), "world".to_string()],
            true,
        ),
        (
            vec!["hey".to_string(), "hello".to_string(), "".to_string()],
            true,
        ),
        (
            vec!["hey".to_string(), "".to_string(), "world".to_string()],
            false,
        ),
        (
            vec!["hey".to_string(), "".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "hello".to_string(), "world".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "hello".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "".to_string(), "world".to_string()],
            false,
        ),
        (vec!["".to_string(), "".to_string(), "".to_string()], false),
    ];

    for (value, ok) in table {
        let init = InitVec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(init.is_ok(), ok);
    }

    Ok(())
}

#[test]
fn example_17() -> anyhow::Result<()> {
    let table = vec![
        (vec!["good morning".to_string(), "hello".to_string()], true),
        (vec!["good morning".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string()], true),
        (vec!["".to_string(), "".to_string()], false),
    ];

    for (value, expected) in table {
        let refined = Index1Vec::<NonEmptyStringRule>::new(value.clone());
        assert_eq!(refined.is_ok(), expected);
    }

    Ok(())
}

#[test]
fn example_18() -> Result<(), Error<Vec<i32>>> {
    let table = vec![
        (vec!["good morning".to_string(), "hello".to_string()], true),
        (vec!["good morning".to_string(), "".to_string()], false),
        (vec!["".to_string(), "hello".to_string()], true),
        (vec!["".to_string(), "".to_string()], false),
    ];

    for (value, expected) in table {
        let refined = Reverse::<Index0VecRule<NonEmptyStringRule>>::new(value.clone());
        assert_eq!(refined.is_ok(), expected);
    }

    Ok(())
}

#[test]
fn example_19() -> Result<(), Error<Vec<i32>>> {
    let table = vec![
        (
            vec!["hey".to_string(), "hello".to_string(), "world".to_string()],
            true,
        ),
        (
            vec!["hey".to_string(), "hello".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["hey".to_string(), "".to_string(), "world".to_string()],
            false,
        ),
        (
            vec!["hey".to_string(), "".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "hello".to_string(), "world".to_string()],
            true,
        ),
        (
            vec!["".to_string(), "hello".to_string(), "".to_string()],
            false,
        ),
        (
            vec!["".to_string(), "".to_string(), "world".to_string()],
            false,
        ),
        (vec!["".to_string(), "".to_string(), "".to_string()], false),
    ];

    for (value, ok) in table {
        let init = SkipVec::<NonEmptyStringRule, SkipFirst<_>>::new(value.clone());
        assert_eq!(init.is_ok(), ok);
    }

    Ok(())
}

#[test]
fn example_22() -> Result<(), Error<Vec<i32>>> {
    let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
    let ne_vec_deque: NonEmptyVecDeque<i32> = ne_vec.into_iter().collect();
    assert_eq!(ne_vec_deque.into_value(), vec![1, 2, 3]);
    Ok(())
}

#[test]
fn example_23() -> Result<(), Error<String>> {
    length_greater_than!(5);
    length_equal!(5, 10);
    length_less_than!(10);

    type Password = Refined<From5To10Rule<String>>;

    type From5To10Rule<T> = And![
        Or![LengthEqualRule5<T>, LengthGreaterThanRule5<T>],
        Or![LengthLessThanRule10<T>, LengthEqualRule10<T>]
    ];

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
fn example_24() -> Result<(), Error<Vec<String>>> {
    length_greater_than!(5);
    length_equal!(5, 10);
    length_less_than!(10);

    type Friends = Refined<From5To10Rule<Vec<String>>>;

    type From5To10Rule<T> = And![
        Or![LengthEqualRule5<T>, LengthGreaterThanRule5<T>],
        Or![LengthLessThanRule10<T>, LengthEqualRule10<T>]
    ];

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

#[derive(Debug, PartialEq)]
struct Hello;
impl LengthDefinition for Hello {
    fn length(&self) -> usize {
        5
    }
}

#[test]
fn example_25() -> Result<(), Error<Hello>> {
    length_equal!(5);
    let hello = Refined::<LengthEqualRule5<Hello>>::new(Hello)?;
    assert_eq!(hello.into_value(), Hello);
    Ok(())
}
