use std::{fs::File, io::BufReader};
use byteorder::{BigEndian, ReadBytesExt};

use crate::classfile::BYTE_LENGTH_UNAVAILABLE_ERROR;

pub fn generate_interfaces(interfaces_count: u16, reader: &mut BufReader<File>) -> Vec<u16> {
    let mut interfaces: Vec<u16> = vec![];
    for _ in 0..interfaces_count {
        interfaces.push(reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR));
    }
    println!(
        "interface count: {interfaces_count}, interfaces: {:?}",
        interfaces
    );
    interfaces
}
