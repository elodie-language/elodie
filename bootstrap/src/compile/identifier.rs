use crate::{ir, parse};
use crate::compile::Compiler;
use crate::ir::{Identifier, LoadValueNode};
use crate::ir::Node::LoadValue;
use crate::r#type::{DefaultTypeIds, TypeId};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::compile::Result<ir::Node> {
        let identifier = Identifier::from(node);
        let ty = self.scope.get_identifier_type(&identifier).unwrap_or(DefaultTypeIds::never());

        return Ok(LoadValue(LoadValueNode {
            identifier,
            ty,
        }));
    }

    pub(crate) fn compile_self(&mut self, node: &parse::ItselfNode) -> crate::compile::Result<ir::Node> {
        return Ok(LoadValue(LoadValueNode {
            identifier: Identifier(node.0.value()),
            ty: DefaultTypeIds::never(),
        }));
    }
}