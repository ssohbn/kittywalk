// white mouse:
// Bus 003 Device 029: ID 258a:0036 SINOWEALTH Wired Gaming Mouse

// https://wiki.osdev.org/USB_Human_Interface_Devices#USB_mouse

fn main() {
    let trinket = get_tinky().expect("wuh oh device not found");

    let handle = trinket.open().expect("couldnt get handle");

    let mut buffer: [u8; 4] = [0; 4];

    // handle
    //     .read_control(
    //         rusb::request_type(rusb::Direction::In, rusb::RequestType::Class, rusb::Recipient::Interface),
    //         rusb::RequestType,
    //         ,
    //         ,
    //         &mut buffer,
    //         std::time::Duration::new(1, 0),
    //         );
    let data = handle.read_interrupt(0, &mut buffer, std::time::Duration::new(1,0)).unwrap();
    println!("{}", data);
    for byte in buffer {
        print!("{}", byte);
    }
    println!();
}

fn get_tinky() -> Option<rusb::Device<rusb::GlobalContext>> {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if (device_desc.vendor_id(), device_desc.product_id()) == (0x258au16, 0x0036u16) {
            println!("found device!");

            return Some(device);
        };
    }

    None
}
