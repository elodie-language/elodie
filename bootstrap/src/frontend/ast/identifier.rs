use crate::common::DefaultTypeIds;
use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::{Identifier, LoadValueNode};
use crate::frontend::ast::node::Node::LoadValue;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> ast::Result<ast::Node> {
        let identifier = Identifier::from(node);
        let ty = self.scope.get_identifier_type(&identifier).unwrap_or(DefaultTypeIds::never());

        return Ok(LoadValue(LoadValueNode {
            identifier,
            ty,
        }));
    }

    pub(crate) fn compile_self(&mut self, node: &parse::ItselfNode) -> ast::Result<ast::Node> {
        return Ok(LoadValue(LoadValueNode {
            identifier: Identifier(node.0.value()),
            ty: DefaultTypeIds::never(),
        }));
    }
}