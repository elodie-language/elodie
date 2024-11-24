use std::collections::HashMap;

use crate::common::StringCacheIdx;
use crate::r#type::TypeId;
use crate::run::value::FunctionValue;

pub struct TypeDefinitions {
    pub definitions: HashMap<TypeId, TypeDefinition>,
}

impl TypeDefinitions {
    pub fn add_function(&mut self, type_id: TypeId, fn_id: StringCacheIdx, value: FunctionValue) {
        let mut map = HashMap::new();
        map.insert(fn_id, value);

        self.definitions.insert(type_id, TypeDefinition { functions: map });
    }
}

pub struct TypeDefinition {
    pub functions: HashMap<StringCacheIdx, FunctionValue>,
}

impl TypeDefinitions {
    pub fn get_function(&self, type_id: &TypeId, idx: &StringCacheIdx) -> FunctionValue {
        self.definitions.get(type_id).unwrap().functions.get(idx).cloned().unwrap()
    }
}