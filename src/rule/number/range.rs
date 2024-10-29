macro_rules! define_range_rule {
    ($t: ty) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the `RangeRule`
            pub type [<Range $t:camel>]<const FROM: $t, const UNTIL: $t> = $crate::Refined<[<RangeRule $t:camel>]<FROM, UNTIL>>;

            /// Rule where the target value must be greater than or equal to `FROM` and less than `UNTIL`
            pub type [<RangeRule $t:camel>]<const FROM: $t, const UNTIL: $t> = $crate::And![
                $crate::rule::[<GreaterEqualRule $t:camel>]<FROM>,
                $crate::rule::[<LessRule $t:camel>]<UNTIL>
            ];
        }
    };
    ($t: ty, $($ts: ty),+) => {
        define_range_rule!($t);
        define_range_rule!($($ts), +);
    };
}

define_range_rule!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod test {
    use crate::rule::RangeI8;

    #[test]
    fn test_range_i8_ok() {
        let range_result = RangeI8::<1, 10>::new(0);
        assert!(range_result.is_err());

        let range_result = RangeI8::<1, 10>::new(1);
        assert!(range_result.is_ok());

        let range_result = RangeI8::<1, 10>::new(10);
        assert!(range_result.is_err());
    }

    #[test]
    fn test_range_i8_err() {
        let range_result = RangeI8::<1, 10>::new(-1);
        assert!(range_result.is_err());

        let range_result = RangeI8::<1, 10>::new(11);
        assert!(range_result.is_err());
    }
}
