use crate::rule::Iterable;
use std::collections::{HashSet, VecDeque};

macro_rules! declare_iterable_for_collection {
    ($($t:ty),*) => {
        $(
            impl<'a, T: 'a> Iterable<'a> for $t {
                type Item = T;

                fn into_iterator(self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                    Box::new(self.into_iter())
                }
            }
        )*
    };
}

declare_iterable_for_collection![Vec<T>, VecDeque<T>, HashSet<T>];
