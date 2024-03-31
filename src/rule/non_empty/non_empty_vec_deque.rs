use crate::rule::{NonEmpty, NonEmptyRule};
use crate::Refined;
use std::collections::VecDeque;
use std::ops::Add;

/// `NonEmptyVecDeque` is a `VecDeque` that follows the `NonEmptyRule`
///
/// # Example
/// ```rust
/// # use std::collections::VecDeque;
/// # use refined_type::rule::NonEmptyVecDeque;
///
/// let mut deque = VecDeque::new();
/// deque.push_front(1);
/// let deque = NonEmptyVecDeque::new(deque).unwrap().push_front(2).push_back(3);
///
/// assert_eq!(deque.into_value(), vec![2, 1, 3]);
/// ```
pub type NonEmptyVecDeque<T> = Refined<NonEmptyVecDequeRule<T>>;
pub type NonEmptyVecDequeRule<T> = NonEmptyRule<VecDeque<T>>;

impl<T> NonEmptyVecDeque<T> {
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

    pub fn get(&self, index: usize) -> Option<&T> {
        self.value().get(index)
    }

    pub fn len(&self) -> usize {
        self.value().len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn push_front(self, value: T) -> Self {
        let mut result = self.into_value();
        result.push_front(value);
        Refined::new(result)
            .ok()
            .expect("This error is always unreachable")
    }

    pub fn push_back(self, value: T) -> Self {
        let mut result = self.into_value();
        result.push_back(value);
        Refined::new(result)
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

#[cfg(test)]
mod test {
    use crate::rule::non_empty::non_empty_vec_deque::NonEmptyVecDeque;
    use crate::rule::NonEmptyVec;
    use std::collections::VecDeque;

    #[test]
    fn test_collect_to_vec() -> anyhow::Result<()> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?.push_front(2).push_back(3);
        let ne_vec: NonEmptyVec<i32> = deque.into_iter().map(|n| n * 2).map(|n| n * 3).collect();
        assert_eq!(ne_vec.into_value(), vec![12, 6, 18]);
        Ok(())
    }

    #[test]
    fn test_vec_deque_push() -> anyhow::Result<()> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?.push_front(2).push_back(3);
        assert_eq!(deque.into_value(), vec![2, 1, 3]);
        Ok(())
    }

    #[test]
    fn test_get() -> anyhow::Result<()> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?;
        assert_eq!(deque.get(0), Some(&1));
        Ok(())
    }

    #[test]
    fn test_len() -> anyhow::Result<()> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?;
        assert_eq!(deque.len(), 1);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> anyhow::Result<()> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?;
        assert!(!deque.is_empty());
        Ok(())
    }
}
