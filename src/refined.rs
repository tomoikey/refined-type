use crate::rule::Rule;
use anyhow::Result;
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug)]
struct Refined<RULE, T>
where
    RULE: Rule<TARGET = T>,
{
    value: T,
    _phantom_data: PhantomData<RULE>,
}

impl<RULE, T> Refined<RULE, T>
where
    RULE: Rule<TARGET = T>,
{
    pub fn new(value: T) -> Result<Self> {
        Ok(Self {
            value: RULE::validate(value)?,
            _phantom_data: Default::default(),
        })
    }
}

impl<RULE, T> Deref for Refined<RULE, T>
where
    RULE: Rule<TARGET = T>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod test {
    use crate::refined::Refined;
    use crate::rule::NonEmptyStringRule;
    use anyhow::Result;
    use std::ops::Deref;

    type NonEmptyString = Refined<NonEmptyStringRule, String>;

    #[test]
    fn test_non_empty_string_ok() -> Result<()> {
        let non_empty_string: NonEmptyString = Refined::new("Hello".to_string())?;
        assert_eq!(non_empty_string.deref(), "Hello");
        Ok(())
    }

    #[test]
    fn test_non_empty_string_err() -> Result<()> {
        let non_empty_string: Result<NonEmptyString> = Refined::new("".to_string());
        assert!(non_empty_string.is_err());
        Ok(())
    }
}
