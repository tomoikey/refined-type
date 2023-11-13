use crate::rule::composer::Not;
use crate::rule::empty::Empty;
use crate::Refined;

pub type NonEmptyVec<'a, T> = Refined<NonEmptyVecRule<'a, T>, T>;
pub type NonEmptyVecRule<'a, T> = Not<'a, Vec<T>, Empty<Vec<T>>>;

#[cfg(test)]
mod test {
    use crate::rule::non_empty_vec::NonEmptyVecRule;
    use crate::rule::Rule;

    #[test]
    fn test_non_empty_vec() {
        let rule: NonEmptyVecRule<u8> = NonEmptyVecRule::default();

        assert!(rule.validate(vec![1, 2, 3]).is_ok());
        assert!(rule.validate(vec![]).is_err());
    }
}
