use crate::rule::NonEmptyRule;
use crate::Refined;
use std::iter::Map;

pub type NonEmptyMap<I, F> = Refined<NonEmptyMapRule<I, F>>;
pub type NonEmptyMapRule<I, F> = NonEmptyRule<Map<I, F>>;
