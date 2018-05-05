extern crate byteorder;
extern crate serialport;

use std::io;
use std::time::Instant;
use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc;
use std::thread;


fn main() {
    let mut pr = PortReader::new("/dev/cu.usbmodem1411");
    let (tx, rx): (SyncSender<i32>, Receiver<i32>) = mpsc::sync_channel(1);
    thread::spawn(move || {
        for i in 0..10000 {
            if let Ok(_) = tx.try_send(i) {
                continue;
            }
        }
    });

    loop {
        let mut now = Instant::now();
        if let Ok(val) = rx.recv() {
            println!("{:?}", val);
        } else {
            println!("byeeeee!");
            break;
        }
    }
}

struct PortReader {
    port: Box<serialport::SerialPort>,
    unread_stuff: Vec<u8>,
}

impl PortReader {

    // new: the PortReader has logged on
    fn new(port_addr: &str) -> PortReader {
        if let Ok(port) = serialport::open(&port_addr) {
            let unread_stuff: Vec<u8> = Vec::new();
            PortReader{port, unread_stuff}
        } else {
            panic!("omgomgomg")
        }
    }

    // read from the port until we have a whole value
    fn read_value(&mut self) -> i32 {
        let mut serial_buf: Vec<u8> = vec![0,5];
        let mut value: Vec<u8> = self.unread_stuff.clone();
        loop {
            match self.port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    for i in 0..t {
                        if serial_buf[i] == 10 {
                            self.unread_stuff = serial_buf[(i+1)..t].to_vec();
                            if let Ok(val) = String::from_utf8(value).unwrap().parse() {
                                return val
                            } else {
                                return 0
                            }
                        } else {
                            value.extend(vec!(serial_buf[i]));
                        }
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => println!("got this error: {:?}", e),
            }
        }
    }

}

