extern crate byteorder;

use byteorder::{BigEndian, WriteBytesExt, LittleEndian, ReadBytesExt};

fn main() {
    let mut wtr = vec![];
    wtr.write_u16::<BigEndian>(500).unwrap();
    let mut wtr2 = vec![];
    wtr2.write_u16::<LittleEndian>(500).unwrap();
    println!("{:?}", wtr);
    println!("{:?}", wtr2);
    let rdr = vec![52, 50, 56];
    // println!("{:?}", (&rdr[..]).read_u32::<LittleEndian>().unwrap());
    // let mut rdr2 = vec![0, 0, 57, 57];
    // println!("{:?}", (&rdr2[..]).read_u32::<BigEndian>().unwrap());
    println!("{:?}", String::from_utf8(rdr));
    // let mut wtr2 = vec![];
    // wtr2.write_u16::<LittleEndian>(500).unwrap();
    // println!("{:?}", rdr);
    // println!("{:?}", wtr2);
}
