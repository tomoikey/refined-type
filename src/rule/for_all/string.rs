use crate::rule::Iterable;

impl<'a> Iterable<'a> for String {
    type Item = char;

    fn into_iterator(self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(self.chars().collect::<Vec<_>>().into_iter())
    }
}

impl<'a> Iterable<'a> for &'a str {
    type Item = char;

    fn into_iterator(self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(self.chars())
    }
}
