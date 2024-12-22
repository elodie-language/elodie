use std::collections::HashMap;

use crate::common::{StringTable, StringTableId};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct TypeId(pub usize);

static UNKNOWN: TypeId = TypeId(0);

#[derive(Debug, Clone)]
pub struct TypeName(pub StringTableId);

#[derive(Debug, Clone)]
pub struct Type {
    pub id: TypeId,
    pub parent_id: TypeId,
    pub name: TypeName,
    pub variables: Vec<TypeVariable>,
    // trait
    // definitions
    // FIXME track declaration like file, position etc..
}

#[derive(Debug, Clone)]
pub struct TypeVariable {
    pub type_id: TypeId,
    pub variable_id: TypeVariableId,
    pub name: TypeVariableName,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct TypeVariableId(pub usize);

#[derive(Debug, Clone)]
pub struct TypeVariableName(pub StringTableId);


#[derive(Debug)]
pub struct TypeTable {
    offset: usize,
    types: Vec<Type>,
    builtin: HashMap<BuiltinType, TypeId>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BuiltinType {
    Any,
    Boolean,
    Never,
    Number,
    String,
    Unit,
}

impl AsRef<BuiltinType> for BuiltinType {
    fn as_ref(&self) -> &BuiltinType {
        self
    }
}

impl TypeTable {
    pub fn new(string_table: &mut StringTable, seed: usize) -> Self {
        let mut result = Self { types: Vec::new(), offset: seed, builtin: HashMap::new() };

        let any_id = TypeId(result.offset);

        result.types.push(Type {
            id: any_id.clone(),
            parent_id: any_id.clone(),
            name: TypeName(string_table.push_str("Any")),
            variables: vec![],
        });

        result.builtin.insert(BuiltinType::Any, any_id.clone());


        let boolean = result.register(any_id.clone(), TypeName(string_table.push_str("Boolean")));
        result.builtin.insert(BuiltinType::Boolean, boolean);

        let never = result.register(any_id.clone(), TypeName(string_table.push_str("Never")));
        result.builtin.insert(BuiltinType::Never, never);

        let number = result.register(any_id.clone(), TypeName(string_table.push_str("Number")));
        result.builtin.insert(BuiltinType::Number, number);

        let string = result.register(any_id.clone(), TypeName(string_table.push_str("String")));
        result.builtin.insert(BuiltinType::String, string);

        let unit = result.register(any_id.clone(), TypeName(string_table.push_str("Unit")));
        result.builtin.insert(BuiltinType::Unit, unit);

        result
    }

    pub fn builtin(&self, builtin_type: impl AsRef<BuiltinType>) -> TypeId {
        self.builtin[builtin_type.as_ref()].clone()
    }

    pub fn type_id_boolean(&self) -> TypeId { self.builtin(BuiltinType::Boolean) }
    pub fn type_id_number(&self) -> TypeId { self.builtin(BuiltinType::Number) }
    pub fn type_id_string(&self) -> TypeId { self.builtin(BuiltinType::String) }


    pub fn register(&mut self, parent_id: TypeId, name: TypeName) -> TypeId {
        let id = TypeId(self.types.len() + self.offset);
        self.types.push(Type {
            id,
            parent_id,
            name,
            variables: Vec::new(),
        });
        id
    }

    pub fn append_variable(&mut self, type_id: TypeId, name: TypeVariableName, variable_type_id: TypeId) -> TypeId {
        let mut t = self.types.get(type_id.0 - self.offset).unwrap().clone();

        let var_id = TypeVariableId(t.variables.len() + self.offset);

        t.variables.push(TypeVariable {
            type_id: variable_type_id,
            variable_id: var_id,
            name,
        });

        let result = TypeId(self.types.len() + self.offset);
        t.id = result.clone();
        self.types.push(t);

        result
    }

    // append_trait
    // ...
}

