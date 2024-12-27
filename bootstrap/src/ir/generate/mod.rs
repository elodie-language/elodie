use std::ops::Index;

use crate::common::{StringTable, SymbolTable, TypeTable, WithSpan};
use crate::common::Context;
use crate::common::node::Node::{AccessVariable, CallFunctionOfPackage, DeclareVariable, InterpolateString, LiteralBoolean, LiteralNumber, LiteralString};
use crate::ir::analyse::{TypedAst, TypedTreeNode};
use crate::ir::Ir;
use crate::ir::node::IrTreeNode;

mod literal;
mod declare;
mod call;
mod string;
mod access;

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
            AccessVariable(inner) => self.access_variable(inner, node.span()),
            CallFunctionOfPackage(inner) => self.call_function_of_package(inner, node.span()),
            DeclareVariable(inner) => self.declare_variable(inner, node.span()),
            InterpolateString(inner) => self.interpolate_string(inner, node.span()),
            LiteralBoolean(inner) => self.literal_boolean(inner, node.span()),
            LiteralNumber(inner) => self.literal_number(inner, node.span()),
            LiteralString(inner) => self.literal_string(inner, node.span()),
            _ => unimplemented!("{node:#?}")
        }
    }
}

pub(crate) fn generate(ctx: &mut Context, typed: TypedAst) -> Result<Ir> {
    let nodes = Generator::new(ctx).generate(typed)?;
    Ok(Ir { nodes })
}
