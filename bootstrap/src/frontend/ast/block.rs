
use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::BlockNode;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_block(&mut self, node: &parse::BlockNode) -> ast::Result<ast::Node> {
        let mut body = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            body.push(self.compile_node(node)?)
        }

        return Ok(ast::Node::Block(BlockNode { body }));
    }
}