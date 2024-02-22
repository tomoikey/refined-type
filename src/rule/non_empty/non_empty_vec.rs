use crate::rule::NonEmptyRule;
use crate::Refined;

use std::ops::Add;

pub type NonEmptyVec<T> = Refined<NonEmptyVecRule<T>>;

impl<T> Add for NonEmptyVec<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.into_value();
        result.append(&mut rhs.into_value());
        Refined::new(result)
            .ok()
            .expect("This error is always unreachable")
    }
}

pub type NonEmptyVecRule<T> = NonEmptyRule<Vec<T>>;

#[cfg(test)]
mod test {
    use crate::rule::non_empty::NonEmptyVecRule;
    use crate::rule::{NonEmptyVec, Rule};

    #[test]
    fn test_non_empty_vec() {
        assert!(NonEmptyVecRule::validate(vec![1, 2, 3]).is_ok());
        assert!(NonEmptyVecRule::<u8>::validate(vec![]).is_err());
    }

    #[test]
    fn test_add_vec() -> anyhow::Result<()> {
        let ne_vec_1 = NonEmptyVec::new(vec![1, 2, 3])?;
        let ne_vec_2 = NonEmptyVec::new(vec![4, 5, 6])?;
        let ne_vec = ne_vec_1 + ne_vec_2;
        assert_eq!(ne_vec.into_value(), vec![1, 2, 3, 4, 5, 6]);
        Ok(())
    }
}
