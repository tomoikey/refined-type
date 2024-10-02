use crate::rule::SkipOption;

pub struct SkipFirst<ITEM> {
    _phantom_data: std::marker::PhantomData<ITEM>,
}

impl<ITEM> SkipOption for SkipFirst<ITEM> {
    type Item = ITEM;
    type Accumulator = ();
    fn should_skip(i: usize, _: &Self::Item, _: Option<&mut Self::Accumulator>) -> bool {
        i == 0
    }
}
