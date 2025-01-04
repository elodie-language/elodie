use std::ops::Index;

use crate::common::{StringTable, Type, TypeId, TypeTable};

#[derive(Debug, Clone, PartialEq)]
pub enum Inferred {
    Unknown,

    Boolean,
    Function(Box<[Inferred]>, Box<Inferred>),
    Float4,
    Float8,
    Int1,
    Int2,
    Int4,
    Int8,
    Int16,

    Number,
    String,
    Tuple(Box<[Inferred]>),
    Type(TypeId),
    Uint1,
    Uint2,
    Uint4,
    Uint8,
    Uint16,

    Unit,

    OneOf(Box<[Inferred]>),
    AllOf(Box<[Inferred]>),
}

impl Inferred {
    pub fn to_string(&self, string_table: &StringTable) -> String {
        match self {
            Inferred::Boolean => "Boolean".to_string(),
            Inferred::Number => "Number".to_string(),
            Inferred::String => "String".to_string(),
            _ => unimplemented!("{self:#?}")
        }
    }
}

impl Index<Inferred> for TypeTable {
    type Output = Type;
    fn index(&self, index: Inferred) -> &Self::Output {
        let type_id = match index {
            Inferred::Boolean => TypeId::BOOLEAN,
            Inferred::Number => TypeId::NUMBER,
            Inferred::String => TypeId::STRING,
            _ => unimplemented!()
        };

        &self.index(type_id)
    }
}
