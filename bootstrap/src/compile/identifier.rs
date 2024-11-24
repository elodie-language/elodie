use crate::{ir, parse};
use crate::compile::Compiler;
use crate::ir::{Identifier, ItselfNode, LoadValueNode};
use crate::ir::Node::{Itself, LoadValue};
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::compile::Result<ir::Node> {
        return Ok(LoadValue(LoadValueNode {
            identifier: Identifier::from(node),
            type_id: DefaultTypeIds::never(),
        }));
    }

    pub(crate) fn compile_self(&mut self, node: &parse::ItselfNode) -> crate::compile::Result<ir::Node> {
        return Ok(Itself(ItselfNode()));
    }
}