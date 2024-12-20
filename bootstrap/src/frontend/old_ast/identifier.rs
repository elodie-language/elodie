use crate::frontend::old_ast::node::Node::LoadValue;
use crate::frontend::old_ast::node::{Identifier, LoadValueNode};
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_identifier(
        &mut self,
        node: &parse::IdentifierNode,
    ) -> old_ast::Result<old_ast::Node> {
        let identifier = Identifier::from(node);

        return Ok(LoadValue(LoadValueNode {
            span: node.0.span.clone(),
            identifier,
        }));
    }

    pub(crate) fn generate_self(&mut self, node: &parse::ItselfNode) -> old_ast::Result<old_ast::Node> {
        return Ok(LoadValue(LoadValueNode {
            span: node.0.span.clone(),
            identifier: Identifier(node.0.clone()),
        }));
    }
}
