use std::collections::HashMap;

use crate::common::StringCacheIdx;
use crate::r#type::TypeId;
use crate::run::value::FunctionValue;

#[derive(Debug)]
pub struct TypeDefinitions {
    pub definitions: HashMap<TypeId, TypeDefinition>,
}

impl TypeDefinitions {

    pub fn add_function(&mut self, type_id: TypeId, fn_id: StringCacheIdx, value: FunctionValue) {
        if let Some(type_def) = self.definitions.get_mut(&type_id) {
            type_def.functions.insert(fn_id, value);
        } else {
            let mut new_type_def = TypeDefinition { functions: HashMap::new(), };
            new_type_def.functions.insert(fn_id, value);
            self.definitions.insert(type_id, new_type_def);
        }
    }
}

#[derive(Debug)]
pub struct TypeDefinition {
    pub functions: HashMap<StringCacheIdx, FunctionValue>,
}

impl TypeDefinitions {

    pub fn insert_function(&mut self, type_id: TypeId, idx: StringCacheIdx, value :FunctionValue){
        self.definitions.get_mut(&type_id).unwrap().functions.insert(idx, value);
    }

    pub fn get_function(&self, type_id: &TypeId, idx: &StringCacheIdx) -> FunctionValue {
        self.definitions.get(type_id).unwrap().functions.get(idx).cloned().unwrap()
    }
}