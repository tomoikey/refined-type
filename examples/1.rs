use refined_type::rule::composer::{If, IfElse, Not};
use refined_type::rule::{
    EmailStringRule, EvenRuleU8, ExistsVecRule, ForAllVecRule, GreaterRuleU8, HeadVecRule,
    Ipv4AddrRule, LastVecRule, NonEmptyString, NonEmptyStringRule,
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
            And![
                ForAllVecRule<NonEmptyStringRule>,
                ExistsVecRule<Ipv4AddrRule<String>>
            ],
            HeadVecRule<EmailStringRule>,
            LastVecRule<Not<EmailStringRule>>,
        >,
    >,
}

fn main() {
    let data = r#"
    {
        "name": "John Doe",
        "age": 20,
        "friends": ["alice@example.com", "192.168.11.1"]
    }
    "#;

    let data = serde_json::from_str::<Data>(data).map(|n| n.to_string());

    // Ok("name: John Doe, age: 20, friends: Refined { value: [\"alice@example.com\", \"192.168.11.1\"] }")
    println!("{:?}", data);
}
