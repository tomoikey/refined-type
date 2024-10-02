use crate::rule::SkipOption;

pub struct NoSkip<T> {
    _phantom_data: std::marker::PhantomData<T>,
}

impl<ITEM> SkipOption for NoSkip<ITEM> {
    type Item = ITEM;
    type Accumulator = ();
    fn should_skip(_: usize, _: Option<&mut Self::Accumulator>, _: &Self::Item) -> bool {
        false
    }
}
