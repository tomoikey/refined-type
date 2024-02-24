use crate::rule::EmptyDefinition;

macro_rules! empty_definition {
    ($t:ty) => {
        impl $crate::rule::EmptyDefinition for $t {
            fn empty(&self) -> bool {
                *self == 0
            }
        }
    };
    ($t:ty, $($ts:ty), +) => {
        empty_definition!($t);
        empty_definition!($($ts), +);
    };
}

empty_definition!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl EmptyDefinition for f32 {
    fn empty(&self) -> bool {
        *self == 0f32
    }
}

impl EmptyDefinition for f64 {
    fn empty(&self) -> bool {
        *self == 0f64
    }
}
