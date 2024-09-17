mod collection;
mod string;

use crate::rule::Rule;
use crate::Refined;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub type Reverse<RULE, ITERABLE> = Refined<ReverseRule<RULE, ITERABLE>>;

pub type ReverseVec<RULE> = Refined<ReverseVecRule<RULE>>;

pub type ReverseVecDeque<RULE> = Refined<ReverseVecDequeRule<RULE>>;

pub type ReverseString<RULE> = Refined<ReverseStringRule<RULE>>;

pub struct ReverseRule<RULE, ITERABLE> {
    _phantom_data: PhantomData<(RULE, ITERABLE)>,
}

pub type ReverseVecRule<RULE> = ReverseRule<RULE, Vec<<RULE as Rule>::Item>>;

pub type ReverseVecDequeRule<RULE> = ReverseRule<RULE, VecDeque<<RULE as Rule>::Item>>;

pub type ReverseStringRule<RULE> = ReverseRule<RULE, String>;
