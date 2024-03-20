use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read}, str};
use byteorder::{BigEndian, ReadBytesExt};
use crate::{classfile::attributes::*, Constant};

const ACC_PUBLIC: u16 = 0x0001; 
const ACC_PRIVATE : u16= 0x0002; 	
const ACC_PROTECTED: u16 = 0x0004; 
const ACC_STATIC : u16= 0x0008; 
const ACC_FINAL: u16 = 0x0010; 	
const ACC_VOLATILE: u16 = 0x0040; 	
const ACC_TRANSIENT: u16 = 0x0080;	
const ACC_SYNTHETIC: u16 = 0x1000; 	
const ACC_ENUM: u16 = 0x4000;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeType>,
}

pub fn generate_fields(constant_pool: &Vec<Option<Constant>>, reader: &mut BufReader<File>) -> Vec<FieldInfo> {
    let fields_count: u16 = reader.read_u16::<BigEndian>().unwrap();
    let mut fields: Vec<FieldInfo> = vec![];
    for _ in 0..fields_count {
        let access_flags: u16 = reader.read_u16::<BigEndian>().unwrap();
        let name_index: u16 = reader.read_u16::<BigEndian>().unwrap();
        let constant_name = constant_pool.get(name_index as usize).unwrap().as_ref().unwrap();
        println!("constant name for field: {:?}", constant_name);
        let descriptor_index: u16 = reader.read_u16::<BigEndian>().unwrap();
        let attributes_count: u16 = reader.read_u16::<BigEndian>().unwrap();
        println!("Generating attributes for field, count is: {attributes_count}");
        let attributes: Vec<AttributeType> = generate_attributes(attributes_count, constant_pool, reader);
        let field_info: FieldInfo = FieldInfo{access_flags, name_index, descriptor_index, attributes_count, attributes};
        fields.push(field_info);   
    }
    println!("fields_count: {fields_count}, fields: {:?}", fields);
    fields
}
