
#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug, Clone)]
pub struct ElementValue {
    pub tag: u8,
    pub value: ElementValueEnum
}

#[derive(Debug, Clone)]
pub enum ElementValueEnum {
    ConstantValueIndex(ConstValueElement),
    EnumConstValue(EnumConstValueElement),
    ClassInfoIndex(ClassInfoIndexElement),
    AnnotationValue(Annotation),
    ArrayValue(ArrayValueElement),
}

#[derive(Debug, Clone)]
pub struct ConstValueElement {
    pub const_value_index: u16,
}

#[derive(Debug, Clone)]
pub struct EnumConstValueElement {
    pub type_name_index: u16,
    pub const_name_index: u16,
}

#[derive(Debug, Clone)]
pub struct ClassInfoIndexElement {
    pub class_info_index: u16,
}

#[derive(Debug, Clone)]
pub struct ArrayValueElement {
    num_values: u16,
    values: Vec<ElementValue>,
}