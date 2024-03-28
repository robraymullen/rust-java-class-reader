use crate::{Constant, FieldInfo, Method};

use self::attributes::AttributeType;

pub mod annotations;
pub mod attributes;
pub mod constant_pool;
pub mod fields;
pub mod methods;

pub const BYTE_LENGTH_UNAVAILABLE_ERROR: &str = "Could not allocate the required bytes from the class file";

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