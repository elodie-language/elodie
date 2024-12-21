use std::ops::Deref;
use std::rc::Rc;

use crate::common::node::Node;
use crate::frontend::ast::{
    AstDeclareVariableNode, AstIdentifier, AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED,
};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_variable(
        &mut self,
        node: &parse::VariableDeclarationNode,
    ) -> ast::Result<AstTreeNode> {
        let variable = AstIdentifier(node.identifier.value());

        let node_type = if let Some(type_node) = node.r#type.as_ref() {
            Some(self.to_ast_type(type_node))
        } else {
            None
        };

        let node = Rc::new(self.generate_node(node.node.deref())?);
        Ok(AstTreeNode::new(
            Node::DeclareVariable(AstDeclareVariableNode {
                variable,
                value: node,
                value_type: node_type,
            }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }
}
