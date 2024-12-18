use crate::frontend::parse;
use crate::frontend::parse::TypeNode;
use crate::ir::infer::{Inference, InferredType};

impl<'a> Inference<'a> {
    pub(crate) fn type_from_type_node(&self, node: &'a parse::TypeNode) -> crate::ir::infer::Result<InferredType> {
        match node {
            TypeNode::Boolean(_) => Ok(InferredType::Boolean),
            TypeNode::Number(_) => Ok(InferredType::Number),
            TypeNode::String(_) => Ok(InferredType::String),
            _ => unimplemented!("{node:#?}")
        }
    }
}