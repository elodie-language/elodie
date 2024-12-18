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
    pub properties: HashMap<StringTableId, Property>,
    // trait
    // functions
    // FIXME track declaration like file, position etc..
}

#[derive(Debug)]
pub struct Property {}

#[derive(Debug)]
pub struct TypeTable {
    next_id: usize,
    types: HashMap<TypeId, Type>,
    base_type_ids: HashMap<BaseType, TypeId>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum BaseType {
    Boolean,
    Number,
    String,
}

impl TypeTable {
    // FIXME nobody should ever rely on a concrete type id value => therefore make it random
    pub fn new() -> Self {
        let mut result = Self {
            next_id: 1,
            types: HashMap::new(),
            base_type_ids: HashMap::new(),
        };

        let id = result.register(TypeName("Boolean".to_string()), HashMap::new());
        result.base_type_ids.insert(BaseType::Boolean, id);

        let id = result.register(TypeName("Number".to_string()), HashMap::new());
        result.base_type_ids.insert(BaseType::Number, id);

        let id = result.register(TypeName("String".to_string()), HashMap::new());
        result.base_type_ids.insert(BaseType::String, id);

        result
    }


    pub fn get_type(&self, id: &TypeId) -> &Type {
        self.types.get(id).unwrap()
    }

    pub fn get_base_type_id(&self, base_type: &BaseType) -> TypeId {
        self.base_type_ids.get(base_type).cloned().unwrap()
    }

    pub fn is_boolean(&self, ty: &TypeId) -> bool {
        self.base_type_ids.get(&BaseType::Boolean).unwrap() == ty
    }

    pub fn is_number(&self, ty: &TypeId) -> bool {
        self.base_type_ids.get(&BaseType::Number).unwrap() == ty
    }

    pub fn is_string(&self, ty: &TypeId) -> bool {
        self.base_type_ids.get(&BaseType::String).unwrap() == ty
    }

    pub fn register(&mut self, type_name: TypeName, properties: HashMap<StringTableId, Property>) -> TypeId {
        let new_id = TypeId(self.next_id);
        self.next_id += 1;

        self.types.insert(new_id.clone(), Type {
            id: new_id.clone(),
            name: type_name,
            properties,
        });

        new_id
    }
}