use std::rc::Rc;

use crate::common::tree::Node::AccessVariable;
use crate::common::tree::TreeNode;
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstAccessVariableNode, AstIdentifier, AstVariant, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_identifier(
        &mut self,
        node: &parse::IdentifierNode,
    ) -> ast::Result<TreeNode<AstVariant>> {
        return Ok(TreeNode::new(AccessVariable(Rc::new(AstAccessVariableNode {
            variable: AstIdentifier(node.value()),
        })), SPAN_NOT_IMPLEMENTED.clone()));
    }

    pub(crate) fn generate_self(&mut self, node: &parse::ItselfNode) -> ast::Result<TreeNode<AstVariant>> {
        return Ok(TreeNode::new(AccessVariable(Rc::new(AstAccessVariableNode {
            variable: AstIdentifier(node.value()),
        })), SPAN_NOT_IMPLEMENTED.clone()));
    }
}
