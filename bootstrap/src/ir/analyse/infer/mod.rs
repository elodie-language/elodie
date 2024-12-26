use std::ops::Deref;

use crate::common::{StringTable, SymbolTable, TypeTable, WithSpan};
use crate::common::context::Context;
use crate::ir::analyse::{TypedTreeNode, TypeNode};
use crate::ir::analyse::scope::Scope;

mod variable;
mod call;

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
            TypeNode::CallFunctionOfPackage(_) => self.call_function_of_package(node),
            TypeNode::DeclareVariable(_) => self.declare_variable(node),
            TypeNode::LiteralBoolean(_) |
            TypeNode::LiteralNumber(_) |
            TypeNode::LiteralString(_) => { Ok(()) }
            _ => unimplemented!()
        }
    }
}
