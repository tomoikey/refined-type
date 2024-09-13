use refined_type::declare_regex_rule;
use refined_type::rule::Rule;

declare_regex_rule![
    EmailRule,
    r"^[a-zA-Z0-9_.+-]+@([a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]*\.)+[a-zA-Z]{2,}$"
];

#[test]
fn test_regex_valid() {
    let valid = "sample@example.com".to_string();
    assert!(EmailRule::validate(&valid).is_ok())
}

#[test]
fn test_regex_invalid() {
    let invalid = "example.com".to_string();
    assert!(EmailRule::validate(&invalid).is_err())
}
