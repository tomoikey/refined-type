use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use std::str::FromStr;

/// A type that holds a value satisfying the `Ipv6AddrRule`
pub type Ipv6Addr<STRING> = Refined<Ipv6AddrRule<STRING>>;

/// Rule where the target value must be a valid IPv6 address
pub struct Ipv6AddrRule<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: AsRef<str>> Rule for Ipv6AddrRule<T> {
    type Item = T;

    fn validate(target: Self::Item) -> Result<Self::Item, Error<Self::Item>> {
        let target_as_ref = target.as_ref();
        if std::net::Ipv6Addr::from_str(target_as_ref).is_ok() {
            Ok(target)
        } else {
            let message = format!("{} is not a valid IPv6 address", target_as_ref);
            Err(Error::new(target, message))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::{Ipv6AddrRule, Rule};

    #[test]
    fn test_valid_ipv6() {
        let valid_addresses = vec![
            "2001:db8:3333:4444:5555:6666:7777:8888",
            "2001:db8:3333:4444:CCCC:DDDD:EEEE:FFFF",
            "::",
            "2001:db8::",
            "::1234:5678",
            "2001:db8::1234:5678",
            "2001:0db8:0001:0000:0000:0ab9:C0A8:0102",
            "2001:db8:1::ab9:C0A8:102",
        ];

        for addr in valid_addresses {
            assert!(
                Ipv6AddrRule::validate(&addr).is_ok(),
                "{} should be valid",
                addr
            );
        }
    }

    #[test]
    fn test_invalid_ipv6() {
        let invalid_addresses = vec![
            "2001:db8:3333:4444:5555:6666:7777:8888:9999", // Too many segments
            "2001:db8:3333:4444:5555:6666:7777",           // Too few segments
            "2001:db8:3333:4444:5555:6666:7777:gggg",      // Invalid characters
            "2001:db8:3333:4444:5555:6666:7777:88888",     // Segment too long
            "2001:db8:3333:4444:5555:6666:7777:8888:",     // Trailing colon
            ":2001:db8:3333:4444:5555:6666:7777:8888",     // Leading colon
            "2001:db8:3333:4444:5555:6666:7777:8888::",    // Double colon in wrong place
            "::2001:db8:3333:4444:5555:6666:7777:8888",    // Double colon in wrong place
            "2001:db8::3333::4444:5555:6666:7777:8888",    // Multiple double colons
            "2001:db8:3333:4444:5555:6666:7777:8888/64",   // CIDR notation not allowed
            "2001:db8:3333:4444:5555:6666:7777:8888/129",  // Invalid CIDR prefix length
            "2001:db8:3333:4444:5555:6666:7777:8888%eth0", // Zone index not allowed
        ];

        for addr in invalid_addresses {
            assert!(
                Ipv6AddrRule::validate(&addr).is_err(),
                "{} should be invalid",
                addr
            );
        }
    }
}
