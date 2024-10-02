mod no_skip;
mod skip_even_index;
mod skip_first;
mod skip_odd_index;

pub use no_skip::NoSkip;
pub use skip_even_index::SkipEvenIndex;
pub use skip_first::SkipFirst;
pub use skip_odd_index::SkipOddIndex;

pub trait SkipOption {
    type Item;
    type Accumulator;
    fn should_skip(
        i: usize,
        accumulator: Option<&mut Self::Accumulator>,
        item: &Self::Item,
    ) -> bool;
}
