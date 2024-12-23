use std::ops::Deref;

use crate::common::{StringTable, WithSpan};
use crate::common::context::Context;
use crate::common::node::Node::{DeclareVariable, LiteralBoolean, LiteralNumber, LiteralString};
use crate::frontend;
use crate::frontend::ast::AstTreeNode;
use crate::ir::analyse::TypedTreeNode;
use crate::ir::symbol::{SymbolId, SymbolName, SymbolTable};
use crate::ir::TypeTable;

mod declare;
mod literal;

pub(crate) struct Pre<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
    type_table: &'a mut TypeTable,
}

impl<'a> Pre<'a> {

    pub(crate) fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
            type_table: &mut ctx.type_table,
        }
    }

    pub(crate) fn process(
        &mut self,
        ast: frontend::Ast,
    ) -> crate::ir::analyse::Result<Vec<TypedTreeNode>> {
        let mut nodes = vec![];
        for node in &ast.nodes {
            nodes.push(self.node(node)?);
        }
        Ok(nodes)
    }

    fn node(&mut self, ast: &AstTreeNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        match ast.node() {
            DeclareVariable(node) => self.declare_variable(ast.span(), node),
            LiteralBoolean(node) => self.literal_boolean(ast.span(), node),
            LiteralNumber(node) => self.literal_number(ast.span(), node),
            LiteralString(node) => self.literal_string(ast.span(), node),
            _ => unimplemented!("{ast:#?}"),
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
