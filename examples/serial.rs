extern crate byteorder;
extern crate serialport;

use std::io;
use std::time::Instant;
use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc;
use std::thread;


fn main() {
    // if let Ok(mut port) = serialport::open("/dev/cu.usbmodem1411") {
    //     let mut serial_buf: Vec<u8> = vec![0; 5];
    //     println!("receiving data?");
    //     loop {
    //         match port.read(serial_buf.as_mut_slice()) {
    //             Ok(t) => {
    //                 let sensor_val: String = String::from_utf8(serial_buf[..t].to_vec()).unwrap();
    //                 println!("{:?}", sensor_val);
    //             },
    //             Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
    //             Err(e) => println!("got this error: {:?}", e),
    //         }
    //     }
    // } else {
    //     println!("couldn't open the port!");
    // }

    // let mut pr = PortReader::new("/dev/cu.usbmodem1411");
    let (tx, rx): (SyncSender<i32>, Receiver<i32>) = mpsc::sync_channel(1);
    thread::spawn(move || {
        for i in 0..10000 {
            // let mut now = Instant::now();
            if let Ok(_) = tx.try_send(i) {
                continue;
            } else {
                continue;
            }
            // times.push(now.elapsed().subsec_nanos());
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
        // println!("{:?}", now.elapsed());
    }

    // let total_nanos: u32 = times.iter().sum();

    // println!("{:?}", total_nanos as f32 / times.len() as f32);
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
                    // println!("{:?}", serial_buf);
                    for i in 0..t {
                        // if we're at a newline:
                        //   append buffer to self.unread_stuff, then clear self.unread_stuff
                        if serial_buf[i] == 10 {
                            println!("{:?}", value);
                            self.unread_stuff = serial_buf[(i+1)..t].to_vec();
                            return 0
                            // String::from_utf8(value).unwrap().parse().unwrap()

                            // match value.len() {
                            //     0 => 0,
                            //     1 => (&value[..]).read_i8().unwrap() as i32,
                            //     2 => (&value[..]).read_i16::<LittleEndian>().unwrap() as i32,
                            //     3 => (&value[..]).read_i24::<LittleEndian>().unwrap() as i32,
                            //     4 => (&value[..]).read_i32::<LittleEndian>().unwrap(),
                            //     5 => (&value[..]).read_i32::<LittleEndian>().unwrap(),
                            //     6 => (&value[..]).read_i32::<LittleEndian>().unwrap(),
                            //     _ => {
                            //         println!("{:?}", value.len());
                            //         panic!("nooooo!")
                            //     },
                            // }
                            // return (&value[..]).read_i32::<BigEndian>().unwrap();
                        } else {
                            value.extend(vec!(serial_buf[i]));
                        }
                    }
                    // let sensor_val = String::from_utf8(buf).unwrap();
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => println!("got this error: {:?}", e),
            }
        }
    }

}
