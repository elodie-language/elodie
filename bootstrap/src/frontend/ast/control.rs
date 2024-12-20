use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::{ast, parse};
use crate::frontend::ast::{BlockNode, Generator, IfNode, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::node::{AstNode, BreakLoopNode, ContinueLoopNode, LoopNode, Node};

impl<'a> Generator<'a> {
    pub(crate) fn generate_break(&mut self, node: &parse::BreakNode) -> ast::Result<AstNode> {
        if node.result.is_none() {
            Ok(AstNode::new(Node::BreakLoop(BreakLoopNode {
                node: None,
            }), SPAN_NOT_IMPLEMENTED.clone()))
        } else {
            let node = Some(Rc::new(self.generate_node(node.result.as_ref().unwrap())?));
            Ok(AstNode::new(Node::BreakLoop(BreakLoopNode { node }), SPAN_NOT_IMPLEMENTED.clone()))
        }
    }

    pub(crate) fn generate_continue(&mut self, _node: &parse::ContinueNode) -> ast::Result<AstNode> {
        Ok(AstNode::new(Node::ContinueLoop(ContinueLoopNode {}), SPAN_NOT_IMPLEMENTED.clone()))
    }

    pub(crate) fn generate_loop(&mut self, node: &parse::LoopNode) -> ast::Result<AstNode> {
        let mut nodes = Vec::with_capacity(node.block.nodes.len());

        for node in &node.block.nodes {
            nodes.push(self.generate_node(node)?)
        }

        Ok(AstNode::new(Node::Loop(LoopNode { nodes }), SPAN_NOT_IMPLEMENTED.clone()))
    }

    pub(crate) fn generate_if(&mut self, node: &parse::IfNode) -> ast::Result<AstNode> {
        // condition needs to be of type boolean --> every node has a type?!
        let condition = Rc::new(self.generate_node(node.condition.deref())?);

        let mut then_body = vec![];
        for node in &node.then.nodes {
            then_body.push(self.generate_node(node.deref())?)
        }

        let otherwise = if node.otherwise.is_some() {
            let mut otherwise_body = vec![];
            for node in &node.otherwise.as_ref().unwrap().block.nodes {
                otherwise_body.push(self.generate_node(node)?)
            }
            Some(Rc::new(BlockNode { nodes: otherwise_body }))
        } else {
            None
        };

        Ok(AstNode::new(Node::If(IfNode {
            condition,
            then: Rc::new(BlockNode { nodes: then_body }),
            otherwise,
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
