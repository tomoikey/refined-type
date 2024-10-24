use std::collections::VecDeque;

pub trait Iterable<'a> {
    type Item: 'a;

    fn into_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a>;
    fn length(&self) -> usize;
}

impl<'a, T> Iterable<'a> for Vec<T>
where
    T: 'a,
{
    type Item = T;

    fn into_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a> {
        Box::new(self.into_iter())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a, T> Iterable<'a> for VecDeque<T>
where
    T: 'a,
{
    type Item = T;

    fn into_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a> {
        Box::new(self.into_iter())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a> Iterable<'a> for String {
    type Item = char;

    fn into_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a> {
        Box::new(self.chars().collect::<Vec<_>>().into_iter())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a> Iterable<'a> for &'a str {
    type Item = char;

    fn into_iterator(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a> {
        Box::new(self.chars())
    }

    fn length(&self) -> usize {
        self.len()
    }
}
