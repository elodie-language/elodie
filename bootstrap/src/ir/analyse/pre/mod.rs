use std::ops::Deref;

use Node::{Block, If};

use crate::common::{GetString, Inferred, Span, StringTable, SymbolId, SymbolName, SymbolTable, TypeTable, VariableSymbol, WithSpan};
use crate::common::Context;
use crate::common::node::Node;
use crate::common::node::Node::{AccessVariable, CallFunctionOfPackage, Compare, DeclareVariable, InterpolateString, LiteralBoolean, LiteralNumber, LiteralString};
use crate::frontend;
use crate::frontend::ast::AstTreeNode;
use crate::ir::analyse::{Error, TypedTreeNode, UndefinedError};
use crate::ir::analyse::scope::Scope;

mod variable;
mod literal;
mod call;
mod string;
mod access;
mod control;
mod block;
mod compare;

pub(crate) struct Pre<'a> {
    string_table: &'a mut StringTable,
    symbol_table: &'a mut SymbolTable,
    type_table: &'a mut TypeTable,
    scope: Scope,
}

impl<'a> Pre<'a> {
    pub(crate) fn new(ctx: &'a mut Context) -> Self {
        Self {
            string_table: &mut ctx.string_table,
            symbol_table: &mut ctx.symbol_table,
            type_table: &mut ctx.type_table,
            scope: Scope::new(),
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
        self.scope.span_set(ast.span());

        match ast.node() {
            AccessVariable(node) => self.access_variable(node),
            Block(node) => self.block(node),
            CallFunctionOfPackage(node) => self.call_function_of_package(node),
            Compare(node) => self.compare(node),
            DeclareVariable(node) => self.declare_variable(node),
            If(node) => self.r#if(node),
            InterpolateString(node) => self.interpolate_string(node),
            LiteralBoolean(node) => self.literal_boolean(node),
            LiteralNumber(node) => self.literal_number(node),
            LiteralString(node) => self.literal_string(node),
            _ => unimplemented!("{ast:#?}"),
        }
    }

    fn span(&self) -> Span { self.scope.span_get() }

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

    fn variable_register(&mut self, name: SymbolName, inferred: Inferred) -> SymbolId {
        let result = self.symbol_table.register_variable(name, inferred);
        let symbol = &mut self.symbol_table[result];
        self.scope.register_symbol(&symbol);
        result
    }

    fn variable_get(&self, name: impl AsRef<SymbolName>) -> crate::ir::analyse::Result<&VariableSymbol> {
        let id = self.scope.variable(name.as_ref())
            .ok_or(Error::Undefined(UndefinedError::UndefinedVariable {
                variable: self.string_table.get_string(name),
                span: self.span(),
            }))?;

        Ok(self.symbol_table.variable(id))
    }
}
