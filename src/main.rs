// white mouse:
// Bus 003 Device 029: ID 258a:0036 SINOWEALTH Wired Gaming Mouse

// https://wiki.osdev.org/USB_Human_Interface_Devices#USB_mouse

use hidapi; 
fn main() {

    let api = hidapi::HidApi::new().unwrap();

    // Print out information about all connected devices
    for device in api.device_list() {
        println!("{:#?}", device);
    }

    let mouse = (0x046du16, 0xc018u16);
    let (VID, PID) = mouse;

    // Connect to device using its VID and PID
    let device = api.open(VID, PID).unwrap();

    // Read data from device
    let mut buf = [0u8; 8];
    loop {
        let res = device.read(&mut buf[..]).unwrap();
        println!("Read: {:?}", &buf[..res]);
    }

    // Write data to device
    // let buf = [0u8, 1, 2, 3, 4];
    // let res = device.write(&buf).unwrap();
    // println!("Wrote: {:?} byte(s)", res);
}
