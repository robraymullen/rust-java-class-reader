use crate::{classfile::{attributes::*, BYTE_LENGTH_UNAVAILABLE_ERROR}, Constant};
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    str,
};

pub fn generate_methods(
    methods_count: u16,
    constant_pool: &Vec<Option<Constant>>,
    reader: &mut BufReader<File>,
) -> Vec<Method> {
    // let methods_count: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
    let mut methods: Vec<Method> = vec![];

    println!("methods count: {methods_count}");

    for _ in 0..methods_count {
        let access_flags: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let name_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let descriptor_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let attributes_count: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);

        println!("method attributes count: {attributes_count}");
        let attributes = generate_attributes(attributes_count, constant_pool, reader);

        let method = Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        };
        methods.push(method);
    }

    methods
}

#[derive(Debug, Clone)]
pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeType>,
}
