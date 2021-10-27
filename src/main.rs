use common::ipaddr::IpAddress;
use common::macaddr::MacAddress;
use link::device::Device;
use std::io;
use std::str::FromStr;

fn main() {
    let mut input_buffer = String::new();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("stdin(): read_line error.");

    let mut inputs = input_buffer.trim().split_whitespace();

    let command = inputs.next().unwrap();
    let device_name = inputs.next().unwrap();

    match command {
        "send" => {
            let mut sender = Device::new(device_name);
            let dest_mac =
                MacAddress::from_str(inputs.next().unwrap()).unwrap();
            let message = String::from("Hello, how are you?");
            sender.send_frame(&message, 0x2333, &dest_mac).unwrap();
        }
        _ => {
            let mut receiver = Device::new(device_name);
            receiver.receive_poll();
        }
    }
}
