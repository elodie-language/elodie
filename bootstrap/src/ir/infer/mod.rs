use std::collections::HashMap;
use std::ops::Index;

use crate::common::{Context, StringTableId};
use crate::frontend::{parse, Parsed};
use crate::ir::infer::node::Node;

mod node;
mod literal;

#[derive(Debug, Clone, PartialEq)]
pub enum InferredType {
    Unknown,

    Boolean,
    Number,
    String,
    Tuple(Vec<InferredType>),
    Type(HashMap<StringTableId, InferredType>),

    OneOf(Vec<InferredType>),
    AllOf(Vec<InferredType>),

}

impl InferredType {}

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Inferred {
    pub nodes: Vec<Node>,
}

impl Index<usize> for Inferred {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

pub(crate) fn infer(ctx: &mut Context, parsed: Parsed) -> Result<Inferred> {
    Ok(Inferred { nodes: Inference::new(ctx).infer(parsed)? })
}

struct Inference<'a> {
    ctx: &'a mut Context,
}

impl<'a> Inference<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }

    fn infer(&mut self, parsed: Parsed) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        for node in parsed.nodes {
            if !matches!(node, parse::Node::Nop) {
                nodes.push(self.infer_node(node)?);
            }
        }
        Ok(nodes)
    }


    fn infer_node(&mut self, node: parse::Node) -> Result<Node> {
        match node {
            parse::Node::Literal(node) => self.infer_literal(node),
            _ => unimplemented!("{node:#?}")
        }
    }
}