mod classfile;

use crate::classfile::{fields::*, ClassFile};
use crate::classfile::{
    attributes::generate_attributes,
    constant_pool::*,
    methods::{generate_methods, Method},
};
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    str,
};

fn main() {
    let file = File::open("/home/rob/Documents/Projects/rust-java/ClassFileCheck.class").unwrap();
    let mut reader = BufReader::new(file);

    // for line in reader.lines() {
    //     println!("{:?}", line);
    // }

    // let mut s = String::new();
    let magic = reader.read_u32::<BigEndian>().unwrap();
    // println!("{:?}", code);
    let minor_version = reader.read_u16::<BigEndian>().unwrap();
    let major_version = reader.read_u16::<BigEndian>().unwrap();
    let constant_pool_count: usize = reader.read_u16::<BigEndian>().unwrap() as usize;
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        magic, minor_version, major_version, constant_pool_count
    );

    let constant_pool: Vec<Option<Constant>> =
        generate_constant_pool(&mut reader, constant_pool_count - 1);

    println!("constant pool: {:?}", constant_pool);

    let access_flags: u16 = reader.read_u16::<BigEndian>().unwrap();
    let this_class: u16 = reader.read_u16::<BigEndian>().unwrap();
    let super_class: u16 = reader.read_u16::<BigEndian>().unwrap();

    println!("access flags: {access_flags}, this_class: {this_class}, super_class: {super_class}");
    let interfaces_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let interfaces = generate_interfaces(interfaces_count, &mut reader);

    for interface_index in interfaces.iter() {
        let index: usize = (*interface_index).into();
        println!(
            "interface constant: {:?}",
            constant_pool.get(index).unwrap()
        );
    }

    
    let fields_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let fields = generate_fields(fields_count, &constant_pool, &mut reader);

    let methods_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let methods: Vec<Method> = generate_methods(methods_count, &constant_pool, &mut reader);

    // println!("methods: {:?}", methods);
    let attributes_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let attributes = generate_attributes(attributes_count, &constant_pool, &mut reader);

    let class_file = ClassFile {
        magic,
        minor_version,
        major_version,
        constant_pool_count: constant_pool_count.try_into().unwrap(),
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
        attributes_count,
        attributes
    };

    println!("class file \n: {:?}", class_file);
}

fn generate_interfaces(interfaces_count: u16, reader: &mut BufReader<File>) -> Vec<u16> {
    // let interfaces_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let mut interfaces: Vec<u16> = vec![];
    for _ in 0..interfaces_count {
        interfaces.push(reader.read_u16::<BigEndian>().unwrap());
    }
    println!(
        "interface count: {interfaces_count}, interfaces: {:?}",
        interfaces
    );
    interfaces
}

// fn generate_fields(reader: &mut BufReader<File>) -> Vec<u16> {
//     let fields_count: u16 = reader.read_u16::<BigEndian>().unwrap();
//     let mut fields: Vec<u16> = vec![];
//     for _ in 1..fields_count {
//         fields.push(reader.read_u16::<BigEndian>().unwrap());
//     }
//     println!("fields_count: {fields_count}, fields: {:?}", fields);
//     fields
// }

// fn generate_methods(reader: &mut BufReader<File>) -> Vec<u16> {
//     let methods_count: u16 = reader.read_u16::<BigEndian>().unwrap();
//     let mut methods =
// }
