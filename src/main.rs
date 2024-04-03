mod classfile;

use crate::classfile::{fields::*, ClassFile};

fn main() {
    let class_file = ClassFile::new("/home/rob/Documents/Projects/rust-java/ClassFileCheck.class");
    println!("class file \n: {:?}", class_file);
}
