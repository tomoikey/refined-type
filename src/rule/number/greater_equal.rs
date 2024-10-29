macro_rules! declare_greater_equal_rule {
    ($ty: ty) => {
        $crate::paste::item! {
            /// A type that holds a value satisfying the `GreaterEqualRule`
            pub type [<GreaterEqual $ty:camel>]<const N: $ty> = $crate::Refined<[<GreaterEqualRule $ty:camel>]<N>>;

            /// Rule where the target value must be greater than or equal to `N`
            pub type [<GreaterEqualRule $ty:camel>]<const N: $ty> = $crate::Or![$crate::rule::[<EqualRule $ty:camel>]<N>, $crate::rule::[<GreaterRule $ty:camel>]<N>];
        }
    };
    ($t: ty, $($ts: ty),+) => {
        declare_greater_equal_rule!($t);
        declare_greater_equal_rule!($($ts), +);
    };
}

declare_greater_equal_rule!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod test {
    use crate::rule::GreaterEqualU8;

    #[test]
    fn test_greater_equal_than_50u8_ok() {
        let greater_equal_result = GreaterEqualU8::<50>::new(50);
        assert!(greater_equal_result.is_ok());
    }

    #[test]
    fn test_greater_equal_than_50u8_err() {
        let greater_equal_result = GreaterEqualU8::<50>::new(49);
        assert!(greater_equal_result.is_err());
    }
}
