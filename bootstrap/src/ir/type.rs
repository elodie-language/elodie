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
}


impl TypeTable {
    pub fn new(string_table: &mut StringTable, seed: usize) -> Self {
        let mut result = Self { types: Vec::new(), offset: seed };

        let any_id = TypeId(result.offset + 1);

        result.types.push(Type {
            id: any_id.clone(),
            parent_id: any_id.clone(),
            name: TypeName(string_table.push_str("Any")),
            variables: vec![],
        });

        result.register(any_id.clone(), TypeName(string_table.push_str("Boolean")));
        result.register(any_id.clone(), TypeName(string_table.push_str("Number")));
        result.register(any_id.clone(), TypeName(string_table.push_str("String")));

        result
    }

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

