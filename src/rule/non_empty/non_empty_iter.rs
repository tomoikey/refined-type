use crate::rule::NonEmptyRule;
use crate::Refined;
use std::slice::Iter;
use std::vec::IntoIter;

pub type NonEmptyIntoIter<T> = Refined<NonEmptyIntoIterRule<T>>;
pub type NonEmptyIntoIterRule<T> = NonEmptyRule<IntoIter<T>>;

pub type NonEmptyIter<'a, T> = Refined<NonEmptyIterRule<'a, T>>;
pub type NonEmptyIterRule<'a, T> = NonEmptyRule<Iter<'a, T>>;
