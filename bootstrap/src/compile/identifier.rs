use crate::{ast, parse};
use crate::ast::{Identifier, UseIdentifierNode};
use crate::ast::Node::LoadValue;
use crate::ast::r#type::DefaultTypeIds;
use crate::compile::Compiler;

impl Compiler {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::compile::Result<ast::Node> {
        return Ok(LoadValue(UseIdentifierNode {
            identifier: Identifier(node.value().to_string()),
            type_id: DefaultTypeIds::never(),
        }));
    }
}