use crate::rule::SkipOption;

pub struct SkipEvenIndex<ITEM> {
    _phantom_data: std::marker::PhantomData<ITEM>,
}

impl<ITEM> SkipOption for SkipEvenIndex<ITEM> {
    type Item = ITEM;
    type Accumulator = ();
    fn should_skip(i: usize, _: Option<&mut Self::Accumulator>, _: &Self::Item) -> bool {
        i % 2 == 0
    }
}
