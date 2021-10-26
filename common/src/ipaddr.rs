#[derive(Debug)]
pub struct IpAddress {
    address: [u8; 4],
}

#[allow(dead_code)]
impl IpAddress {
    pub fn new() -> Self {
        IpAddress { address: [0; 4] }
    }
}
