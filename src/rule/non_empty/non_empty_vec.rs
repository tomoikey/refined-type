use crate::rule::NonEmptyRule;
use crate::Refined;

pub type NonEmptyVec<T> = Refined<NonEmptyVecRule<T>>;
pub type NonEmptyVecRule<T> = NonEmptyRule<Vec<T>>;

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
