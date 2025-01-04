use std::collections::HashMap;
use std::ops::Index;

use crate::common::{StringTable, StringTableId};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum TypeId {
    Builtin(usize),
    Custom(usize),
}

impl TypeId {
    pub const NEVER: TypeId = TypeId::Builtin(0);
    pub const UNKNOWN: TypeId = TypeId::Builtin(1);
    pub const ANY: TypeId = TypeId::Builtin(2);
    pub const BOOLEAN: TypeId = TypeId::Builtin(3);
    pub const NUMBER: TypeId = TypeId::Builtin(4);
    pub const STRING: TypeId = TypeId::Builtin(5);
    pub const UNIT: TypeId = TypeId::Builtin(6);
    pub const FLOAT4: TypeId = TypeId::Builtin(7);
    pub const FLOAT8: TypeId = TypeId::Builtin(8);
    pub const INT1: TypeId = TypeId::Builtin(9);
    pub const INT2: TypeId = TypeId::Builtin(10);
    pub const INT4: TypeId = TypeId::Builtin(11);
    pub const INT8: TypeId = TypeId::Builtin(12);
    pub const INT16: TypeId = TypeId::Builtin(13);
    pub const UINT1: TypeId = TypeId::Builtin(14);
    pub const UINT2: TypeId = TypeId::Builtin(15);
    pub const UINT4: TypeId = TypeId::Builtin(16);
    pub const UINT8: TypeId = TypeId::Builtin(17);
    pub const UINT16: TypeId = TypeId::Builtin(18);
}


#[derive(Debug, Clone)]
pub struct TypeName(pub StringTableId);

#[derive(Debug, Clone)]
pub struct Type {
    pub id: TypeId,
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
    builtin: HashMap<TypeId, Type>,
    custom: Vec<Type>,
}

impl TypeTable {
    pub fn new(string_table: &mut StringTable) -> Self {
        let mut result = Self {
            builtin: HashMap::new(),
            custom: Vec::new(),
        };

        result.builtin.insert(TypeId::BOOLEAN, Type { id: TypeId::BOOLEAN, name: TypeName(string_table.push_str("Boolean")), variables: vec![] });
        result.builtin.insert(TypeId::NUMBER, Type { id: TypeId::NUMBER, name: TypeName(string_table.push_str("Number")), variables: vec![] });
        result.builtin.insert(TypeId::STRING, Type { id: TypeId::STRING, name: TypeName(string_table.push_str("String")), variables: vec![] });

        // result.types.push(Type {
        //     id: any_id.clone(),
        //     name: TypeName(string_table.push_str("ANY")),
        //     variables: vec![],
        // });
        //
        // result.builtin.insert(BuiltinType::Any, any_id.clone());
        //
        //
        // let boolean = result.register(any_id.clone(), TypeName(string_table.push_str("Boolean")));
        // result.builtin.insert(BuiltinType::Boolean, boolean);
        //
        // let never = result.register(any_id.clone(), TypeName(string_table.push_str("Never")));
        // result.builtin.insert(BuiltinType::Never, never);
        //
        // let number = result.register(any_id.clone(), TypeName(string_table.push_str("Number")));
        // result.builtin.insert(BuiltinType::Number, number);
        //
        // let string = result.register(any_id.clone(), TypeName(string_table.push_str("String")));
        // result.builtin.insert(BuiltinType::String, string);
        //
        // let unit = result.register(any_id.clone(), TypeName(string_table.push_str("Unit")));
        // result.builtin.insert(BuiltinType::Unit, unit);

        result
    }


    pub fn register(&mut self, name: TypeName) -> TypeId {
        let id = TypeId::Custom(self.custom.len());
        self.custom.push(Type {
            id,
            name,
            variables: Vec::new(),
        });
        id
    }

    // pub fn append_variable(&mut self, type_id: TypeId, name: TypeVariableName, variable_type_id: TypeId) -> TypeId {
    //     // let mut t = self.builtin.get(type_id.0 - self.offset).unwrap().clone();
    //     //
    //     // let var_id = TypeVariableId(t.variables.len() + self.offset);
    //     //
    //     // t.variables.push(TypeVariable {
    //     //     type_id: variable_type_id,
    //     //     variable_id: var_id,
    //     //     name,
    //     // });
    //     //
    //     // let result = TypeId(self.builtin.len() + self.offset);
    //     // t.id = result.clone();
    //     // self.builtin.push(t);
    //     // result
    // }

    // append_trait
    // ...
}

impl Index<TypeId> for TypeTable {
    type Output = Type;
    fn index(&self, index: TypeId) -> &Self::Output {
        match index {
            TypeId::Builtin(_) => &self.builtin[&index],
            TypeId::Custom(id) => self.custom.index(id)
        }
    }
}