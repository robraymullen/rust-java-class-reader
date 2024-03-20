use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read}, str};
use byteorder::{BigEndian, ReadBytesExt};
use crate::Constant;

use super::annotations::{Annotation, ClassInfoIndexElement, ConstValueElement, ElementValue, ElementValueEnum, ElementValuePair, EnumConstValueElement};


pub fn generate_attributes(attribute_count: u16, constant_pool: &Vec<Option<Constant>>, reader: &mut BufReader<File>) -> Vec<AttributeType> {
    let mut attributes: Vec<AttributeType> = vec![];

    for attribute_index in 0..attribute_count {
        
        let attribute_name_index: usize = reader.read_u16::<BigEndian>().unwrap().into();
        let attribute_length: u32 = reader.read_u32::<BigEndian>().unwrap();
        let attribute_constant_entry = constant_pool.get(attribute_name_index).unwrap().as_ref().unwrap();
        println!("attribute length: {:?}, attribute_count: {:?}", attribute_length, attribute_count);
        match attribute_constant_entry {
            Constant::Utf8Info(utf8_constant) => {
                let attribute_name_index: u16 = attribute_name_index as u16;
                println!("attribute name as string: {:?}", utf8_constant.utf_str);
                match utf8_constant.utf_str.as_str() {
                    CONSTANT_VALUE_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        let constantvalue_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                        let constant_value_attribute = AttributeConstantValue{attribute_length, attribute_name_index, constantvalue_index};
                        attributes.push(AttributeType::ConstantValue(constant_value_attribute));
                    },
                    SYNTHETIC_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        let attribute_length: u32 = reader.read_u32::<BigEndian>().unwrap(); //this should be a fixed value of 0
                        let synthetic_attribute = AttributeSynthetic{attribute_name_index, attribute_length};
                        attributes.push(AttributeType::Synthetic(synthetic_attribute));
                    },
                    SIGNATURE_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        // let attribute_length: u32 = reader.read_u32::<BigEndian>().unwrap(); //should be a fixed value of 2
                        let signature_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                        let signature_attribute: AttributeSignature = AttributeSignature{attribute_name_index, attribute_length, signature_index};
                        attributes.push(AttributeType::Signature(signature_attribute)); 
                    },
                    DEPRECATED_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        // let attribute_length: u32 = reader.read_u32::<BigEndian>().unwrap(); //should be a fixed value of 0
                        let deprecated_attribute = AttributeDeprecated{attribute_name_index, attribute_length};
                        attributes.push(AttributeType::Deprecated(deprecated_attribute));
                    },
                    STACKMAPTABLE_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match stack map table");
                    },
                    RUNTIMEINVISIBLEANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match runtime invisible annotations");
                    },
                    RUNTIMEINVISIBLEPARAMETERANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match runtime invisible parameter annotations");
                    },
                    RUNTIMEVISIBLEPARAMETERANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match runtime visible parameter annotations");
                    },
                    BOOTSTRAPMETHODS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match bootstrap methods");
                    },
                    CODE_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match code attribute");
                    },
                    EXCEPTIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match exceptions attribute string");
                    },
                    RUNTIMEVISIBLEANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        
                        // let attribute_length: u32 = reader.read_u32::<BigEndian>().unwrap();
                        let num_annotations: u16 = reader.read_u16::<BigEndian>().unwrap();
                        let mut annotations: Vec<Annotation> = vec![];
                        for _ in 0..num_annotations {
                            let type_index = reader.read_u16::<BigEndian>().unwrap();
                            let num_element_value_pairs = reader.read_u16::<BigEndian>().unwrap();
                            
                            let mut element_value_pairs: Vec<ElementValuePair> = vec![];
                            println!("num_element_value_pairs: {num_element_value_pairs}");
                            for _ in 0..num_element_value_pairs {
                                let element_name_index: u16 = reader.read_u16::<BigEndian>().unwrap();

                                let tag: u8 = reader.read_u8().unwrap();
                                println!("tag found: {:?}", tag);
                                let tag_char = tag as char;

                                let value: ElementValue;

                                match tag_char {
                                    's'|'B'|'C'|'D'|'F'|'I'|'J'|'S'|'Z' => {
                                        let const_value_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                                        let constant_value: ConstValueElement = ConstValueElement{const_value_index};
                                        value = ElementValue{tag, value: ElementValueEnum::ConstantValueIndex(constant_value)};
                                    },
                                    'e' => {
                                        let type_name_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                                        let const_name_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                                        let enum_const_value: EnumConstValueElement = EnumConstValueElement{type_name_index, const_name_index};
                                        value = ElementValue{tag, value: ElementValueEnum::EnumConstValue(enum_const_value)};
                                    },
                                    'c' => {
                                        let class_info_index: u16 = reader.read_u16::<BigEndian>().unwrap();
                                        let class_const = ClassInfoIndexElement{class_info_index};
                                        value = ElementValue{tag, value: ElementValueEnum::ClassInfoIndex(class_const)};
                                    },
                                    '@' => {
                                        panic!("@ tag found for annotation");
                                    },
                                    '[' => {
                                        panic!("[ value found for annotation");
                                    },
                                    _ => {
                                        panic!("No valid tag value found for annotation: {:?}, num of annotations: {:?}, num of value pairs: {:?}", tag, num_annotations, num_element_value_pairs);
                                    }
                                }

                                let element_value_pair: ElementValuePair = ElementValuePair{element_name_index, value};
                                element_value_pairs.push(element_value_pair);
                            }
                            let annotation: Annotation = Annotation{type_index, num_element_value_pairs, element_value_pairs};
                            annotations.push(annotation);
                        }
                        let runtime_visible_annotation: AttributeRuntimeVisibleAnnotations = AttributeRuntimeVisibleAnnotations{attribute_name_index, attribute_length, num_annotations, annotations};
                    }
                    _ => {
                        panic!("No valid attribute found");
                    }
                }
                
            },
            _ => {
                panic!("No valid utf8 string entry found for attribute_name_index: {}", attribute_name_index);
            }
        }
    }

    attributes
}

const CONSTANT_VALUE_STR: &str = "ConstantValue";
const CODE_STR: &str = "Code";
const STACKMAPTABLE_STR: &str = "StackMapTable";
const EXCEPTIONS_STR: &str = "Exceptions";
const INNERCLASSES_STR: &str = "InnerClasses";
const ENCLOSINGMETHODS_STR: &str = "EnclosingMethod";
const SYNTHETIC_STR: &str = "Synthetic";
const SIGNATURE_STR: &str = "Signature";
const SOURCEFILE_STR: &str = "SourceFile";
const SOURCEDEBUGEXTENSION_STR: &str = "SourceDebugExtension";
const LINENUMBERTABLE_STR: &str = "LineNumberTable";
const LOCALVARIABLETABLE_STR: &str = "LocalVariableTable";
const LOCALVARIABLETYPETABLE_STR: &str = "LocalVariableTypeTable";
const DEPRECATED_STR: &str = "Deprecated";
const RUNTIMEVISIBLEANNOTATIONS_STR: &str = "RuntimeVisibleAnnotations";
const RUNTIMEINVISIBLEANNOTATIONS_STR: &str = "RuntimeInvisibleAnnotations";
const RUNTIMEVISIBLEPARAMETERANNOTATIONS_STR: &str = "RuntimeVisibleParameterAnnotations";
const RUNTIMEINVISIBLEPARAMETERANNOTATIONS_STR: &str = "RuntimeInvisibleParameterAnnotations";
const ANNOTATIONDEFAULT_STR: &str = "AnnotationsDefault";
const BOOTSTRAPMETHODS_STR: &str = "BootstrapMethods";

#[derive(Debug, Clone)]
pub struct Attribute {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<AttributeType>,
}

#[derive(Debug, Clone)]
pub enum AttributeType {
    ConstantValue(AttributeConstantValue),
    Code(AttributeCode),
    Exceptions(AttributeExceptions),
    InnerClasses(AttributeInnerClasses),
    EnclosingMethods(AttributeEnclosingMethod),
    Synthetic(AttributeSynthetic),
    Signature(AttributeSignature),
    Deprecated(AttributeDeprecated),
    RuntimeVisibleAnnotations(AttributeRuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods,
}

#[derive(Debug, Clone)]
pub struct AttributeDeprecated {
    attribute_name_index: u16,
    attribute_length: u32,
}

#[derive(Debug, Clone)]
pub struct AttributeSignature {
    attribute_name_index: u16,
    attribute_length: u32,
    signature_index: u16,
}

#[derive(Debug, Clone)]
pub struct AttributeConstantValue {
    attribute_name_index: u16,
    attribute_length: u32,
    constantvalue_index: u16,
}

#[derive(Debug, Clone)]
pub struct AttributeCode {
    attribute_name_index: u16,
    attribute_length: u32,
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u16,
    exception_table: Vec<ExceptionTableEntry>,
    attributes_count: u16,
    attributes_info: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug, Clone)]
pub struct AttributeExceptions {
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_exceptions: u16,
    exception_index_table: Vec<u16>
}

#[derive(Debug, Clone)]
pub struct AttributeInnerClasses {
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_classes: u16,
    classes: Vec<InnerClass>,
}

#[derive(Debug, Clone)]
struct InnerClass {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct AttributeEnclosingMethod {
    attribute_name_index: u16,
    attribute_length: u32,
    class_index: u16,
    method_index: u16,
}

#[derive(Debug, Clone)]
pub struct AttributeSynthetic {
    attribute_name_index: u16,
    attribute_length: u32,
}

#[derive(Debug, Clone)] 
pub struct AttributeRuntimeVisibleAnnotations {
    attribute_name_index: u16,
    attribute_length: u32,
    num_annotations: u16,
    annotations: Vec<Annotation>,
}
