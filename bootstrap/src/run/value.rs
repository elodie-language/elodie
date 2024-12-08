use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::common::StringTableId;
use crate::ir::{BlockNode, FunctionArgumentNode, Identifier};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Function(FunctionValue),
    #[deprecated]
    IntrinsicFunction(IntrinsicFunctionValue),
    List(ListValue),
    Number(f64),
    F64(f64),
    Object(ObjectValue),
    Package(PackageValue),
    String(String),
    Tuple(TupleValue),
    // FIXME make intrinsics not a normal object - have some Intrinsic value instead
    Unit,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(v) => v.to_string(),
            Value::IntrinsicFunction(_) => "[IntrinsicFunction]".to_string(),
            Value::Function(_) => "[Function]".to_string(),
            Value::Number(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::Object(_) => "[Object]".to_string(),
            Value::Package(_) => "[Package]".to_string(),
            Value::List(_) => "[List]".to_string(),
            Value::String(v) => v.clone(),
            Value::Tuple(_) => "[Tuple]".to_string(),
            Value::Unit => "Unit".to_string(),
        }
    }
}

#[derive(Clone)]
#[deprecated]
pub struct IntrinsicFunctionValue(pub Rc<dyn Fn(&[Value]) -> crate::run::Result<Value>>);

impl Debug for IntrinsicFunctionValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[IntrinsicFunction]")
    }
}

#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub arguments: Vec<Rc<FunctionArgumentNode>>,
    pub body: Rc<BlockNode>,
}

#[derive(Debug, Clone)]
pub struct PackageValue {
    pub identifier: StringTableId,
    pub functions: HashMap<StringTableId, FunctionValue>,
    pub external_functions: HashMap<StringTableId, IntrinsicFunctionValue>,
    pub packages: HashMap<StringTableId, PackageValue>,
}

#[derive(Clone, Debug)]
pub struct ListValue(pub Rc<RefCell<Vec<Value>>>);

impl PackageValue {
    pub fn get_function(&self, identifier: StringTableId) -> Option<&FunctionValue> {
        self.functions.get(&identifier)
    }

    pub fn get_intrinsic_function(&self, identifier: StringTableId) -> Option<&IntrinsicFunctionValue> {
        self.external_functions.get(&identifier)
    }
}

#[derive(Debug, Clone)]
pub struct ObjectValue {
    pub properties: HashMap<StringTableId, Value>,
}

impl ObjectValue {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new()
        }
    }

    pub fn set_property(&mut self, key: StringTableId, value: Value) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &StringTableId) -> Option<&Value> {
        self.properties.get(key)
    }

    #[deprecated]
    pub fn get_property_host_function(&self, identifier: impl AsRef<Identifier>) -> Option<&IntrinsicFunctionValue> {
        let identifier = identifier.as_ref();
        if let Some(Value::IntrinsicFunction(result)) = &self.properties.get(&identifier.0) {
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct TupleValue {
    values: Vec<Value>,
}