use serde::{Deserialize, Serialize};
use serde_json::json;

use refined_type::result::Error;
use refined_type::rule::composer::Not;
use refined_type::rule::{
    EqualU8, ExistsVec, ForAllVec, GreaterU8, HeadVec, IndexRuleVec, IndexVec, InitVec, LastVec,
    LengthDefinition, LengthEqual, LengthEqualRule, LengthGreater, LengthLess, LengthMinMax,
    LessU8, MinMaxU8, NonEmptyString, NonEmptyStringRule, NonEmptyVec, NonEmptyVecDeque, Reverse,
    Rule, SkipFirst, SkipVec, TailVec,
};
use refined_type::{And, Or, Refined};

// define a struct for converting from JSON.
#[derive(Debug, Deserialize)]
struct Human {
    name: NonEmptyString,
    age: MinMaxU8<18, 80>,
    friends: NonEmptyVec<String>,
}

#[test]
fn get_started_simple_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 20,
        "friends": ["tom", "taro"]
    }}
    .to_string();

    let human = serde_json::from_str::<Human>(&json)?;

    assert_eq!(human.name.into_value(), "john");
    assert_eq!(human.age.into_value(), 20);
    assert_eq!(human.friends.into_value(), vec!["tom", "taro"]);
    Ok(())
}

// In the second example, while `friends` meets the rule, `name` does not, causing the conversion from JSON to fail
#[test]
fn get_started_empty_name_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "",
        "age": 20,
        "friends": ["tom", "taro"]
    }}
    .to_string();

    // because `name` is empty
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

#[test]
fn get_started_outbound_age_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 100,
        "friends": ["tom", "taro"]
    }}
    .to_string();

    // because `age` is not in the range of 18 to 80
    assert!(serde_json::from_str::<Human>(&json).is_err());
    Ok(())
}

// In the third example, while `name` satisfies the rule, `friends` does not, causing the conversion from JSON to fail.
#[test]
fn get_started_empty_vec_example() -> anyhow::Result<()> {
    let json = json! {{
        "name": "john",
        "age": 20,
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

#[test]
fn min_max_example() -> Result<(), Error<u8>> {
    type Age = MinMaxU8<18, 80>;

    let age = Age::new(18)?;
    assert_eq!(age.into_value(), 18);

    let age = Age::new(80)?;
    assert_eq!(age.into_value(), 80);

    let age = Age::new(17);
    assert!(age.is_err());

    let age = Age::new(81);
    assert!(age.is_err());
    Ok(())
}

#[test]
fn less_example() -> Result<(), Error<u8>> {
    type Age = LessU8<80>;

    let age = Age::new(79)?;
    assert_eq!(age.into_value(), 79);

    let age = Age::new(80);
    assert!(age.is_err());

    Ok(())
}

#[test]
fn greater_example() -> Result<(), Error<u8>> {
    type Age = GreaterU8<18>;

    let age = Age::new(19)?;
    assert_eq!(age.into_value(), 19);

    let age = Age::new(18);
    assert!(age.is_err());

    Ok(())
}

#[test]
fn equal_example() -> Result<(), Error<u8>> {
    type Age = EqualU8<18>;

    let age = Age::new(18)?;
    assert_eq!(age.into_value(), 18);

    let age = Age::new(19);
    assert!(age.is_err());

    Ok(())
}

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
        let refined = IndexVec::<1, NonEmptyStringRule>::new(value.clone());
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
        let refined = Reverse::<IndexRuleVec<0, NonEmptyStringRule>>::new(value.clone());
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
fn length_min_max_example() -> Result<(), Error<String>> {
    type Password = LengthMinMax<5, 10, String>;

    let password = Password::new("123456".to_string())?;
    assert_eq!(password.into_value(), "123456");

    let password = Password::new("1234".to_string());
    assert!(password.is_err());

    let password = Password::new("12345678901".to_string());
    assert!(password.is_err());

    Ok(())
}

#[test]
fn length_greater_example() -> Result<(), Error<String>> {
    type Password = LengthGreater<5, String>;

    let password = Password::new("123456".to_string())?;
    assert_eq!(password.into_value(), "123456");

    let password = Password::new("1234".to_string());
    assert!(password.is_err());

    Ok(())
}

#[test]
fn length_less_example() -> Result<(), Error<String>> {
    type Password = LengthLess<10, String>;

    let password = Password::new("123456".to_string())?;
    assert_eq!(password.into_value(), "123456");

    let password = Password::new("12345678901".to_string());
    assert!(password.is_err());

    Ok(())
}

#[test]
fn length_equal_example() -> Result<(), Error<String>> {
    type Password = LengthEqual<5, String>;

    let password = Password::new("12345".to_string())?;
    assert_eq!(password.into_value(), "12345");

    let password = Password::new("1234".to_string());
    assert!(password.is_err());

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
fn custom_length_example() -> Result<(), Error<Hello>> {
    let hello = Refined::<LengthEqualRule<5, Hello>>::new(Hello)?;
    assert_eq!(hello.into_value(), Hello);
    Ok(())
}
