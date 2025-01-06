use std::ops::Index;

use crate::common::{StringTable, SymbolTable, TypeTable, WithSpan};
use crate::common::Context;
use crate::common::node::Node::{AccessVariable, Block, BreakLoop, Calculate, CallFunctionOfPackage, Compare, DeclareVariable, If, InterpolateString, LiteralBoolean, LiteralFloat4, LiteralFloat8, LiteralInt1, LiteralInt16, LiteralInt2, LiteralInt4, LiteralInt8, LiteralNumber, LiteralString, LiteralUint1, LiteralUint16, LiteralUint2, LiteralUint4, LiteralUint8, Loop};
use crate::ir::analyse::{TypedAst, TypedTreeNode};
use crate::ir::Ir;
use crate::ir::node::IrTreeNode;

mod literal;
mod declare;
mod call;
mod string;
mod access;
mod block;
mod control;
mod compare;
mod calculate;

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
            Block(inner) => self.block(inner, node.span()),
            BreakLoop(inner) => self.r#break(inner, node.span()),
            Calculate(inner) => self.calculate(inner, node.span()),
            CallFunctionOfPackage(inner) => self.call_function_of_package(inner, node.span()),
            Compare(inner) => self.compare(inner, node.span()),
            DeclareVariable(inner) => self.declare_variable(inner, node.span()),
            If(inner) => self.r#if(inner, node.span()),
            InterpolateString(inner) => self.interpolate_string(inner, node.span()),
            LiteralBoolean(inner) => self.literal_boolean(inner, node.span()),
            LiteralFloat4(inner) => self.literal_float4(inner, node.span()),
            LiteralFloat8(inner) => self.literal_float8(inner, node.span()),
            LiteralInt1(inner) => self.literal_int1(inner, node.span()),
            LiteralInt2(inner) => self.literal_int2(inner, node.span()),
            LiteralInt4(inner) => self.literal_int4(inner, node.span()),
            LiteralInt8(inner) => self.literal_int8(inner, node.span()),
            LiteralInt16(inner) => self.literal_int16(inner, node.span()),
            LiteralNumber(inner) => self.literal_number(inner, node.span()),
            LiteralString(inner) => self.literal_string(inner, node.span()),
            LiteralUint1(inner) => self.literal_uint1(inner, node.span()),
            LiteralUint2(inner) => self.literal_uint2(inner, node.span()),
            LiteralUint4(inner) => self.literal_uint4(inner, node.span()),
            LiteralUint8(inner) => self.literal_uint8(inner, node.span()),
            LiteralUint16(inner) => self.literal_uint16(inner, node.span()),
            Loop(inner) => self.r#loop(inner, node.span()),
            _ => unimplemented!("{node:#?}")
        }
    }
}

pub(crate) fn generate(ctx: &mut Context, typed: TypedAst) -> Result<Ir> {
    let nodes = Generator::new(ctx).generate(typed)?;
    Ok(Ir { nodes })
}
