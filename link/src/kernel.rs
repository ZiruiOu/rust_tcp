use super::device::Device;
use common::macaddr::MacAddress;

type FrameReceiveCallback = fn(&[u8], usize) -> Result<(), pcap::Error>;

#[allow(dead_code)]
pub struct Kernel {
    devices: Vec<Device>,
    callback: Option<FrameReceiveCallback>,
}

#[allow(dead_code)]
impl Kernel {
    pub fn new() -> Self {
        Kernel {
            devices: Vec::<Device>::new(),
            callback: None,
        }
    }

    pub fn add_device(&mut self, device_name: &str) -> usize {
        (*self).devices.push(Device::new(device_name));
        return (*self).devices.len() - 1;
    }

    pub fn find_device(&self, device_name: &str) -> Option<usize> {
        for (index, device) in (*self).devices.iter().enumerate() {
            if (*device).device_name() == device_name {
                return Some(index);
            }
        }
        return None;
    }

    pub fn get_device(&mut self, device_name: &str) -> Option<&mut Device> {
        for device in (*self).devices.iter_mut() {
            if device.device_name() == device_name {
                return Some(device);
            }
        }
        None
    }

    pub fn set_callback(&mut self, callback: FrameReceiveCallback) {
        (*self).callback = Some(callback);
    }

    pub fn send_frame(
        &mut self,
        message: &String,
        ethernet_type: u16,
        dest_mac: &MacAddress,
        id: usize,
    ) -> Result<(), pcap::Error> {
        match (*self).devices.get_mut(id) {
            Some(sender) => {
                sender.send_frame(message, ethernet_type, dest_mac)?;
                Ok(())
            }
            None => Err(pcap::Error::PcapError(format!(
                "kernel: device id {} doesn't exist.",
                id
            ))),
        }
    }

    pub fn receive_poll(&mut self) -> Result<(), pcap::Error> {
        loop {
            let device_list = &mut (*self).devices;
            let callback = &(*self).callback;
            for device in (*device_list).iter_mut() {
                if let Some(ref handler) = callback {
                    device.execute_callback(handler)?;
                }
            }
        }
    }
}
