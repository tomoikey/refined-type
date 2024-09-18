use crate::rule::SkipOption;

pub struct NoSkip<T> {
    _phantom_data: std::marker::PhantomData<T>,
}

impl<ITEM> SkipOption for NoSkip<ITEM> {
    type Item = ITEM;
    fn should_skip(_: usize, _: &Self::Item) -> bool {
        false
    }
}
