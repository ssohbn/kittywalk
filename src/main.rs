use hidapi; 
use std::sync::mpsc;
use std::thread;

// this is all my mouseys >(._.)<

const KT_MOUSE: (u16, u16)  = (0x093au16, 0x2510u16);
//const MODEL_O: (u16, u16) = (0x258Au16,0x0036u16);
const HP_MOUSE: (u16, u16) = (0x046du16, 0xc018u16);
// const TOMAS: (u16, u16) = (0x258Au16, 0x1007u16); // this mouse was weird and sent data as i16
                                                  // instead of i8 so ill probably have to like u
                                                  // know do something about that

fn main() {
    let (send, receive) = mpsc::channel();

    // open connected usb mouse devices
    let api = hidapi::HidApi::new().unwrap();

    let left = api.open(KT_MOUSE.0, KT_MOUSE.1);
    start_mouse_thread(left, send.clone(), Foot::LEFT);

    let right = api.open(HP_MOUSE.0, HP_MOUSE.1);
    start_mouse_thread(right, send.clone(), Foot::RIGHT);

    loop {
        let res = receive.recv().unwrap();
        println!("res: {:?}", res);

    }
}

fn start_mouse_thread(device_result: hidapi::HidResult<hidapi::HidDevice>, sender: mpsc::Sender<MouseData>, foot: Foot) {

    // early return if mouse connecting messed up
    if !device_result.is_ok() {
        eprintln!("failed to connect to mouse");
        return
    }

    let right = device_result.unwrap();
    // left mouse thread
    thread::spawn(move || {
        loop {
            let (rdx, rdy) = poll_device(&right);
            sender.send(MouseData::new(rdx, rdy, foot)).expect("rightcould not send data");
        }
    });


}

/// grab change in position from mouse
fn poll_device(device: &hidapi::HidDevice) -> (i8, i8) {
    let mut buf = [0u8; 4];
    device.read(&mut buf[..]).unwrap();

    (*buf.get(1).unwrap() as i8, *buf.get(2).unwrap() as i8)
}

// never thought id write this in code
#[derive(Debug, Clone, Copy)]
enum Foot {
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct MouseData {
    x_movement: i8,
    y_movement: i8,
    foot: Foot,
}

impl MouseData {
    fn new(x_movement: i8, y_movement: i8, foot: Foot) -> MouseData {
        MouseData {
            x_movement,
            y_movement,
            foot,
        }
    }
}
