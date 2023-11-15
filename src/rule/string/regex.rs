use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use regex::Regex as Reg;

pub type Regex = Refined<RegexRule, String>;

pub struct RegexRule {
    regex: Reg,
}

impl RegexRule {
    pub fn new(value: Reg) -> Self {
        Self { regex: value }
    }
}

impl Rule for RegexRule {
    type Item = String;

    fn validate(&self, target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        if self.regex.is_match(target.as_str()) {
            Ok(target)
        } else {
            Err(Error::new(
                format!("{} is not matched regex", target),
                target,
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rule::string::regex::RegexRule;
    use crate::rule::Rule;
    use regex::Regex;

    #[test]
    fn test_regex() {
        let rule = RegexRule::new(Regex::new(r"^Hello").unwrap());
        assert!(rule.validate("Hello".to_string()).is_ok());
        assert!(rule.validate("Hey!, Hello".to_string()).is_err());
    }
}
