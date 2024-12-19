use crate::frontend::ast;
use crate::ir::analyse::infer::{Inference};
use crate::ir::analyse::InferredType;

impl<'a> Inference<'a> {
    pub(crate) fn type_from_type_node(&self, node: &'a ast::TypeNode) -> crate::ir::analyse::Result<InferredType> {
        match node {
            ast::TypeNode::Boolean(_) => Ok(InferredType::Boolean),
            ast::TypeNode::Number(_) => Ok(InferredType::Number),
            ast::TypeNode::String(_) => Ok(InferredType::String),
            _ => unimplemented!("{node:#?}")
        }
    }
}