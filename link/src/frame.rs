use common::macaddr::MacAddress;
use std::convert::{From, TryInto};

#[derive(Debug)]
pub struct Frame {
    pub dest_mac: MacAddress,
    pub src_mac: MacAddress,
    pub ethernet_type: u16,
    pub message: Option<String>,
    pub checksum: u32,
}

impl Frame {
    pub fn new(
        dest_mac: &MacAddress,
        src_mac: &MacAddress,
        ethernet_type: u16,
        message: &String,
        checksum: u32,
    ) -> Self {
        Frame {
            dest_mac: MacAddress::from((*dest_mac).as_slice()),
            src_mac: MacAddress::from((*src_mac).as_slice()),
            ethernet_type: ethernet_type,
            message: Some(String::from((*message).as_str())),
            checksum: checksum,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        let type_buffer = u16::to_be_bytes((*self).ethernet_type);
        let checksum_buffer = u32::to_be_bytes((*self).checksum);

        buffer.extend_from_slice((*self).dest_mac.as_slice());
        buffer.extend_from_slice((*self).src_mac.as_slice());
        buffer.extend_from_slice(&type_buffer);
        if let Some(ref good_message) = (*self).message {
            buffer.extend_from_slice((*good_message).as_bytes())
        };
        buffer.extend_from_slice(&checksum_buffer);

        buffer
    }
}

impl From<&[u8]> for Frame {
    fn from(buffer: &[u8]) -> Self {
        let length = (*buffer).len();

        let dest_mac = MacAddress::from(&buffer[0..6]);
        let src_mac = MacAddress::from(&buffer[6..12]);
        let ethernet_type =
            u16::from_be_bytes(buffer[12..14].try_into().unwrap());
        let message = match String::from_utf8(buffer[14..length - 4].to_vec()) {
            Ok(good_message) => Some(good_message),
            _ => None,
        };
        let checksum =
            u32::from_be_bytes(buffer[length - 4..length].try_into().unwrap());

        Frame {
            dest_mac: dest_mac,
            src_mac: src_mac,
            ethernet_type: ethernet_type,
            message: message,
            checksum: checksum,
        }
    }
}
