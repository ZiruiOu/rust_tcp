use std::cmp::PartialEq;
use std::convert::TryInto;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct IpAddress {
    address: [u8; 4],
}

#[allow(dead_code)]
impl IpAddress {
    pub fn new() -> Self {
        IpAddress { address: [0; 4] }
    }

    pub fn as_slice(&self) -> &[u8] {
        &(*self).address
    }

    pub fn is_broadcast(&self) -> bool {
        *self
            == IpAddress {
                address: [0xff, 0xff, 0xff, 0xff],
            }
    }
}

impl FromStr for IpAddress {
    type Err = ParseIntError;
    fn from_str(ip: &str) -> Result<Self, Self::Err> {
        let result: [u8; 4] = ip
            .split('.')
            .map(|byte| {
                u8::from_str_radix(byte, 10)
                    .expect("IpAddress: from_str_radix error.")
            })
            .collect::<Vec<u8>>()
            .try_into()
            .expect("IpAddress: cannot convert to 4-bytes array.");

        Ok(IpAddress { address: result })
    }
}

impl From<[u8; 4]> for IpAddress {
    fn from(ip: [u8; 4]) -> Self {
        IpAddress { address: ip }
    }
}

impl<'a> From<&'a [u8]> for IpAddress {
    fn from(ip: &'a [u8]) -> Self {
        IpAddress {
            address: [ip[0], ip[1], ip[2], ip[3]],
        }
    }
}

impl<'a> From<&'a [u8; 4]> for IpAddress {
    fn from(ip: &'a [u8; 4]) -> Self {
        IpAddress {
            address: ip.clone(),
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            (*self).address[0],
            (*self).address[1],
            (*self).address[2],
            (*self).address[3]
        )
    }
}

impl PartialEq for IpAddress {
    fn eq(&self, other: &IpAddress) -> bool {
        for i in 0..4 {
            if self.address[i] != other.address[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let addr = "127.0.0.1";
        let ip_addr = IpAddress::from_str(addr).unwrap();

        let result = format!("{}", ip_addr);
        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_4_bytes_array() {
        let addr = [127, 0, 0, 1];
        let ip_addr = IpAddress::from(addr);

        let expect = "127.0.0.1";
        let result = format!("{}", ip_addr);

        assert_eq!(expect, result);
    }

    #[test]
    fn test_from_4_bytes_array_ref() {
        let addr = [127, 0, 0, 1];
        let ip_addr = IpAddress::from(&addr);

        let expect = "127.0.0.1";
        let result = format!("{}", ip_addr);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_from_array_ref() {
        let addr = vec![127, 0, 0, 1];
        let ip_addr = IpAddress::from(addr.as_slice());

        let expect = "127.0.0.1";
        let result = format!("{}", ip_addr);
        assert_eq!(expect, result);
    }

    #[test]
    fn test_equal() {
        let addr1 = IpAddress::from_str("127.0.0.1").unwrap();
        let addr2 = IpAddress::from([127, 0, 0, 1]);
        let addr3 = IpAddress::from([127, 127, 127, 127]);

        assert_eq!(addr1 == addr2, true);
        assert_eq!(addr1 == addr3, false);
    }

    #[test]
    fn test_is_broadcast() {
        let addr1 = IpAddress::from_str("127.0.0.1").unwrap();
        let addr2 = IpAddress::from_str("255.255.255.255").unwrap();

        assert_eq!(addr1.is_broadcast(), false);
        assert_eq!(addr2.is_broadcast(), true);
    }
}
