use hidapi;
use std::sync::mpsc;
use std::thread;

use std::io::prelude::*;
use std::net::TcpStream;

use bytemuck;

// this is all my mouseys
//🐁🐁🐁🐁🐁

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

    let mut stream = TcpStream::connect("127.0.0.1:1300").unwrap(); // eh ill do something more
                                                                    // secret for this ip stuff
                                                                    // later

    loop {
        let res = receive.recv().unwrap();
        println!("res: {:?}", res);

        let bytes = bytemuck::bytes_of(&res);
        println!("{:#?}", bytes);

        stream.write(bytes).expect("stream write fail roflsauce");
    }
}

fn start_mouse_thread(device_result: hidapi::HidResult<hidapi::HidDevice>, sender: mpsc::Sender<MouseData>, foot: Foot) {
    // early return if mouse connecting messed up
    if !device_result.is_ok() {
        eprintln!("failed to connect to mouse");
        return
    }

    let device = device_result.unwrap();
    // left mouse thread
    thread::spawn(move || {
        loop {
            let (dx, dy) = poll_device(&device);
            sender.send(MouseData::new(dx, dy, foot)).expect("rightcould not send data");
        }
    });
}

// grab change in position from mouse
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

#[derive(Debug, Copy, Clone)]
struct MouseData {
    x_movement: i8,
    y_movement: i8,
    foot: Foot, // yeah i dont think this should be in a normal mouse struct...
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

// completely honest. i do not know what these do. I read the docs and I should be able to do this
// and it worked in my test program but this might be a headache later :)
unsafe impl bytemuck::Pod for MouseData {}
unsafe impl bytemuck::Zeroable for MouseData {}

