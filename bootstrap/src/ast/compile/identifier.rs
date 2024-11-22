use crate::ast;
use crate::ast::{Identifier, UseIdentifierNode, parse};
use crate::ast::compile::Compiler;
use crate::ast::Node::LoadValue;
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::ast::compile::Result<ast::Node> {
        return Ok(LoadValue(UseIdentifierNode {
            identifier: Identifier(node.value().to_string()),
            type_id: DefaultTypeIds::never(),
        }));
    }
}