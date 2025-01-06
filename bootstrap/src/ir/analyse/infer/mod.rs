use std::ops::Deref;

use crate::common::{StringTable, SymbolTable, TypeTable, WithSpan};
use crate::common::Context;
use crate::ir::analyse::{TypedTreeNode, TypeNode};
use crate::ir::analyse::scope::Scope;

mod variable;
mod call;
mod block;
mod control;

pub(crate) struct Inferrer<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
    type_table: &'a mut TypeTable,
    scope: Scope,
}


impl<'a> Inferrer<'a> {
    pub(crate) fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
            type_table: &mut ctx.type_table,
            scope: Scope::new(),
        }
    }

    pub(crate) fn infer_nodes(
        &mut self,
        nodes: &mut Vec<TypedTreeNode>,
    ) -> crate::ir::analyse::Result<()> {
        for node in &mut *nodes {
            self.node(node)?
        }
        Ok(())
    }

    pub(crate) fn node(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        match node.node() {
            TypeNode::Block(_) => self.block(node),
            TypeNode::BreakLoop(_) => self.r#break(node),
            TypeNode::Calculate(_) => Ok({}),
            TypeNode::CallFunctionOfPackage(_) => self.call_function_of_package(node),
            TypeNode::Compare(_) => Ok({}),
            TypeNode::DeclareVariable(_) => self.declare_variable(node),
            TypeNode::If(_) => self.r#if(node),
            TypeNode::LiteralBoolean(_) |
            TypeNode::LiteralFloat4(_) |
            TypeNode::LiteralFloat8(_) |
            TypeNode::LiteralInt1(_) |
            TypeNode::LiteralInt2(_) |
            TypeNode::LiteralInt4(_) |
            TypeNode::LiteralInt8(_) |
            TypeNode::LiteralInt16(_) |
            TypeNode::LiteralUint1(_) |
            TypeNode::LiteralUint2(_) |
            TypeNode::LiteralUint4(_) |
            TypeNode::LiteralUint8(_) |
            TypeNode::LiteralUint16(_) |
            TypeNode::LiteralNumber(_) |
            TypeNode::LiteralString(_) => { Ok(()) }
            TypeNode::Loop(_) => self.r#loop(node),
            _ => unimplemented!("{node:#?}")
        }
    }
}
