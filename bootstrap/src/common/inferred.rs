use std::ops::Index;
use crate::common::{StringTable, Type, TypeId, TypeTable};

#[derive(Debug, Clone, PartialEq)]
pub enum Inferred {
    Unknown,

    Boolean,
    Function(Box<[Inferred]>, Box<Inferred>),
    Number,
    Package,
    String,
    Tuple(Box<[Inferred]>),
    Type(TypeId),

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
            Inferred::Boolean => self.type_id_boolean(),
            Inferred::Number => self.type_id_number(),
            Inferred::String => self.type_id_string(),
            _ => unimplemented!()
        };

        &self.index(type_id)
    }
}
