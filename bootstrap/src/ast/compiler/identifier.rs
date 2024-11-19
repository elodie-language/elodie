use crate::ast;
use crate::ast::{Identifier, LoadVariableNode, parse};
use crate::ast::compiler::Compiler;
use crate::ast::Node::LoadVariable;
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_identifier(&mut self, node: &parse::IdentifierNode) -> crate::ast::compiler::Result<ast::Node> {
        return Ok(LoadVariable(LoadVariableNode {
            identifier: Identifier(node.value().to_string()),
            type_id: DefaultTypeIds::never(),
        }));
    }
}