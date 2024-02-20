macro_rules! more_rule {
    ($t: ty) => {
        paste::item! {
            ///
            pub type [<More $t:upper>] = $crate::Refined<[<More $t:upper Rule>], $t>;
            pub struct [<More $t:upper Rule>] {
                than: $t
            }
            impl [<More $t:upper Rule>] {
                pub fn new(than: $t) -> Self {
                    Self { than }
                }
            }
            impl $crate::rule::Rule for [<More $t:upper Rule>] {
                type Item = $t;

                fn validate(&self, target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if self.than <= target {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(format!("The input is not equal or more {}", self.than), target))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        more_rule! {$t}
        more_rule! {$($ts), +}
    };
}

// more_rule! {
//     i8,
//     i16,
//     i32,
//     i64,
//     i128,
//     isize,
//     u8,
//     u16,
//     u32,
//     u64,
//     u128,
//     usize,
//     f32,
//     f64
// }

#[cfg(test)]
mod test {
    // use crate::rule::number::more::MoreI8Rule;
    // use crate::rule::Rule;
    //
    // #[test]
    // fn test_more_i8() {
    //     let rule = MoreI8Rule::new(3);
    //     assert!(rule.validate(2).is_err());
    //     assert!(rule.validate(3).is_ok());
    //     assert!(rule.validate(4).is_ok());
    // }
}
