use crate::frontend::ast;
use crate::ir::analyse::infer::Inference;
use crate::ir::analyse::InferredType;

impl<'a> Inference<'a> {
    pub(crate) fn type_from_type_node(
        &self,
        node: &'a ast::AstType,
    ) -> crate::ir::analyse::Result<InferredType> {
        match node {
            ast::AstType::Boolean => Ok(InferredType::Boolean),
            ast::AstType::Number => Ok(InferredType::Number),
            ast::AstType::String => Ok(InferredType::String),
            _ => unimplemented!("{node:#?}"),
        }
    }
}
