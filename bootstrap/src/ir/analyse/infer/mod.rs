use crate::common::{StringTable, WithSpan};
use crate::frontend::{ast, NewAst};
use crate::frontend::ast::Ast;
use crate::frontend::ast::node::AstNode;
use crate::ir::analyse::AnalysedNode;
use crate::ir::context::Context;
use crate::ir::symbol::{SymbolId, SymbolName, SymbolTable};

mod literal;
mod declare;
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

    pub(crate) fn infer(&mut self, ast: NewAst) -> crate::ir::analyse::Result<Vec<AnalysedNode>> {
        let mut nodes = vec![];
        for node in &ast.nodes {
            nodes.push(self.infer_node(node)?);
        }
        Ok(nodes)
    }

    fn infer_node(&mut self, ast: &AstNode) -> crate::ir::analyse::Result<AnalysedNode> {
        match ast.node() {
            ast::Node::DeclareVariable(node) => self.infer_declare_variable(ast.span(), node),
            ast::Node::LiteralBoolean(node) => self.infer_literal_boolean(ast.span(), node),
            ast::Node::LiteralNumber(node) => self.infer_literal_number(ast.span(), node),
            ast::Node::LiteralString(node) => self.infer_literal_string(ast.span(), node),
            // ast::Node::Literal(node) => self.infer_literal(node),
            _ => unimplemented!("{ast:#?}")
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
