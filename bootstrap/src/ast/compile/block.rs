use crate::ast;
use crate::ast::{BlockNode, parse};
use crate::ast::compile::Compiler;
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_block(&mut self, node: &parse::BlockNode) -> ast::compile::Result<ast::Node> {
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