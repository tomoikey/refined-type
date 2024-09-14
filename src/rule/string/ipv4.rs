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

    fn validate(target: &Self::Item) -> Result<(), Error> {
        let target = target.as_ref();
        if std::net::Ipv4Addr::from_str(target).is_ok() {
            Ok(())
        } else {
            Err(Error::new(format!(
                "{} is not a valid IPv4 address",
                target
            )))
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

    fn validate(target: &Self::Item) -> Result<(), Error> {
        let target = target.as_ref();
        if std::net::Ipv4Addr::from_str(target)
            .map_err(|e| Error::new(e.to_string()))?
            .is_private()
        {
            Err(Error::new(format!("{} is a private IP address", target)))
        } else {
            Ok(())
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

    fn validate(target: &Self::Item) -> Result<(), Error> {
        let target = target.as_ref();
        if std::net::Ipv4Addr::from_str(target)
            .map_err(|e| Error::new(e.to_string()))?
            .is_private()
        {
            Ok(())
        } else {
            Err(Error::new(format!("{} is a public IP address", target)))
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
