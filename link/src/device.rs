use common::ipaddr::IpAddress;
use common::macaddr::MacAddress;
use core::result::Result;
use pcap::{self, Active, Capture};
use pnet::datalink::{self, NetworkInterface};
use pnet::ipnetwork::IpNetwork;

#[allow(dead_code)]
pub struct Device {
    device_name: String,
    pcap_handler: Capture<Active>,
    mac_addr: MacAddress,
    ip_addr: IpAddress,
}

#[allow(dead_code)]
impl Device {
    pub fn new(name: &str) -> Self {
        let mut handler = Capture::from_device(name)
            .expect(format!("Device: device {} not exists.", name).as_str())
            .promisc(true)
            .immediate_mode(true)
            .snaplen(6000)
            .open()
            .expect(
                format!("Device: device {:?} cannot initialize handler.", name)
                    .as_str(),
            );

        let mut handler = handler
            .setnonblock()
            .expect("Device: sent non-block error.");

        let mut mac_address = MacAddress::new();
        Device::init_mac_address(name, &mut mac_address);

        let mut ip_address = IpAddress::new();
        Device::init_ip_address(name, &mut ip_address);

        Device {
            device_name: String::from(name),
            pcap_handler: handler,
            mac_addr: mac_address,
            ip_addr: ip_address,
        }
    }

    fn init_mac_address(name: &str, mac_addr: &mut MacAddress) {
        let netif = datalink::interfaces()
            .into_iter()
            .filter(|iface: &NetworkInterface| iface.name == name)
            .next()
            .expect("init_mac_address: device not found.");

        match netif.mac {
            Some(address) => {
                *mac_addr = MacAddress::from(address.octets());
            }
            None => {
                panic!("init_mac_address: device mac address doesn't exist.");
            }
        }
    }

    fn init_ip_address(name: &str, ip_addr: &mut IpAddress) {
        // naive implementation using utilities.
        let netif = datalink::interfaces()
            .into_iter()
            .filter(|iface: &NetworkInterface| iface.name == name)
            .next()
            .expect("init_ip_address: device nnot found.");

        for network in netif.ips.iter() {
            if let IpNetwork::V4(ipv4_network) = network {
                *ip_addr = IpAddress::from(ipv4_network.ip().octets());
                return;
            }
        }

        panic!("init_ip_address: device ip address doesn't exist.");
    }

    pub fn send_frame(
        &mut self,
        message: &String,
        ethernet_type: u16,
        dest_mac: &MacAddress,
    ) -> Result<(), pcap::Error> {
        let buffer = Device::create_frame(
            &(*self).mac_addr,
            dest_mac,
            ethernet_type,
            message,
        );
        (*self).pcap_handler.sendpacket(buffer.as_slice())?;
        Ok(())
    }

    fn create_frame(
        src_mac: &MacAddress,
        dest_mac: &MacAddress,
        ethernet_type: u16,
        message: &String,
    ) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        let mut type_buffer = u16::to_be_bytes(ethernet_type);

        buffer.extend_from_slice(dest_mac.as_slice());
        buffer.extend_from_slice(src_mac.as_slice());
        buffer.extend_from_slice(&mut type_buffer);
        buffer.extend_from_slice((*message).as_bytes());

        buffer
    }

    pub fn receive_poll(&mut self) {
        loop {
            while let Ok(packet) = (*self).pcap_handler.next() {
                println!("{:?}", packet);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_device() {
        let device_name = "veth1-2";
        let device = Device::new(device_name);
    }
}
