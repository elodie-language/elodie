use crate::frontend::old_ast::node::{BreakLoopNode, ContinueLoopNode, LoopNode, Node};
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_break(&mut self, node: &parse::BreakNode) -> old_ast::Result<old_ast::Node> {
        if node.result.is_none() {
            Ok(Node::BreakLoop(BreakLoopNode {
                body: None,
                span: node.token.span.clone(),
            }))
        } else {
            let body = Some(Box::new(self.generate_node(node.result.as_ref().unwrap())?));

            Ok(Node::BreakLoop(BreakLoopNode {
                body,
                span: node.token.span.clone(),
            }))
        }
    }

    pub(crate) fn generate_continue(
        &mut self,
        node: &parse::ContinueNode,
    ) -> crate::frontend::old_ast::Result<old_ast::Node> {
        Ok(Node::ContinueLoop(ContinueLoopNode {
            span: node.token.span.clone(),
        }))
    }

    pub(crate) fn generate_loop(&mut self, node: &parse::LoopNode) -> old_ast::Result<old_ast::Node> {
        let mut body = Vec::with_capacity(node.block.nodes.len());

        for node in &node.block.nodes {
            body.push(self.generate_node(node)?)
        }

        Ok(Node::Loop(LoopNode {
            span: node.token.span.clone(),
            body,
        }))
    }
}
