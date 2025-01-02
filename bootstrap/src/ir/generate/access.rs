use crate::common::node::Node::AccessVariable;
use crate::common::Span;
use crate::ir::analyse::TypeAccessVariableNode;
use crate::ir::generate::Generator;
use crate::ir::{IrAccessVariableNode, IrTreeNode};

impl<'a> Generator<'a> {
    pub(crate) fn access_variable(&mut self, node: &TypeAccessVariableNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let variable = self.symbol_table.variable(node.variable);

        Ok(IrTreeNode::new(
            AccessVariable(IrAccessVariableNode{
                variable: node.variable
            }),
            span,
            variable.type_id.unwrap()
        ))
    }
}