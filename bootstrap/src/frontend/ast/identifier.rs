use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{Identifier, LoadValueNode};
use crate::frontend::ast::node::Node::LoadValue;

impl<'a> Generator<'a> {
    pub(crate) fn generate_identifier(&mut self, node: &parse::IdentifierNode) -> ast::Result<ast::Node> {
        let identifier = Identifier::from(node);

        return Ok(LoadValue(LoadValueNode {
            identifier,
        }));
    }

    pub(crate) fn generate_self(&mut self, node: &parse::ItselfNode) -> ast::Result<ast::Node> {
        return Ok(LoadValue(LoadValueNode {
            identifier: Identifier(node.0.value()),
        }));
    }
}