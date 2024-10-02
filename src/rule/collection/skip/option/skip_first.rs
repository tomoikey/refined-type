use crate::rule::SkipOption;

pub struct SkipFirst<ITEM> {
    _phantom_data: std::marker::PhantomData<ITEM>,
}

impl<ITEM> SkipOption for SkipFirst<ITEM> {
    type Item = ITEM;
    type Accumulator = ();
    fn should_skip(i: usize, _: Option<&mut Self::Accumulator>, _: &Self::Item) -> bool {
        i == 0
    }
}
