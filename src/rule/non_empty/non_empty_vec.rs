use crate::rule::NonEmpty;
use crate::Refined;

pub type NonEmptyVec<'a, T> = Refined<NonEmptyVecRule<'a, T>>;
pub type NonEmptyVecRule<'a, T> = NonEmpty<'a, Vec<T>>;

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
