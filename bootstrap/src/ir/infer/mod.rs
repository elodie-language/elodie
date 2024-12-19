use std::collections::HashMap;
use std::ops::Index;

pub use node::*;

use crate::common::{StringTable, StringTableId};
use crate::frontend::{ast, Ast};
use crate::ir::context::Context;
use crate::ir::infer::node::Node;
use crate::ir::symbol::{SymbolId, SymbolName, SymbolTable};

mod node;
mod literal;
mod declare;
mod r#type;

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
pub struct Inferred<'a> {
    pub nodes: Vec<Node<'a>>,
}

impl<'a> Index<usize> for Inferred<'a> {
    type Output = Node<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

pub(crate) fn infer<'a>(ctx: &'a mut Context) -> Result<Inferred<'a>> {
    Ok(Inferred { nodes: Inference::new(ctx).infer()? })
}

struct Inference<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
    ast: &'a Ast,
}

impl<'a> Inference<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
            ast: &ctx.ast,
        }
    }

    fn infer(&mut self) -> Result<Vec<Node<'a>>> {
        let mut nodes = vec![];
        for node in &self.ast.nodes {
            nodes.push(self.infer_node(node)?);
        }
        Ok(nodes)
    }

    fn infer_node(&mut self, node: &'a ast::Node) -> Result<Node<'a>> {
        match node {
            ast::Node::DeclareVariable(node) => self.infer_declare_variable(node),
            ast::Node::Literal(node) => self.infer_literal(node),
            _ => unimplemented!("{node:#?}")
        }
    }

    fn register_argument(&mut self, name: SymbolName) -> SymbolId {
        // self.ctx.symbol_table.register_argument(name)
        todo!()
    }

    // fn register_function(&mut self, name: SymbolName) -> SymbolId {
    //     self.ctx.symbol_table.register_function(name)
    // }
    //
    // fn register_package(&mut self, name: SymbolName) -> SymbolId {
    //     self.ctx.symbol_table.register_package(name)
    // }
    //
    // fn register_type(&mut self, name: SymbolName) -> SymbolId {
    //     self.ctx.symbol_table.register_type(name)
    // }
    //
    fn register_variable(&mut self, name: SymbolName) -> SymbolId {
        self.symbol_table.register_variable(name)
    }
}
