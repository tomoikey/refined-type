use refined_type::{length_equal, length_greater_than, length_less_than, Refined};
use refined_type::rule::composer::{And, Or};

length_greater_than!(5);
length_equal!(5, 10);
length_less_than!(10);

#[test]
fn test_length() -> Result<(), refined_type::result::Error> {
    type Password = Refined<From5To10Rule<String>>;

    type From5To10Rule<T> = And<
        Or<LengthEqualRule5<T>, LengthGreaterThanRule5<T>>,
        Or<LengthLessThanRule10<T>, LengthEqualRule10<T>>,
    >;

    let raw_password = "password";
    let password = Password::new(raw_password.to_string())?;
    assert_eq!(password.into_value(), "password");
    Ok(())
}

#[test]
fn test_length_fail() {
    type Password = Refined<
        And<
            And<LengthEqualRule5<String>, LengthGreaterThanRule5<String>>,
            And<LengthLessThanRule10<String>, LengthEqualRule10<String>>,
        >,
    >;
    let raw_password = "password password";
    let password = Password::new(raw_password.to_string());
    assert!(password.is_err());
}

#[test]
fn test_length_greater_than_5() -> Result<(), refined_type::result::Error> {
    let target = "123456";
    let refined = LengthGreaterThan5::new(target)?;
    assert_eq!(refined.into_value(), "123456");
    Ok(())
}

#[test]
fn test_length_greater_than_5_fail() {
    let target = "1234";
    let refined = LengthGreaterThan5::new(target);
    assert!(refined.is_err());
}

#[test]
fn test_length_equal_5() -> Result<(), refined_type::result::Error> {
    let target = "12345";
    let refined = LengthEqual5::new(target)?;
    assert_eq!(refined.into_value(), "12345");
    Ok(())
}

#[test]
fn test_length_equal_5_fail() {
    let target = "1234";
    let refined = LengthEqual5::new(target);
    assert!(refined.is_err());
}

#[test]
fn test_length_equal_10() -> Result<(), refined_type::result::Error> {
    let target = "1234567890";
    let refined = LengthEqual10::new(target)?;
    assert_eq!(refined.into_value(), "1234567890");
    Ok(())
}

#[test]
fn test_length_equal_10_fail() {
    let target = "123456789";
    let refined = LengthEqual10::new(target);
    assert!(refined.is_err());
}

#[test]
fn test_length_less_than_10() -> Result<(), refined_type::result::Error> {
    let target = "123456789";
    let refined = LengthLessThan10::new(target)?;
    assert_eq!(refined.into_value(), "123456789");
    Ok(())
}

#[test]
fn test_length_less_than_10_fail() {
    let target = "1234567890";
    let refined = LengthLessThan10::new(target);
    assert!(refined.is_err());
}
