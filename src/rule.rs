mod non_empty_string;

pub use non_empty_string::NonEmptyStringRule;

use anyhow::Result;

pub trait Rule {
    type TARGET;
    fn validate(target: Self::TARGET) -> Result<Self::TARGET>;
}
