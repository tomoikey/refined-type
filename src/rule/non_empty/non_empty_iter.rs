use crate::rule::NonEmptyRule;
use crate::Refined;
use std::vec::IntoIter;

pub type NonEmptyIntoIter<T> = Refined<NonEmptyIntoIterRule<T>>;
pub type NonEmptyIntoIterRule<T> = NonEmptyRule<IntoIter<T>>;
