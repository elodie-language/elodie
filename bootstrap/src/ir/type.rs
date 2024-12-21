use std::collections::HashMap;

use crate::common::StringTableId;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct TypeId(pub usize);

#[derive(Debug)]
pub struct TypeName(pub String);

#[derive(Debug)]
pub struct Type {
    pub id: TypeId,
    pub name: TypeName,
    pub variables: HashMap<StringTableId, TypeVariable>,
    // trait
    // definitions
    // FIXME track declaration like file, position etc..
}

#[derive(Debug)]
pub struct TypeVariable {}

#[derive(Debug)]
pub struct TypeTable {
    next_id: usize,
    types: HashMap<TypeId, Type>,
}

impl TypeTable {
    // FIXME nobody should ever rely on a concrete type id value => therefore make it random
    pub fn new() -> Self {
        let mut result = Self {
            next_id: 1,
            types: HashMap::new(),
        };
        result
    }
}
