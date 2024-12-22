use std::ops::Index;

use crate::common::{StringTable, WithSpan};
use crate::common::context::Context;
use crate::common::node::Node::LiteralNumber;
use crate::ir::{Ir, SymbolTable, TypeTable};
use crate::ir::analyse::{TypedAst, TypedTreeNode};
use crate::ir::node::IrTreeNode;

mod literal;

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) struct Generator<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
    type_table: &'a mut TypeTable,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
            type_table: &mut ctx.type_table,
        }
    }

    pub(crate) fn generate(&mut self, typed: TypedAst) -> Result<Vec<IrTreeNode>> {
        let mut result = vec![];
        for node in &typed.nodes {
            result.push(self.node(node)?)
        }
        Ok(result)
    }

    pub(crate) fn node(&mut self, node: &TypedTreeNode) -> Result<IrTreeNode> {
        match &node.node {
            LiteralNumber(inner) => self.literal_number(inner, node.span()),
            _ => unimplemented!("{node:#?}")
        }
    }
}

pub(crate) fn generate(ctx: &mut Context, typed: TypedAst) -> Result<Ir> {
    let nodes = Generator::new(ctx).generate(typed)?;
    Ok(Ir { nodes })
}
