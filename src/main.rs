use std::{fs::File, io::{BufRead, BufReader, Read}, str};
use byteorder::{BigEndian, ReadBytesExt};
// use std::io::{Write, Read, Cursor};

struct ConstantPool {
    tag: u8,
    info: Vec<u8>,
}

const CONSTANT_POOL_CLASS              :u8 = 7;
const CONSTANT_POOL_FIELDREF           :u8 = 9;
const CONSTANT_POOL_METHODREF          :u8 = 10;
const CONSTANT_POOL_INTERFACEMETHODREF :u8 = 11;
const CONSTANT_POOL_STRING             :u8 = 8;
const CONSTANT_POOL_INTEGER            :u8 = 3;
const CONSTANT_POOL_FLOAT              :u8 = 4;
const CONSTANT_POOL_LONG               :u8 = 5;
const CONSTANT_POOL_DOUBLE             :u8 = 6;
const CONSTANT_POOL_NAME_AND_TYPE      :u8 = 12;
const CONSTANT_POOL_UTF8               :u8 = 1;
const CONSTANT_POOL_METHOD_HANDLE      :u8 = 15;
const CONSTANT_POOL_METHOD_TYPE        :u8 = 16;
const CONSTANT_POOL_INVOKE_DYNAMIC     :u8 = 18;

fn generate_constant_pool(reader: &mut BufReader<File>, constant_pool_size: usize) -> Vec<ConstantPool> {
    let constant_pool: Vec<ConstantPool> = vec![];
    for index in 1..constant_pool_size {
        let tag: u8 = reader.read_u8().unwrap();

        match tag {
            CONSTANT_POOL_CLASS => {
                let name_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("Class name index: {name_index}");
            },
            CONSTANT_POOL_FIELDREF
            | CONSTANT_POOL_METHODREF
            | CONSTANT_POOL_INTERFACEMETHODREF => {
                let class_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                let name_and_type_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("Class index: {class_index}, name and type index: {name_and_type_index}");
            },
            CONSTANT_POOL_STRING => {
                let string_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("string index: {string_index}");
            },
            CONSTANT_POOL_INTEGER => {
                let bytes: u32 = reader.read_u32::<BigEndian>().unwrap();
                println!("bytes: {bytes}");
            },
            CONSTANT_POOL_FLOAT => {
                let bytes: u32 = reader.read_u32::<BigEndian>().unwrap();
                println!("float bytes: {bytes}");
            },
            CONSTANT_POOL_LONG => {
                let high_bytes: u32 = reader.read_u32::<BigEndian>().unwrap();
                let low_bytes: u32 = reader.read_u32::<BigEndian>().unwrap();
                println!("long high bytes: {high_bytes}");
            },
            CONSTANT_POOL_DOUBLE => {
                let high_bytes: u32 = reader.read_u32::<BigEndian>().unwrap();
                let low_bytes: u32 = reader.read_u32::<BigEndian>().unwrap();
                println!("double high bytes: {high_bytes}");
            },
            CONSTANT_POOL_NAME_AND_TYPE => {
                let name_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                let descriptor_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("name and type. name index: {name_index}, descriptor index: {descriptor_index}");
            },
            CONSTANT_POOL_UTF8 => {
                let length = reader.read_u16::<BigEndian>().unwrap();
                let mut bytes: Vec<u8> = vec![];
                for _ in 0..length {
                    bytes.push(reader.read_u8().unwrap());
                }
                let string = str::from_utf8(&bytes).unwrap();
                println!("utf8 string: {string}, length: {length}");
            },
            CONSTANT_POOL_METHOD_HANDLE => {
                let reference_kind: u8 = reader.read_u8().unwrap();
                let reference_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("method handle. reference kind: {reference_kind}, reference index: {reference_index}");
            },
            CONSTANT_POOL_METHOD_TYPE => {
                let descriptor_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("method type. descriptor index: {descriptor_index}");
            },
            CONSTANT_POOL_INVOKE_DYNAMIC => {
                let bootstrap_method_attr_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                let name_and_type_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                println!("invoke dynamic. bootstrap method attr index: {bootstrap_method_attr_index}, name and type index: {name_and_type_index}");
            },
            _ => panic!("Unexpected tag type for constant pool")
        }
    }
    constant_pool
}

fn main() {
    let file = File::open("/home/rob/Documents/Projects/rust-java/ClassFileCheck.class").unwrap();
    let mut reader = BufReader::new(file);
    
    // for line in reader.lines() {
    //     println!("{:?}", line);
    // }

    // let mut s = String::new();
    let magic = reader.read_u32::<BigEndian>();
    // println!("{:?}", code);
    let minor_version = reader.read_u16::<BigEndian>();
    let major_version = reader.read_u16::<BigEndian>();
    let constant_pool_count: usize = reader.read_u16::<BigEndian>().unwrap() as usize;
    println!("{:?}, {:?}, {:?}, {:?}", magic, minor_version, major_version, constant_pool_count);

    let constant_pool: Vec<ConstantPool> = generate_constant_pool(&mut reader, constant_pool_count);
}


