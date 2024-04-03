use byteorder::{BigEndian, ReadBytesExt};
use std::{fs::File, io::BufReader, str};

use crate::classfile::BYTE_LENGTH_UNAVAILABLE_ERROR;

pub fn generate_constant_pool(
    reader: &mut BufReader<File>,
    constant_pool_size: usize,
) -> Vec<Option<Constant>> {
    let mut constant_pool: Vec<Option<Constant>> = vec![];

    for _ in 1..(constant_pool_size + 1) {
        constant_pool.push(None);
    }

    println!(
        "constant pool size: {:?}, given size: {:?}",
        constant_pool.len(),
        constant_pool_size + 1
    );
    for index in 1..constant_pool_size {
        let tag: u8 = reader.read_u8().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);

        match tag {
            CONSTANT_POOL_CLASS => {
                let name_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let class = ConstantClass { tag, name_index };
                println!("index: {index}, class: {:?}", class);
                constant_pool[index] = Some(Constant::Class(class));
                println!("Class name index: {name_index}");
            }
            CONSTANT_POOL_FIELDREF | CONSTANT_POOL_METHODREF | CONSTANT_POOL_INTERFACEMETHODREF => {
                let class_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let name_and_type_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let constant_ref = ConstantRef {
                    tag,
                    class_index,
                    name_and_type_index,
                };
                constant_pool[index] = Some(Constant::Ref(constant_ref));
                println!("Class index: {class_index}, name and type index: {name_and_type_index}");
            }
            CONSTANT_POOL_STRING => {
                let string_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let string = ConstantStringInfo { tag, string_index };
                constant_pool[index] = Some(Constant::String(string));
                println!("string index: {string_index}");
            }
            CONSTANT_POOL_INTEGER => {
                let bytes: u32 = reader
                    .read_u32::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let integer = ConstantInteger { tag, bytes };
                constant_pool[index] = Some(Constant::Integer(integer));
                println!("integer bytes: {bytes}");
            }
            CONSTANT_POOL_FLOAT => {
                let bytes: u32 = reader
                    .read_u32::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let float = ConstantFloat { tag, bytes };
                constant_pool[index] = Some(Constant::Float(float));
                println!("float bytes: {bytes}");
            }
            CONSTANT_POOL_LONG => {
                let high_bytes: u32 = reader
                    .read_u32::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let low_bytes: u32 = reader
                    .read_u32::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let long = ConstantLong {
                    tag,
                    high_bytes,
                    low_bytes,
                };
                constant_pool[index] = Some(Constant::Long(long));
                println!("long high bytes: {high_bytes}, long low bytes: {low_bytes}");
            }
            CONSTANT_POOL_DOUBLE => {
                let high_bytes: u32 = reader
                    .read_u32::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let low_bytes: u32 = reader
                    .read_u32::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let double = ConstantDouble {
                    tag,
                    high_bytes,
                    low_bytes,
                };
                constant_pool[index] = Some(Constant::Double(double));
                println!("double high bytes: {high_bytes}");
            }
            CONSTANT_POOL_NAME_AND_TYPE => {
                let name_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let descriptor_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let name_and_type = ConstantNameAndType {
                    tag,
                    name_index,
                    descriptor_index,
                };
                constant_pool[index] = Some(Constant::NameAndType(name_and_type));
                println!(
                    "name and type. name index: {name_index}, descriptor index: {descriptor_index}"
                );
            }
            CONSTANT_POOL_UTF8 => {
                let length = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let mut bytes: Vec<u8> = vec![];
                for _ in 0..length {
                    bytes.push(reader.read_u8().expect(BYTE_LENGTH_UNAVAILABLE_ERROR));
                }
                let utf_str: String = str::from_utf8(&bytes)
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR)
                    .into();
                let utf8 = ConstantUtf8Info {
                    tag,
                    utf_str: utf_str.clone(),
                };
                constant_pool[index] = Some(Constant::Utf8Info(utf8));
                println!("utf8 string: {utf_str}, length: {length}");
            }
            CONSTANT_POOL_METHOD_HANDLE => {
                let reference_kind: u8 = reader.read_u8().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let reference_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let method_handle = ConstantMethodHandle {
                    tag,
                    reference_index,
                    reference_kind,
                };
                constant_pool[index] = Some(Constant::MethodHandle(method_handle));
                println!("method handle. reference kind: {reference_kind}, reference index: {reference_index}");
            }
            CONSTANT_POOL_METHOD_TYPE => {
                let descriptor_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let method_type = ConstantMethodType {
                    tag,
                    descriptor_index,
                };
                constant_pool[index] = Some(Constant::MethodType(method_type));
                println!("method type. descriptor index: {descriptor_index}");
            }
            CONSTANT_POOL_INVOKE_DYNAMIC => {
                let bootstrap_method_attr_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let name_and_type_index: u16 = reader
                    .read_u16::<BigEndian>()
                    .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                let invoke_dynamic = ConstantInvokeDynamic {
                    tag,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                };
                constant_pool[index] = Some(Constant::InvokeDynamic(invoke_dynamic));
                println!("invoke dynamic. bootstrap method attr index: {bootstrap_method_attr_index}, name and type index: {name_and_type_index}");
            }
            _ => {
                println!("index: {index}, tag: {tag}");
                panic!("Unexpected tag type for constant pool")
            }
        }
    }
    constant_pool
}

pub struct ConstantPool {
    tag: u8,
    info: Vec<u8>,
}

const CONSTANT_POOL_CLASS: u8 = 7;
const CONSTANT_POOL_FIELDREF: u8 = 9;
const CONSTANT_POOL_METHODREF: u8 = 10;
const CONSTANT_POOL_INTERFACEMETHODREF: u8 = 11;
const CONSTANT_POOL_STRING: u8 = 8;
const CONSTANT_POOL_INTEGER: u8 = 3;
const CONSTANT_POOL_FLOAT: u8 = 4;
const CONSTANT_POOL_LONG: u8 = 5;
const CONSTANT_POOL_DOUBLE: u8 = 6;
const CONSTANT_POOL_NAME_AND_TYPE: u8 = 12;
const CONSTANT_POOL_UTF8: u8 = 1;
const CONSTANT_POOL_METHOD_HANDLE: u8 = 15;
const CONSTANT_POOL_METHOD_TYPE: u8 = 16;
const CONSTANT_POOL_INVOKE_DYNAMIC: u8 = 18;

#[derive(Debug, Clone, Copy)]
pub struct ConstantClass {
    tag: u8,
    name_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantRef {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantFieldRef {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantMethodRef {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantInterfaceMethodRef {
    tag: u8,
    class_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantStringInfo {
    tag: u8,
    string_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantInteger {
    tag: u8,
    bytes: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantFloat {
    tag: u8,
    bytes: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantLong {
    tag: u8,
    high_bytes: u32,
    low_bytes: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantDouble {
    tag: u8,
    high_bytes: u32,
    low_bytes: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantNameAndType {
    tag: u8,
    name_index: u16,
    descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantUtf8Info {
    pub tag: u8,
    // length: u16,
    // bytes: Vec<u8>,
    pub utf_str: String,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantMethodHandle {
    tag: u8,
    reference_kind: u8,
    reference_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantMethodType {
    tag: u8,
    descriptor_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantInvokeDynamic {
    tag: u8,
    bootstrap_method_attr_index: u16,
    name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub enum Constant {
    Class(ConstantClass),
    Ref(ConstantRef),
    FieldRef(ConstantFieldRef),
    MethodRef(ConstantMethodRef),
    InterfaceMethodRef(ConstantInterfaceMethodRef),
    String(ConstantStringInfo),
    Integer(ConstantInteger),
    Float(ConstantFloat),
    Long(ConstantLong),
    Double(ConstantDouble),
    NameAndType(ConstantNameAndType),
    Utf8Info(ConstantUtf8Info),
    MethodHandle(ConstantMethodHandle),
    MethodType(ConstantMethodType),
    InvokeDynamic(ConstantInvokeDynamic),
}
