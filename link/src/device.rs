use super::common::ipaddrs::IpAddress;
use super::common::macaddr::MacAddress;
use pcap::{Capture, Device};
use pnet::datalink::{self, NetworkInterface};

pub struct Device {
    device_name: String,
    pcap_handler: Capture<Active>,
    mac_addr: MacAddress,
    ip_addr: IpAddress,
}

impl Device {
    pub fn new(name: &str) -> Self {
        let mut handler = Capture::from_device(name)
            .expect(format!("Device: device {} not exists.", name))
            .promisc(true)
            .immediate_mode(true)
            .snaplen(6000)
            .open()
            .expect(format!(
                "Device: device {} cannot initialize handler.",
                name
            ));

        let mut handler = handler
            .setnonblock()
            .expect("Device: sent non-block error.");

        let mut mac_address = MacAddress::new();
        Device::init_mac_address(&mut mac_address);

        let mut ip_address = IpAddress::new();
        Device::init_ip_address(&mut ip_address);

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
                mac_addr = MacAddress::from(address.octets());
            }
            None => {
                panic!("init_mac_address: device mac address doesn't exist.");
            }
        }
    }

    fn init_ip_address(name: &str, ip_addr: &mut IpAddress) {
        // naive implementation using utilities.
        let netif = datalink::interfaces()
            .filter(|iface: &NetworkInterface| iface.name == name)
            .next()
            .expect("init_ip_address: device nnot found.");

        for network in netif.ips.iter() {
            if let V4(ipv4_network) = network {
                ip_addr = IpAddress::from(ipv4_network.ip().octets());
                return;
            }
        }
    }
}
