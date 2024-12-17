use crate::{ir};
use crate::ir::compile::Compiler;
use crate::ir::{Identifier, LoadValueNode};
use crate::ir::Node::LoadValue;
use crate::common::{DefaultTypeIds, TypeId};
use crate::frontend::parse;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::ir::compile::Result<ir::Node> {
        let identifier = Identifier::from(node);
        let ty = self.scope.get_identifier_type(&identifier).unwrap_or(DefaultTypeIds::never());

        return Ok(LoadValue(LoadValueNode {
            identifier,
            ty,
        }));
    }

    pub(crate) fn compile_self(&mut self, node: &parse::ItselfNode) -> crate::ir::compile::Result<ir::Node> {
        return Ok(LoadValue(LoadValueNode {
            identifier: Identifier(node.0.value()),
            ty: DefaultTypeIds::never(),
        }));
    }
}