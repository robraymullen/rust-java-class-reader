use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read}, str};
use byteorder::{BigEndian, ReadBytesExt};
use crate::{classfile::attributes::*, Constant};

pub fn generate_methods(constant_pool: &Vec<Option<Constant>>, reader: &mut BufReader<File>) -> Vec<Method> {
    let methods_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let mut methods: Vec<Method> = vec![];

    println!("methods count: {methods_count}");
    
    for _ in 0 .. methods_count {
        let access_flags: u16 = reader.read_u16::<BigEndian>().unwrap();
        let name_index: u16 = reader.read_u16::<BigEndian>().unwrap();
        let descriptor_index: u16 = reader.read_u16::<BigEndian>().unwrap();
        let attributes_count: u16 = reader.read_u16::<BigEndian>().unwrap();

        println!("method attributes count: {attributes_count}");
        let attributes = generate_attributes(attributes_count, constant_pool, reader);
        
        let method = Method{access_flags, name_index, descriptor_index, attributes_count, attributes};
        methods.push(method);
    }

    methods
}

pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeType>,
}