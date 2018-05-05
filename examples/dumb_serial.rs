// extern crate byteorder;
extern crate serialport;

use std::io;
// use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    println!("{:?}", String::from("hello"));
    if let Ok(mut port) = serialport::open("/dev/cu.usbmodem1411") {
        let mut serial_buf: Vec<u8> = vec![0; 10];
        println!("receiving data?");
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    // let sensor_val: String = String::from_utf8(serial_buf[..t].to_vec()).unwrap();
                    println!("{:?}", &serial_buf[..t]);
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => println!("got this error: {:?}", e),
            }
        }
    } else {
        println!("couldn't open the port!");
    }
}
