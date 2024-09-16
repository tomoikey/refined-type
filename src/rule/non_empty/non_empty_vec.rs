use crate::rule::{NonEmpty, NonEmptyRule};
use crate::Refined;

use std::ops::Add;

/// A type that holds a value satisfying the `NonEmptyVecRule`
///
/// # Example
/// ```rust
/// # use refined_type::rule::NonEmptyVec;
/// let vec = NonEmptyVec::new(vec![1]).unwrap().push(2).push(3);
///
/// assert_eq!(vec.into_value(), vec![1, 2, 3]);
/// ```
pub type NonEmptyVec<T> = Refined<NonEmptyVecRule<T>>;

/// Rule where the input `Vec` is not empty
pub type NonEmptyVecRule<T> = NonEmptyRule<Vec<T>>;

impl<T> NonEmptyVec<T> {
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> NonEmpty<std::vec::IntoIter<T>> {
        Refined::new_unchecked(self.into_value().into_iter())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn iter(&self) -> NonEmpty<std::slice::Iter<T>> {
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

    pub fn push(self, value: T) -> Self {
        let mut result = self.into_value();
        result.push(value);
        Refined::new_unchecked(result)
    }
}

impl<T> Add for NonEmptyVec<T> {
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
    use crate::rule::non_empty::NonEmptyVecRule;
    use crate::rule::{NonEmptyVec, Rule};

    #[test]
    fn test_non_empty_vec() {
        assert!(NonEmptyVecRule::validate(vec![1, 2, 3]).is_ok());
        assert!(NonEmptyVecRule::<u8>::validate(vec![]).is_err());
    }

    #[test]
    fn test_add_vec() -> Result<(), Error<Vec<i32>>> {
        let ne_vec_1 = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec_2 = NonEmptyVec::new(vec![4, 5, 6])?;
        let ne_vec = ne_vec_1 + ne_vec_2;
        assert_eq!(ne_vec.into_value(), vec![1, 2, 3, 4, 5, 6]);
        Ok(())
    }

    #[test]
    fn test_into_iter() -> Result<(), Error<Vec<i32>>> {
        let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec: NonEmptyVec<i32> = ne_vec.into_iter().map(|n| n * 2).map(|n| n * 3).collect();
        assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
        Ok(())
    }

    #[test]
    fn test_iter() -> Result<(), Error<Vec<i32>>> {
        let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec: NonEmptyVec<i32> = ne_vec.iter().map(|n| n * 2).map(|n| n * 3).collect();
        assert_eq!(ne_vec.into_value(), vec![6, 12, 18]);
        Ok(())
    }

    #[test]
    fn test_collect_to_deque() -> Result<(), Error<Vec<i32>>> {
        let ne_vec = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec: NonEmptyVecDeque<i32> = ne_vec.into_iter().collect();
        assert_eq!(ne_vec.into_value(), vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_push() -> Result<(), Error<Vec<i32>>> {
        let vec = NonEmptyVec::new(vec![1])?.push(2).push(3);
        assert_eq!(vec.into_value(), vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), Error<Vec<i32>>> {
        let vec = NonEmptyVec::new(vec![1])?;
        assert_eq!(vec.get(0), Some(&1));
        Ok(())
    }

    #[test]
    fn test_len() -> Result<(), Error<Vec<i32>>> {
        let vec = NonEmptyVec::new(vec![1])?;
        assert_eq!(vec.len(), 1);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<(), Error<Vec<i32>>> {
        let vec = NonEmptyVec::new(vec![1])?;
        assert!(!vec.is_empty());
        Ok(())
    }
}
