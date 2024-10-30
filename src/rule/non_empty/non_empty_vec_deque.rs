use crate::rule::{NonEmpty, NonEmptyRule};
use crate::Refined;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Add;

/// A type that holds a value satisfying the `NonEmptyVecDequeRule`
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

/// Rule where the input `VecDeque` is not empty
pub type NonEmptyVecDequeRule<T> = NonEmptyRule<VecDeque<T>>;

impl<T: Debug> NonEmptyVecDeque<T> {
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> NonEmpty<std::collections::vec_deque::IntoIter<T>> {
        Refined::new_unchecked(self.into_value().into_iter())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn iter(&self) -> NonEmpty<std::collections::vec_deque::Iter<T>> {
        Refined::new_unchecked(self.value().iter())
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
        Refined::new_unchecked(result)
    }

    pub fn push_back(self, value: T) -> Self {
        let mut result = self.into_value();
        result.push_back(value);
        Refined::new_unchecked(result)
    }
}

impl<T: Debug> Add for NonEmptyVecDeque<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.into_value();
        result.append(&mut rhs.into_value());
        Refined::new_unchecked(result)
    }
}

#[cfg(test)]
mod test {
    use crate::result::Error;
    use crate::rule::non_empty::non_empty_vec_deque::NonEmptyVecDeque;
    use crate::rule::NonEmptyVec;
    use std::collections::VecDeque;

    #[test]
    fn test_collect_to_vec() -> Result<(), Error<VecDeque<i32>>> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?.push_front(2).push_back(3);
        let ne_vec: NonEmptyVec<i32> = deque.into_iter().map(|n| n * 2).map(|n| n * 3).collect();
        assert_eq!(ne_vec.into_value(), vec![12, 6, 18]);
        Ok(())
    }

    #[test]
    fn test_vec_deque_push() -> Result<(), Error<VecDeque<i32>>> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?.push_front(2).push_back(3);
        assert_eq!(deque.into_value(), vec![2, 1, 3]);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), Error<VecDeque<i32>>> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?;
        assert_eq!(deque.get(0), Some(&1));
        Ok(())
    }

    #[test]
    fn test_len() -> Result<(), Error<VecDeque<i32>>> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?;
        assert_eq!(deque.len(), 1);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<(), Error<VecDeque<i32>>> {
        let mut deque = VecDeque::new();
        deque.push_front(1);
        let deque = NonEmptyVecDeque::new(deque)?;
        assert!(!deque.is_empty());
        Ok(())
    }
}
