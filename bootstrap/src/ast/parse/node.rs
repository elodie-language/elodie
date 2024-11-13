use std::ops::Index;

use crate::ast::token::Token;

pub struct RootNode {
    nodes: Vec<Node>,
}

impl Index<usize> for RootNode {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl From<Vec<Node>> for RootNode {
    fn from(value: Vec<Node>) -> Self {
        Self { nodes: value }
    }
}

impl RootNode {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Block(BlockNode),
    Literal(LiteralNode),
}

#[derive(Debug, PartialEq)]
pub struct BlockNode {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode {
    Number {
        token: Token,
        value: f64,
    },
    String {
        token: Token,
        value: String,
    },
    Boolean {
        token: Token,
        value: bool,
    },
}
