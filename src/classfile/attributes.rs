use crate::{classfile::BYTE_LENGTH_UNAVAILABLE_ERROR, Constant};
use byteorder::{BigEndian, ReadBytesExt};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    str,
};

use super::annotations::{
    Annotation, ClassInfoIndexElement, ConstValueElement, ElementValue, ElementValueEnum,
    ElementValuePair, EnumConstValueElement,
};

pub fn generate_attributes(
    attribute_count: u16,
    constant_pool: &Vec<Option<Constant>>,
    reader: &mut BufReader<File>,
) -> Vec<AttributeType> {
    let mut attributes: Vec<AttributeType> = vec![];

    for attribute_index in 0..attribute_count {
        let attribute_name_index: usize = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR).into();
        let attribute_length: u32 = reader.read_u32::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        let attribute_constant_entry = constant_pool
            .get(attribute_name_index - 1)
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR)
            .as_ref()
            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
        println!(
            "attribute_index: {attribute_index}, attribute length: {:?}, attribute_count: {:?}",
            attribute_length, attribute_count
        );
        println!("attribute_constant_entry: {:?}", attribute_constant_entry);
        match attribute_constant_entry {
            Constant::Utf8Info(utf8_constant) => {
                let attribute_name_index: u16 = attribute_name_index as u16;
                println!("attribute name as string: {:?}", utf8_constant.utf_str);
                match utf8_constant.utf_str.as_str() {
                    CONSTANT_VALUE_STR => {
                        let constantvalue_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let constant_value_attribute = AttributeConstantValue {
                            attribute_length,
                            attribute_name_index,
                            constantvalue_index,
                        };
                        attributes.push(AttributeType::ConstantValue(constant_value_attribute));
                    }
                    CODE_STR => {
                        let max_stack: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let max_locals: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let code_length: u32 = reader.read_u32::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);

                        let mut code: Vec<u8> = vec![];
                        for _ in 0..code_length {
                            code.push(reader.read_u8().expect(BYTE_LENGTH_UNAVAILABLE_ERROR));
                        }
                        println!("code attribute inner code: {:?}", code);

                        let exception_table_length: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let mut exception_table: Vec<ExceptionTableEntry> = vec![];
                        for _ in 0..exception_table_length {
                            let start_pc: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let end_pc: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let handler_pc: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let catch_type: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let exception: ExceptionTableEntry = ExceptionTableEntry {
                                start_pc,
                                end_pc,
                                handler_pc,
                                catch_type,
                            };
                            exception_table.push(exception);
                        }
                        let attributes_count: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let attribute_info =
                            generate_attributes(attribute_count, constant_pool, reader);
                        let code = AttributeCode {
                            attribute_name_index,
                            attribute_length,
                            max_stack,
                            max_locals,
                            code_length,
                            code,
                            exception_table_length,
                            exception_table,
                            attributes_count,
                            attribute_info,
                        };
                        attributes.push(AttributeType::Code(code));
                    }
                    STACKMAPTABLE_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match stack map table");
                    }
                    EXCEPTIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match exceptions attribute string");
                    }
                    INNERCLASSES_STR => {
                        let number_of_classes: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let mut classes: Vec<InnerClass> = vec![];
                        for _ in 0..number_of_classes {
                            let inner_class_info_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let outer_class_info_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let inner_name_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let inner_class_access_flags: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let inner_class: InnerClass = InnerClass{inner_class_info_index, outer_class_info_index, inner_name_index, inner_class_access_flags};
                            classes.push(inner_class);
                        }
                        let inner_class_attr: AttributeInnerClasses = AttributeInnerClasses{attribute_name_index, attribute_length, number_of_classes, classes};
                        attributes.push(AttributeType::InnerClasses(inner_class_attr));
                        println!("match inner classes string");
                    }
                    ENCLOSINGMETHODS_STR => {
                        println!("match enclosing methods string");
                    }
                    SYNTHETIC_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        let attribute_length: u32 = reader.read_u32::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR); //this should be a fixed value of 0
                        let synthetic_attribute = AttributeSynthetic {
                            attribute_name_index,
                            attribute_length,
                        };
                        attributes.push(AttributeType::Synthetic(synthetic_attribute));
                    }
                    SIGNATURE_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        let signature_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let signature_attribute: AttributeSignature = AttributeSignature {
                            attribute_name_index,
                            attribute_length,
                            signature_index,
                        };
                        attributes.push(AttributeType::Signature(signature_attribute));
                    }
                    SOURCEFILE_STR => {
                        let sourcefile_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let sourcefile_attr: AttributeSourceFile = AttributeSourceFile{attribute_name_index, attribute_length, sourcefile_index};
                        attributes.push(AttributeType::SourceFile(sourcefile_attr));
                        println!("match source file string");
                    }
                    SOURCEDEBUGEXTENSION_STR => {
                        println!("match source debug extension string");
                    }
                    LINENUMBERTABLE_STR => {
                        let line_number_table_length: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let mut line_number_table: Vec<LineNumberTable_Element> = vec![];
                        for _ in 0..line_number_table_length {
                            let start_pc: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let line_number: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let entry: LineNumberTable_Element = LineNumberTable_Element {
                                start_pc,
                                line_number,
                            };
                            line_number_table.push(entry);
                        }
                        let line_number_table_attr = AttributeLineNumberTable {
                            attribute_name_index,
                            attribute_length,
                            line_number_table_length,
                            line_number_table,
                        };
                        attributes.push(AttributeType::LineNumberTable(line_number_table_attr));
                    }
                    LOCALVARIABLETABLE_STR => {
                        println!("match localvariable table string");
                    }
                    LOCALVARIABLETYPETABLE_STR => {
                        println!("match local variable type table string");
                    }
                    DEPRECATED_STR => {
                        let deprecated_attribute = AttributeDeprecated {
                            attribute_name_index,
                            attribute_length,
                        };
                        attributes.push(AttributeType::Deprecated(deprecated_attribute));
                    }
                    RUNTIMEINVISIBLEANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match runtime invisible annotations");
                    }
                    RUNTIMEINVISIBLEPARAMETERANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match runtime invisible parameter annotations");
                    }
                    RUNTIMEVISIBLEPARAMETERANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match runtime visible parameter annotations");
                    }
                    ANNOTATIONDEFAULT_STR => {
                        println!("match annotation default string");
                    }
                    BOOTSTRAPMETHODS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        println!("match bootstrap methods");
                        let num_bootstrap_methods: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                        let mut bootstrap_methods: Vec<BootstrapMethodElement> = vec![];
                        for _ in 0..num_bootstrap_methods {
                            let bootstrap_method_ref: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let num_bootstrap_arguments: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let mut bootstrap_arguments: Vec<u16> = vec![];
                            for _ in 0..num_bootstrap_arguments {
                                let argument = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                bootstrap_arguments.push(argument);
                            }
                            let method: BootstrapMethodElement = BootstrapMethodElement{bootstrap_method_ref, num_bootstrap_arguments, bootstrap_arguments};
                            bootstrap_methods.push(method);
                        }
                        let bootstrap_attr: AttributeBootstrapMethods = AttributeBootstrapMethods{attribute_name_index, attribute_length, num_bootstrap_methods, bootstrap_methods};
                        attributes.push(AttributeType::BootstrapMethods(bootstrap_attr));
                    }
                    RUNTIMEVISIBLEANNOTATIONS_STR => {
                        println!("attribute index: {:?}", attribute_index);
                        let name = constant_pool
                            .get(attribute_index as usize)
                            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR)
                            .as_ref()
                            .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);

                        if attribute_length > 0 {
                            println!(
                                "name at the attribute index {attribute_index} is: {:?}",
                                name
                            );

                            let num_annotations: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                            let mut annotations: Vec<Annotation> = vec![];
                            for _ in 0..num_annotations {
                                let type_index: u16 = reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                let num_element_value_pairs: u16 =
                                    reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);

                                let mut element_value_pairs: Vec<ElementValuePair> = vec![];
                                println!("num_element_value_pairs: {num_element_value_pairs}");
                                for _ in 0..num_element_value_pairs {
                                    let element_name_index: u16 =
                                        reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                    let name = constant_pool
                                        .get(element_name_index as usize)
                                        .expect(BYTE_LENGTH_UNAVAILABLE_ERROR)
                                        .as_ref()
                                        .expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                    println!("name at element_name_index: {element_name_index}, is: {:?}", name);
                                    let tag: u8 = reader.read_u8().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                    println!("tag found: {:?}", tag);
                                    let tag_char = tag as char;

                                    let value: ElementValue;

                                    let value: ElementValue = match tag_char {
                                        's' | 'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => {
                                            let const_value_index: u16 =
                                                reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                            let constant_value: ConstValueElement =
                                                ConstValueElement { const_value_index };
                                            ElementValue {
                                                tag,
                                                value: ElementValueEnum::ConstantValueIndex(
                                                    constant_value,
                                                ),
                                            }
                                        }
                                        'e' => {
                                            let type_name_index: u16 =
                                                reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                            let const_name_index: u16 =
                                                reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                            let enum_const_value: EnumConstValueElement =
                                                EnumConstValueElement {
                                                    type_name_index,
                                                    const_name_index,
                                                };
                                            ElementValue {
                                                tag,
                                                value: ElementValueEnum::EnumConstValue(
                                                    enum_const_value,
                                                ),
                                            }
                                        }
                                        'c' => {
                                            let class_info_index: u16 =
                                                reader.read_u16::<BigEndian>().expect(BYTE_LENGTH_UNAVAILABLE_ERROR);
                                            let class_const =
                                                ClassInfoIndexElement { class_info_index };
                                            ElementValue {
                                                tag,
                                                value: ElementValueEnum::ClassInfoIndex(
                                                    class_const,
                                                ),
                                            }
                                        }
                                        '@' => {
                                            panic!("@ tag found for annotation");
                                        }
                                        '[' => {
                                            panic!("[ value found for annotation");
                                        }
                                        _ => {
                                            // println!("constant pool is: {:?}", constant_pool);
                                            panic!("No valid tag value found for annotation: {:?}, num of annotations: {:?}, num of value pairs: {:?}", tag, num_annotations, num_element_value_pairs);
                                        }
                                    };

                                    let element_value_pair: ElementValuePair = ElementValuePair {
                                        element_name_index,
                                        value,
                                    };
                                    element_value_pairs.push(element_value_pair);
                                }
                                let annotation: Annotation = Annotation {
                                    type_index,
                                    num_element_value_pairs,
                                    element_value_pairs,
                                };
                                annotations.push(annotation);
                            }
                            let runtime_visible_annotation: AttributeRuntimeVisibleAnnotations =
                                AttributeRuntimeVisibleAnnotations {
                                    attribute_name_index,
                                    attribute_length,
                                    num_annotations,
                                    annotations,
                                };
                            attributes.push(AttributeType::RuntimeVisibleAnnotations(
                                runtime_visible_annotation,
                            ));
                        }
                        let runtime_visible_annotation: AttributeRuntimeVisibleAnnotations =
                            AttributeRuntimeVisibleAnnotations {
                                attribute_name_index,
                                attribute_length,
                                num_annotations: 0,
                                annotations: vec![],
                            };
                        attributes.push(AttributeType::RuntimeVisibleAnnotations(
                            runtime_visible_annotation,
                        ));
                    }
                    _ => {
                        println!("about to panic. attribute name was: {:?}, attribute_length is: {attribute_length}", utf8_constant.utf_str.as_str());
                        let mut buffer = vec![0u8; attribute_length as usize];
                        let _ = reader.read_exact(&mut buffer);
                    }
                }
            }
            _ => {
                panic!(
                    "No valid utf8 string entry found for attribute_name_index: {}",
                    attribute_name_index
                );
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
pub struct AttributeBootstrapMethods {
    attribute_name_index: u16,
    attribute_length: u32,
    num_bootstrap_methods: u16,
    bootstrap_methods: Vec<BootstrapMethodElement>
}

#[derive(Debug, Clone)]
pub struct BootstrapMethodElement {
    bootstrap_method_ref: u16,
    num_bootstrap_arguments: u16,
    bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct AttributeSourceFile {
    attribute_name_index: u16,
    attribute_length: u32,
    sourcefile_index: u16,
}

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
    SourceFile(AttributeSourceFile),
    SourceDebugExtension,
    LineNumberTable(AttributeLineNumberTable),
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated(AttributeDeprecated),
    RuntimeVisibleAnnotations(AttributeRuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods(AttributeBootstrapMethods),
}

#[derive(Debug, Clone)]
pub struct AttributeLineNumberTable {
    attribute_name_index: u16,
    attribute_length: u32,
    line_number_table_length: u16,
    line_number_table: Vec<LineNumberTable_Element>,
}

#[derive(Debug, Clone)]
pub struct LineNumberTable_Element {
    start_pc: u16,
    line_number: u16,
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
    attribute_info: Vec<AttributeType>,
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
    exception_index_table: Vec<u16>,
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
