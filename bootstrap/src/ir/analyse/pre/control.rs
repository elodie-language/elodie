use crate::frontend::ast::AstIfNode;
use crate::ir::analyse::pre::Pre;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Pre<'a> {
    pub(crate) fn r#if(&mut self, node: &AstIfNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        todo!()
    }
}