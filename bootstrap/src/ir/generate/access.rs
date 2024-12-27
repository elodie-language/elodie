use crate::common::Span;
use crate::ir::analyse::TypeAccessVariableNode;
use crate::ir::generate::Generator;
use crate::ir::IrTreeNode;

impl<'a> Generator<'a> {
    pub(crate) fn access_variable(&mut self, node: &TypeAccessVariableNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        todo!()
    }
}