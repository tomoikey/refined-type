macro_rules! define_min_max_rule {
    ($t: ty) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the `MinMaxRule`
            pub type [<MinMax $t:camel>]<const MIN: $t, const MAX: $t> = $crate::Refined<[<MinMaxRule $t:camel>]<MIN, MAX>>;

            /// Rule where the target value must be greater than or equal to `MIN` and less than or equal to `MAX`
            pub type [<MinMaxRule $t:camel>]<const MIN: $t, const MAX: $t> = $crate::Or![
                $crate::rule::[<RangeRule $t:camel>]<MIN, MAX>,
                $crate::rule::[<EqualRule $t:camel>]<MAX>
            ];
        }
    };
    ($t: ty, $($ts: ty),+) => {
        define_min_max_rule!($t);
        define_min_max_rule!($($ts), +);
    };
}

define_min_max_rule!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod test {
    use crate::rule::MinMaxI8;

    #[test]
    fn test_min_max_i8_ok() {
        let min_max_result = MinMaxI8::<1, 10>::new(5);
        assert!(min_max_result.is_ok());
    }

    #[test]
    fn test_min_max_i8_err() {
        let min_max_result = MinMaxI8::<1, 10>::new(100);
        assert!(min_max_result.is_err());
    }
}
