use crate::{Error, Rule};

struct MinMaxI32Rule {
    min: i32,
    max: i32,
}

impl MinMaxI32Rule {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl Rule for MinMaxI32Rule {
    type Item = i32;

    fn validate(&self, target: Self::Item) -> crate::Result<Self::Item> {
        if self.min <= target && target <= self.max {
            Ok(target)
        } else {
            Err(Error::new(format!(
                "The input `i32` is not between {} and {}",
                self.min, self.max
            )))
        }
    }
}
