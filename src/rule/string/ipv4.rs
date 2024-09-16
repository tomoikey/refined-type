use crate::result::Error;
use crate::rule::Rule;
use crate::Refined;
use std::str::FromStr;

/// A type that holds a value satisfying the `Ipv4AddrRule`
pub type Ipv4Addr<STRING> = Refined<Ipv4AddrRule<STRING>>;

/// Rule where the target value must be a valid IPv4 address
pub struct Ipv4AddrRule<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: AsRef<str>> Rule for Ipv4AddrRule<T> {
    type Item = T;

    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        let target_as_ref = target.as_ref();
        if std::net::Ipv4Addr::from_str(target_as_ref).is_ok() {
            Ok(target)
        } else {
            let message = format!("{} is not a valid IPv4 address", target_as_ref);
            Err(Error::new(target, message))
        }
    }
}

/// A type that holds a value satisfying the `PublicIpv4AddrRule`
pub type PublicIpv4Addr<T> = Refined<PublicIpv4AddrRule<T>>;

pub struct PublicIpv4AddrRule<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: AsRef<str>> Rule for PublicIpv4AddrRule<T> {
    type Item = T;

    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        let target_as_ref = target.as_ref();
        let ipv4_result = std::net::Ipv4Addr::from_str(target_as_ref);
        if let Ok(ipv4) = ipv4_result {
            if !ipv4.is_private() {
                Ok(target)
            } else {
                let message = format!("{} is a private IP address", target_as_ref);
                Err(Error::new(target, message))
            }
        } else {
            let message = format!("{} is not a valid IPv4 address", target_as_ref);
            Err(Error::new(target, message))
        }
    }
}

/// A type that holds a value satisfying the `PrivateIpv4Rule`
pub type PrivateIpv4Addr<T> = Refined<PrivateIpv4AddrRule<T>>;

pub struct PrivateIpv4AddrRule<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: AsRef<str>> Rule for PrivateIpv4AddrRule<T> {
    type Item = T;

    fn validate(target: Self::Item) -> crate::Result<Self::Item> {
        let target_as_ref = target.as_ref();
        if let Ok(ipv4) = std::net::Ipv4Addr::from_str(target_as_ref) {
            if ipv4.is_private() {
                Ok(target)
            } else {
                let message = format!("{} is a public IP address", target_as_ref);
                Err(Error::new(target, message))
            }
        } else {
            let message = format!("{} is not a valid IPv4 address", target_as_ref);
            Err(Error::new(target, message))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::string::ipv4::PrivateIpv4AddrRule;
    use crate::rule::{Ipv4AddrRule, PublicIpv4AddrRule, Rule};

    #[test]
    fn test_valid_ipv4() {
        let valid = String::from("100.0.0.1");
        assert!(Ipv4AddrRule::validate(&valid).is_ok());

        let valid = "192.168.0.1";
        assert!(Ipv4AddrRule::validate(&valid).is_ok());
    }

    #[test]
    fn test_invalid_ipv4() {
        let invalid = String::from("256.0.0.1");
        assert!(Ipv4AddrRule::validate(&invalid).is_err());

        let invalid = String::from("127.0.0.256");
        assert!(Ipv4AddrRule::validate(&invalid).is_err());
    }

    #[test]
    fn test_valid_public_ipv4() {
        let valid = String::from("100.0.0.1");
        assert!(PublicIpv4AddrRule::validate(&valid).is_ok());
    }

    #[test]
    fn test_invalid_public_ipv4() {
        let invalid = String::from("192.168.0.1");
        assert!(PublicIpv4AddrRule::validate(&invalid).is_err());
    }

    #[test]
    fn test_valid_private_ipv4() {
        let valid = String::from("10.0.0.0");
        assert!(PrivateIpv4AddrRule::validate(&valid).is_ok());

        let valid = "172.16.0.0";
        assert!(PrivateIpv4AddrRule::validate(&valid).is_ok());

        let valid = "192.168.0.0";
        assert!(PrivateIpv4AddrRule::validate(&valid).is_ok())
    }

    #[test]
    fn test_invalid_private_ipv4() {
        let invalid = String::from("256.0.0.1");
        assert!(PrivateIpv4AddrRule::validate(&invalid).is_err());

        let invalid = String::from("127.0.0.256");
        assert!(PrivateIpv4AddrRule::validate(&invalid).is_err());

        let invalid = String::from("127.0.0.1");
        assert!(PrivateIpv4AddrRule::validate(&invalid).is_err());

        let invalid = String::from("10.0.0.1:9000");
        assert!(PrivateIpv4AddrRule::validate(&invalid).is_err())
    }
}
