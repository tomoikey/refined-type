use refined_type::rule::composer::{If, IfElse};
use refined_type::rule::{
    EmailStringRule, EvenRuleU8, ExistsVecRule, ForAllVecRule, GreaterRuleU8, HeadVecRule,
    NonEmptyString, NonEmptyStringRule, NonEmptyVecRule,
};
use refined_type::{And, Refined};
use serde::Deserialize;
use std::fmt::Display;

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, age: {}, friends: {:?}",
            self.name, self.age, self.friends
        )
    }
}

#[allow(clippy::type_complexity)]
#[derive(Debug, Deserialize)]
pub struct Data {
    name: NonEmptyString,
    age: Refined<If<GreaterRuleU8<10>, EvenRuleU8>>,
    friends: Refined<
        IfElse<
            And![ForAllVecRule<NonEmptyStringRule>, NonEmptyVecRule<String>],
            HeadVecRule<EmailStringRule>,
            ExistsVecRule<EmailStringRule>,
        >,
    >,
}

fn main() {
    let data = r#"
    {
        "name": "John Doe",
        "age": 20,
        "friends": ["alice@example.com", "Bob"]
    }
    "#;

    let data: Data = serde_json::from_str(data).unwrap();
    println!("{}", data); // name: John Doe, age: 20, friends: Refined { value: ["alice@example.com", "Bob"] }
}
