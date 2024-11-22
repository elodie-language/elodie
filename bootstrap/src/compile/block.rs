use crate::{ast, parse};
use crate::ast::BlockNode;
use crate::ast::r#type::DefaultTypeIds;
use crate::compile::Compiler;

impl Compiler {
    pub(crate) fn compile_block(&mut self, node: &parse::BlockNode) -> crate::compile::Result<ast::Node> {
        let mut body = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            body.push(self.compile_node(node)?)
        }

        return Ok(ast::Node::Block(BlockNode {
            body,
            return_type: DefaultTypeIds::never(),
        }));
    }
}