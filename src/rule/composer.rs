mod and;
mod not;
mod or;

pub use and::And;
pub use not::Not;
pub use or::Or;

#[cfg(test)]
mod test {
    #[test]
    fn test_and_or_not() {}
}
