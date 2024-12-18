use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{BreakLoopNode, ContinueLoopNode, LoopNode, Node};

impl<'a> Generator<'a> {
    pub(crate) fn generator_break(&mut self, node: &parse::BreakNode) -> ast::Result<ast::Node> {
        if node.result.is_none() {
            Ok(Node::BreakLoop(BreakLoopNode { body: None }))
        } else {
            let body = Some(Box::new(self.generator_node(node.result.as_ref().unwrap())?));

            Ok(Node::BreakLoop(BreakLoopNode { body }))
        }
    }

    pub(crate) fn generator_continue(&mut self, _node: &parse::ContinueNode) -> crate::frontend::ast::Result<ast::Node> {
        Ok(Node::ContinueLoop(ContinueLoopNode {}))
    }

    pub(crate) fn generator_loop(&mut self, node: &parse::LoopNode) -> ast::Result<ast::Node> {
        let mut body = Vec::with_capacity(node.block.nodes.len());

        for node in &node.block.nodes {
            body.push(self.generator_node(node)?)
        }

        Ok(
            Node::Loop(LoopNode {
                body,
            })
        )
    }
}