use std::ops::Deref;
use std::rc::Rc;

use crate::common::node::Node;
use crate::frontend::ast::{
    AstBlockNode, AstBreakLoopNode, AstContinueLoopNode, AstIfNode, AstLoopNode, AstNode,
    AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED,
};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_break(&mut self, node: &parse::BreakNode) -> ast::Result<AstTreeNode> {
        if node.result.is_none() {
            Ok(AstTreeNode::new(
                AstNode::BreakLoop(AstBreakLoopNode { node: None }),
                SPAN_NOT_IMPLEMENTED.clone(),
            ))
        } else {
            let node = Some(Rc::new(self.generate_node(node.result.as_ref().unwrap())?));
            Ok(AstTreeNode::new(
                AstNode::BreakLoop(AstBreakLoopNode { node }),
                SPAN_NOT_IMPLEMENTED.clone(),
            ))
        }
    }

    pub(crate) fn generate_continue(
        &mut self,
        _node: &parse::ContinueNode,
    ) -> ast::Result<AstTreeNode> {
        Ok(AstTreeNode::new(
            Node::ContinueLoop(AstContinueLoopNode {}),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }

    pub(crate) fn generate_loop(&mut self, node: &parse::LoopNode) -> ast::Result<AstTreeNode> {
        let mut nodes = Vec::with_capacity(node.block.nodes.len());

        for node in &node.block.nodes {
            nodes.push(self.generate_node(node)?)
        }

        Ok(AstTreeNode::new(
            Node::Loop(AstLoopNode { nodes }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }

    pub(crate) fn generate_if(&mut self, node: &parse::IfNode) -> ast::Result<AstTreeNode> {
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
            Some(Rc::new(AstBlockNode {
                nodes: otherwise_body,
            }))
        } else {
            None
        };

        Ok(AstTreeNode::new(
            Node::If(AstIfNode {
                condition,
                then: Rc::new(AstBlockNode { nodes: then_body }),
                otherwise,
            }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }
}
