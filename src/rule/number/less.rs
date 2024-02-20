macro_rules! less_rule {
    ($t: ty) => {
        paste::item! {
            ///
            pub type [<Less $t:upper>] = $crate::Refined<[<Less $t:upper Rule>], $t>;
            pub struct [<Less $t:upper Rule>] {
                than: $t
            }
            impl [<Less $t:upper Rule>] {
                pub fn new(than: $t) -> Self {
                    Self { than }
                }
            }
            impl $crate::rule::Rule for [<Less $t:upper Rule>] {
                type Item = $t;

                fn validate(&self, target: Self::Item) -> Result<Self::Item, $crate::result::Error<Self::Item>> {
                    if target <= self.than {
                        Ok(target)
                    } else {
                        Err($crate::result::Error::new(format!("The input is not equal or less than {}", self.than), target))
                    }
                }
            }
        }
    };
    ($t: ty, $($ts: ty),+) => {
        less_rule! {$t}
        less_rule! {$($ts), +}
    };
}

// less_rule! {
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
    // use crate::rule::number::less::LessI8Rule;
    // use crate::rule::Rule;

    // #[test]
    // fn test_less_i8() {
    //     let rule = LessI8Rule::new(5);
    //     assert!(rule.validate(6).is_err());
    //     assert!(rule.validate(5).is_ok());
    //     assert!(rule.validate(-3).is_ok());
    // }
}
