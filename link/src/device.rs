use super::common::ipaddrs::IpAddress;
use super::common::macaddr::MacAddress;
use pcap::{Capture, Device};

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

    // TODO: add definition for these two functions
    fn init_mac_address(name: &str, mac_addr: &mut MacAddress) {
        // naive implementation using utilities.
    }

    fn init_ip_address(name: &str, ip_addr: &mut IpAddress) {
        // naive implementation using utilities.
    }
}
