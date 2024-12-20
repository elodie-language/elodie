use crate::frontend::new_ast;
use crate::ir::analyse::infer::Inference;
use crate::ir::analyse::InferredType;

impl<'a> Inference<'a> {
    pub(crate) fn type_from_type_node(&self, node: &'a new_ast::AstType) -> crate::ir::analyse::Result<InferredType> {
        match node {
            new_ast::AstType::Boolean => Ok(InferredType::Boolean),
            new_ast::AstType::Number => Ok(InferredType::Number),
            new_ast::AstType::String => Ok(InferredType::String),
            _ => unimplemented!("{node:#?}")
        }
    }
}