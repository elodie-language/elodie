use crate::frontend::{ast, parse};
use crate::frontend::ast::{AccessVariableNode, Generator, Node, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::node::{AstNode, Identifier};
use crate::frontend::ast::Node::AccessVariable;

impl<'a> Generator<'a> {

    pub(crate) fn generate_identifier(
        &mut self,
        node: &parse::IdentifierNode,
    ) -> ast::Result<AstNode> {
        return Ok(AstNode::new(Node::AccessVariable(AccessVariableNode {
            variable: Identifier(node.value()),
        }), SPAN_NOT_IMPLEMENTED.clone()));
    }

    pub(crate) fn generate_self(&mut self, node: &parse::ItselfNode) -> ast::Result<AstNode> {
        return Ok(AstNode::new(AccessVariable(AccessVariableNode {
            variable: Identifier(node.value()),
        }), SPAN_NOT_IMPLEMENTED.clone()));
    }
}
