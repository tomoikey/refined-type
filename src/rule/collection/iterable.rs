use std::collections::VecDeque;

pub trait Iterable {
    type Item;

    fn into_iterator<'a>(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a>
    where
        Self: 'a;
    fn length(&self) -> usize;
}

impl<T> Iterable for Vec<T> {
    type Item = T;

    fn into_iterator<'a>(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.into_iter())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> Iterable for VecDeque<T> {
    type Item = T;

    fn into_iterator<'a>(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.into_iter())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl Iterable for String {
    type Item = char;

    fn into_iterator<'a>(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.chars().collect::<Vec<_>>().into_iter())
    }

    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a> Iterable for &'a str {
    type Item = char;

    fn into_iterator<'b>(self) -> Box<dyn DoubleEndedIterator<Item = Self::Item> + 'b>
    where
        Self: 'b,
    {
        Box::new(self.chars())
    }

    fn length(&self) -> usize {
        self.len()
    }
}
