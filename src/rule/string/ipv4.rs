use crate::{declare_regex_rule, Refined};

/// A type that holds a value satisfying the `Ipv4Rule`
pub type PrivateIpv4Addr<T> = Refined<PrivateIpv4AddrRule<T>>;

declare_regex_rule![
    pub PrivateIpv4AddrRule,
    r"^10\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$|^172\.(1[6-9]|2[0-9]|3[0-1])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$|^192\.168\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\.([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$"
];

#[cfg(test)]
mod tests {
    use crate::rule::string::ipv4::PrivateIpv4AddrRule;
    use crate::rule::Rule;

    #[test]
    fn test_valid_ipv4() {
        let valid = String::from("10.0.0.0");
        assert!(PrivateIpv4AddrRule::validate(&valid).is_ok());

        let valid = "172.16.0.0";
        assert!(PrivateIpv4AddrRule::validate(&valid).is_ok());

        let valid = "192.168.0.0";
        assert!(PrivateIpv4AddrRule::validate(&valid).is_ok())
    }

    #[test]
    fn test_invalid_ipv4() {
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
