use crate::{ir, parse};
use crate::ir::{Identifier, UseIdentifierNode};
use crate::ir::Node::LoadValue;
use crate::compile::Compiler;
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::compile::Result<ir::Node> {
        return Ok(LoadValue(UseIdentifierNode {
            identifier: Identifier::from(node),
            type_id: DefaultTypeIds::never(),
        }));
    }
}