use crate::common::node::Node::AccessVariable;
use crate::frontend::ast::{
    AstAccessVariableNode, AstIdentifier, AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED,
};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_identifier(
        &mut self,
        node: &parse::IdentifierNode,
    ) -> ast::Result<AstTreeNode> {
        return Ok(AstTreeNode::new(
            AccessVariable(AstAccessVariableNode {
                variable: AstIdentifier(node.value()),
            }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ));
    }

    pub(crate) fn generate_self(&mut self, node: &parse::ItselfNode) -> ast::Result<AstTreeNode> {
        return Ok(AstTreeNode::new(
            AccessVariable(AstAccessVariableNode {
                variable: AstIdentifier(node.value()),
            }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ));
    }
}
