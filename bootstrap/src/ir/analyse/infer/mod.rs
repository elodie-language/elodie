use std::ops::Deref;

use crate::common::node::Node::{DeclareVariable, LiteralBoolean, LiteralNumber, LiteralString};
use crate::common::{StringTable, WithSpan};
use crate::frontend;
use crate::frontend::ast::AstTreeNode;
use crate::ir::analyse::AnalyseTreeNode;
use crate::ir::context::Context;
use crate::ir::symbol::{SymbolId, SymbolName, SymbolTable};

mod declare;
mod literal;
mod r#type;

pub(crate) struct Inference<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
}

impl<'a> Inference<'a> {
    pub(crate) fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
        }
    }

    pub(crate) fn infer(
        &mut self,
        ast: frontend::Ast,
    ) -> crate::ir::analyse::Result<Vec<AnalyseTreeNode>> {
        let mut nodes = vec![];
        for node in &ast.nodes {
            nodes.push(self.infer_node(node)?);
        }
        Ok(nodes)
    }

    fn infer_node(&mut self, ast: &AstTreeNode) -> crate::ir::analyse::Result<AnalyseTreeNode> {
        match ast.node() {
            DeclareVariable(node) => self.infer_declare_variable(ast.span(), node),
            LiteralBoolean(node) => self.infer_literal_boolean(ast.span(), node),
            LiteralNumber(node) => self.infer_literal_number(ast.span(), node),
            LiteralString(node) => self.infer_literal_string(ast.span(), node),
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
