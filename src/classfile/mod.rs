use std::{fs::File, io::BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{
    classfile::{
        attributes::generate_attributes, constant_pool::generate_constant_pool,
        interfaces::generate_interfaces, methods::{generate_methods, Method},
        constant_pool::Constant, 
    },
    generate_fields, FieldInfo,
};

use self::attributes::AttributeType;

pub mod annotations;
pub mod attributes;
pub mod constant_pool;
pub mod fields;
pub mod interfaces;
pub mod methods;

pub const BYTE_LENGTH_UNAVAILABLE_ERROR: &str =
    "Could not allocate the required bytes from the class file";

#[derive(Debug, Clone)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<Option<Constant>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<Method>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeType>,
}

impl ClassFile {
    pub fn new(class_file_path: &str) -> ClassFile {
        let file = File::open(class_file_path).expect("Class file unavailable");
        let mut reader = BufReader::new(file);

        let magic = reader
            .read_u32::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let minor_version = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let major_version = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let constant_pool_count: usize = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR)
            as usize;
        println!(
            "{:?}, {:?}, {:?}, {:?}",
            magic, minor_version, major_version, constant_pool_count
        );

        let constant_pool: Vec<Option<Constant>> =
            generate_constant_pool(&mut reader, constant_pool_count - 1);

        println!("constant pool: {:?}", constant_pool);

        let access_flags: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let this_class: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let super_class: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);

        println!(
            "access flags: {access_flags}, this_class: {this_class}, super_class: {super_class}"
        );
        let interfaces_count: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let interfaces = generate_interfaces(interfaces_count, &mut reader);

        for interface_index in interfaces.iter() {
            let index: usize = (*interface_index).into();
            println!(
                "interface constant: {:?}",
                constant_pool
                    .get(index)
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR)
            );
        }

        let fields_count: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let fields = generate_fields(fields_count, &constant_pool, &mut reader);

        let methods_count: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let methods: Vec<Method> = generate_methods(methods_count, &constant_pool, &mut reader);

        let attributes_count: u16 = reader
            .read_u16::<BigEndian>()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let attributes = generate_attributes(attributes_count, &constant_pool, &mut reader);

        ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count: constant_pool_count
                .try_into()
                .expect(BYTE_LENGTH_UNAVAILABLE_ERROR),
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
            attributes,
        }

        // println!("class file \n: {:?}", class_file);
    }
}
