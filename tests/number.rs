use refined_type::less_rule;
use refined_type::result::Error;

less_rule!((5, i8));

#[test]
fn test_less_than_5() -> Result<(), Error> {
    let target = 4;
    let refined = Less5i8::new(target)?;
    assert_eq!(refined.into_value(), 4);
    Ok(())
}
