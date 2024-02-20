use crate::rule::closed_algebraic::ClosedAlgebraic;
use crate::rule::NonEmpty;
use crate::Refined;
use std::fmt::Debug;

pub type NonEmptyVec<T> = Refined<NonEmptyVecRule<T>>;
pub type NonEmptyVecRule<T> = NonEmpty<Vec<T>>;

/// # Math Theory
/// NonEmptyVec + NonEmptyVec = NonEmptyVec
impl<T> ClosedAlgebraic for NonEmptyVec<T>
where
    T: Debug,
{
    fn plus(self, that: NonEmptyVec<T>) -> NonEmptyVec<T> {
        let mut result = self.into_value();
        result.append(&mut that.into_value());
        Refined::new(result).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::rule::non_empty::NonEmptyVecRule;
    use crate::rule::Rule;

    #[test]
    fn test_non_empty_vec() {
        assert!(NonEmptyVecRule::validate(vec![1, 2, 3]).is_ok());
        assert!(NonEmptyVecRule::<u8>::validate(vec![]).is_err());
    }
}
