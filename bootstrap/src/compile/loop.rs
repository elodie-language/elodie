use crate::{ir, compile, parse};
use crate::ir::{BreakLoopNode, ContinueLoopNode, LoopNode, Node};
use crate::r#type::DefaultTypeIds;
use crate::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_break(&mut self, node: &parse::BreakNode) -> compile::Result<ir::Node> {
        if node.result.is_none() {
            Ok(Node::BreakLoop(BreakLoopNode { body: None, return_type: DefaultTypeIds::unit() }))
        } else {
            let body = Some(Box::new(self.compile_node(node.result.as_ref().unwrap())?));
            Ok(Node::BreakLoop(BreakLoopNode {
                body,
                return_type: DefaultTypeIds::never(),
            }))
        }
    }

    pub(crate) fn compile_continue(&mut self, _node: &parse::ContinueNode) -> crate::compile::Result<ir::Node> {
        Ok(Node::ContinueLoop(ContinueLoopNode {}))
    }

    pub(crate) fn compile_loop(&mut self, node: &parse::LoopNode) -> compile::Result<ir::Node> {
        let mut body = Vec::with_capacity(node.block.nodes.len());

        for node in &node.block.nodes {
            body.push(self.compile_node(node)?)
        }

        Ok(
            Node::Loop(LoopNode {
                body,
                return_type: DefaultTypeIds::unit(),
            })
        )
    }
}