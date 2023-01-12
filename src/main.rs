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

    // left mouse connection
    let kt_mouse = (0x093au16, 0x2510u16);
    let (vid, pid) = kt_mouse;
    let left = api.open(vid, pid).unwrap();

    // right mouse connection
    let hp_mouse = (0x046du16, 0xc018u16);
    let (vid, pid) = hp_mouse;
    let right = api.open(vid, pid).unwrap();

    // Read data from device
    loop {
        let mut buf = [0u8; 4];
        let res = left.read_timeout(&mut buf[..], 1).unwrap();
        let (left_x_change, left_y_change): (i8, i8) = (*buf.get(1).unwrap() as i8, *buf.get(2).unwrap() as i8);

        let mut buf = [0u8; 4];
        let res = right.read_timeout(&mut buf[..], 1).unwrap();
        let (right_x_change, right_y_change): (i8, i8) = (*buf.get(1).unwrap() as i8, *buf.get(2).unwrap() as i8);
        
        println!("ldx: {}, ldy: {}\nrdx: {}, rdy: {}", left_x_change, left_y_change, right_x_change, right_y_change);
    }

    // Write data to device
    // let buf = [0u8, 1, 2, 3, 4];
    // let res = device.write(&buf).unwrap();
    // println!("Wrote: {:?} byte(s)", res);
}
