use std::cmp::PartialEq;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct MacAddress {
    address: [u8; 6],
}

#[allow(dead_code)]
impl MacAddress {
    pub fn new() -> Self {
        MacAddress {
            address: [0 as u8; 6],
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        return &(*self).address;
    }

    pub fn is_broadcast(&self) -> bool {
        return *self == MacAddress { address: [0xff; 6] };
    }
}

// implementation From Trait for 4 situations
// 1. from &str
// 2. from String
// 3. from [u8; 6]
// 4. from &[u8]

impl FromStr for MacAddress {
    type Err = ParseIntError;

    fn from_str(mac: &str) -> Result<Self, Self::Err> {
        let result: [u8; 6] = mac
            .split(':')
            .map(|byte| {
                u8::from_str_radix(byte, 16)
                    .expect("MacAddress: from_str_radix error")
            })
            .collect::<Vec<u8>>()
            .try_into()
            .expect("MacAddress: cannot convert to 6-bytes array.");
        Ok(MacAddress { address: result })
    }
}

impl From<[u8; 6]> for MacAddress {
    fn from(addr: [u8; 6]) -> Self {
        MacAddress { address: addr }
    }
}

impl<'a> From<&'a [u8; 6]> for MacAddress {
    fn from(addr: &'a [u8; 6]) -> Self {
        MacAddress {
            address: (*addr).clone(),
        }
    }
}

impl<'a> From<&'a [u8]> for MacAddress {
    fn from(addr: &'a [u8]) -> Self {
        MacAddress {
            address: [addr[0], addr[1], addr[2], addr[3], addr[4], addr[5]],
        }
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.address[0],
            self.address[1],
            self.address[2],
            self.address[3],
            self.address[4],
            self.address[5]
        )
    }
}

impl PartialEq for MacAddress {
    fn eq(&self, other: &MacAddress) -> bool {
        for i in 0..6 {
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
    fn test_is_broadcast() {
        let addr1 = MacAddress::from_str("11:22:33:44:55:66").unwrap();
        assert_eq!(addr1.is_broadcast(), false);

        let addr2 = MacAddress::from_str("ff:ff:ff:ff:ff:ff").unwrap();
        assert_eq!(addr2.is_broadcast(), true);
    }

    #[test]
    fn test_from_str() {
        let mac = "11:22:33:44:55:66";
        let address = MacAddress::from_str(mac);

        let result = format!("{}", address.unwrap());
        assert_eq!(result, "11:22:33:44:55:66");
    }

    #[test]
    fn test_from_array() {
        let array1: Vec<u8> = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
        let addr1 = MacAddress::from(array1.as_slice());

        let result = format!("{}", addr1);
        assert_eq!(result, String::from("11:22:33:44:55:66"));
    }
}
