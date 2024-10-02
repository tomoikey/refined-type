mod no_skip;
mod skip_first;

pub use no_skip::NoSkip;
pub use skip_first::SkipFirst;

pub trait SkipOption {
    type Item;
    type Accumulator;
    fn should_skip(
        i: usize,
        accumulator: Option<&mut Self::Accumulator>,
        item: &Self::Item,
    ) -> bool;
}
