use hidapi;
use std::fmt::write;
use std::sync::mpsc;
use std::thread;

use std::io::prelude::*;
use std::net::TcpStream;

use bytemuck;

use clap::Parser;

// this is all my mouseys
//🐁🐁🐁🐁🐁

//const KT_MOUSE: (u16, u16)  = (0x093au16, 0x2510u16);
//const MODEL_O: (u16, u16) = (0x258Au16,0x0036u16);
//const HP_MOUSE: (u16, u16) = (0x046du16, 0xc018u16);
//const MS_MOUSE: (u16, u16) = (0x045Eu16, 0x0040u16);
// const TOMAS: (u16, u16) = (0x258Au16, 0x1007u16); // this mouse was weird and sent data as i16
                                                  // instead of i8 so ill probably have to like u
                                                  // know do something about that
fn main() {

    let args = Args::parse();

    let left_mouse = parse_mouse_string(args.left_mouse);
    let right_mouse = parse_mouse_string(args.right_mouse);

    let (send, receive) = mpsc::channel();

    // open connected usb mouse devices
    let api = hidapi::HidApi::new().unwrap();

    let left = api.open(left_mouse.0, left_mouse.1);
    start_mouse_thread(left, send.clone(), Foot::LEFT, args.left_mouse_type);

    let right = api.open(right_mouse.0, right_mouse.1);
    start_mouse_thread(right, send.clone(), Foot::RIGHT, args.right_mouse_type);

    println!("trying to connect to {}", args.ip);

    loop {
        if let Ok(mut stream) = TcpStream::connect(args.ip.clone()) {
            loop {
                println!("----");
                let res = receive.recv().unwrap();
                println!("res: {:?}", res);

                let bytes = bytemuck::bytes_of(&res);
                println!("{:#?}", bytes);
                
                let x = ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[3] as u32) << 8) | (bytes[4] as u32);
                println!("{:#b}", x);
                println!("----");

                if let Err(_) = stream.write(bytes) {
                break;
                }
                // stream.write(bytes).expect("stream write fail roflsauce");
            };
        } else {
            continue;
        };
    }
}

fn start_mouse_thread(device_result: hidapi::HidResult<hidapi::HidDevice>, sender: mpsc::Sender<MouseData>, foot: Foot, mousetype: MouseType) {
    // early return if mouse connecting messed up
    if !device_result.is_ok() {
        eprintln!("failed to connect to mouse");
        return
    }

    let device = device_result.unwrap();
    // left mouse thread
    thread::spawn(move || {
        loop {
            let (dx, dy) = poll_device(&device, mousetype);
            sender.send(MouseData::new(dx, dy, foot)).expect("device not send data");
        }
    });
}

// grab change in position from mouse
fn poll_device(device: &hidapi::HidDevice, mousetype: MouseType) -> (i16, i16) {
    let mut buf = [0u8; 7];
    device.read(&mut buf).unwrap();

    // so devilishly hacky...
    let (dx, dy) = match mousetype {
        MouseType::NormalI8 => {
            unsafe { (*buf.get_unchecked(1) as i8 as i16, *buf.get_unchecked(2) as i8 as i16) }
        },
        MouseType::NormalI16 => {
            (i16::from_le_bytes(buf[1..3].try_into().unwrap()), i16::from_le_bytes(buf[3..5].try_into().unwrap()))
        },
        MouseType::DeviousI16 => {
            (i16::from_le_bytes(buf[2..4].try_into().unwrap()), i16::from_le_bytes(buf[4..6].try_into().unwrap()))
        },
    };

    (dx, dy)

}

// never thought id write this in code
#[derive(Debug, Clone, Copy)]
enum Foot {
    LEFT,
    RIGHT,
}

/// devious hackfix to specify mouse protocols
#[derive(Debug, Copy, Clone, clap::ValueEnum)]
enum MouseType {
    /// first byte is click, next 2 are movement
    NormalI8,
    /// first byte is click, next 4 are movement
    NormalI16,
    /// first two bytes are not movement, next 4 are movement
    DeviousI16,
}

/// POD struct containing movement and foot
#[derive(Debug, Copy, Clone)]
struct MouseData {
    x_movement: i16,
    y_movement: i16,
    foot: Foot, // yeah i dont think this should be in a normal mouse struct...
}

impl MouseData {
    fn new(x_movement: i16, y_movement: i16, foot: Foot) -> MouseData {
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

/// clap commandline args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// "ip:port" to send data to
    #[arg(short, long, required = true)]
    ip: String,

    /// vid:pid of left mouse. find with lsusb or something
    #[arg(long, required = true)]
    left_mouse: String,

    #[arg(required = true, value_enum, short)]
    left_mouse_type: MouseType,

    /// vid:pid of right mouse. find with lsusb or something
    #[arg(long, required = true)]
    right_mouse: String,

    #[arg(required = true, value_enum, short)]
    right_mouse_type: MouseType,
}

// theres probably a way to automate calling this after clap takes the argument
fn parse_mouse_string(mouse_info: String) -> (u16, u16) {
    let mut split = mouse_info.split(":");
    let (vid, pid) = (split.next(), split.next());
    let (vid, pid) = (u16::from_str_radix(vid.to_owned().unwrap(), 16).unwrap(), u16::from_str_radix(pid.to_owned().unwrap(), 16).unwrap());
    (vid, pid)
}
