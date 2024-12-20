use std::ops::Deref;

use crate::frontend::ast::{AstType, Generator};
use crate::frontend::parse;
use crate::frontend::parse::{TypeFunctionNode, TypeNode};

impl<'a> Generator<'a> {
    pub(crate) fn to_ast_type(&self, node: &parse::TypeNode) -> AstType {
        match node {
            TypeNode::Boolean(_) => AstType::Boolean,
            TypeNode::Object(_) => AstType::Object,
            TypeNode::Number(_) => AstType::Number,
            TypeNode::String(_) => AstType::String,
            TypeNode::Function(TypeFunctionNode { arguments, return_type, .. }) => AstType::Function {
                arguments: arguments.iter().map(|a| Box::new(self.to_ast_type(a.r#type.deref()))).collect::<Vec<_>>(),
                return_type: return_type.as_ref().map(|r| Box::new(self.to_ast_type(r.deref()))),
            }
        }
    }
}
