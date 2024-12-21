use std::ops::Deref;
use std::rc::Rc;

use crate::common::tree::{Node, TreeNode};
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstDeclareVariableNode, AstIdentifier, AstVariant, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_variable(
        &mut self,
        node: &parse::VariableDeclarationNode,
    ) -> ast::Result<TreeNode<AstVariant>> {
        let variable = AstIdentifier(node.identifier.value());

        let node_type = if let Some(type_node) = node.r#type.as_ref() {
            Some(self.to_ast_type(type_node))
        } else {
            None
        };

        let node = Rc::new(self.generate_node(node.node.deref())?);
        Ok(TreeNode::new(Node::DeclareVariable(Rc::new(AstDeclareVariableNode {
            variable,
            value: node,
            value_type: node_type,
        })), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
