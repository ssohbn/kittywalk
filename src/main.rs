// white mouse:
// Bus 003 Device 029: ID 258a:0036 SINOWEALTH Wired Gaming Mouse

// https://wiki.osdev.org/USB_Human_Interface_Devices#USB_mouse

use hidapi; 

// this is all my mouseys >(._.)<

//const KT_MOUSE: (u16, u16)  = (0x093au16, 0x2510u16);
//const MODEL_O: (u16, u16) = (0x258Au16,0x0036u16);
const HP_MOUSE: (u16, u16) = (0x046du16, 0xc018u16);
const TOMAS: (u16, u16) = (0x258Au16, 0x1007u16);

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    // left mouse connection
    let (vid, pid) = TOMAS;
    let left = api.open(vid, pid).unwrap();

    // right mouse connection
//    let (vid, pid) = HP_MOUSE;
//    let right = api.open(vid, pid).unwrap();

    // Read data from device
    loop {
        let (ldx, ldy) = poll_device(&left);
        println!("ldx: {}, ldy: {}", ldx, ldy);

//        let (rdx, rdy) = poll_device(&right);
//        println!("rdx: {}, rdy: {}", rdx, rdy);
    }
}

/// grab change in position from mouse
fn poll_device(device: &hidapi::HidDevice) -> (i8, i8) {
    let mut buf = [0u8; 4];
    let _res = device.read_timeout(&mut buf[..], 100).unwrap();

    (*buf.get(1).unwrap() as i8, *buf.get(2).unwrap() as i8)
}

