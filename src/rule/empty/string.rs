use crate::rule::EmptyDefinition;

impl EmptyDefinition for String {
    fn empty(&self) -> bool {
        self == &"".to_string()
    }
}

impl EmptyDefinition for &str {
    fn empty(&self) -> bool {
        self == &""
    }
}
