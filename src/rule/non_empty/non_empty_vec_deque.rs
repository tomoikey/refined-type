use crate::rule::{EmptyDefinition, NonEmpty, NonEmptyRule};
use crate::Refined;
use std::collections::VecDeque;
use std::ops::Add;

pub type NonEmptyVecDeque<T> = Refined<NonEmptyVecDequeRule<T>>;
pub type NonEmptyVecDequeRule<T> = NonEmptyRule<VecDeque<T>>;

impl<T> NonEmptyVecDeque<T>
where
    T: EmptyDefinition,
{
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> NonEmpty<std::collections::vec_deque::IntoIter<T>> {
        Refined::new(self.into_value().into_iter())
            .ok()
            .expect("This error is always unreachable")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn iter(&self) -> NonEmpty<std::collections::vec_deque::Iter<T>> {
        Refined::new(self.value().iter())
            .ok()
            .expect("This error is always unreachable")
    }
}

impl<T> Add for NonEmptyVecDeque<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.into_value();
        result.append(&mut rhs.into_value());
        Refined::new(result)
            .ok()
            .expect("This error is always unreachable")
    }
}
