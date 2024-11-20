use std::ops::Deref;

use crate::ast::{ast, parse, SourceFile};
use crate::ast::compile::symbol::SymbolTable;
use crate::ast::parse::RootNode;

mod r#let;
mod typesystem;
mod symbol;
mod validate;
mod collect;
mod infix;
mod literal;
mod r#loop;
mod r#if;
mod identifier;
mod block;
mod function;
mod package;

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) fn from(node: RootNode) -> Result<SourceFile> {
    let mut compiler = Compiler::default();
    compiler.compile(node)
}

pub(crate) struct Compiler {
    symbol_table: SymbolTable,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            symbol_table: Default::default(),
        }
    }
}

impl Compiler {
    pub(crate) fn compile(&mut self, node: RootNode) -> Result<SourceFile> {
        // 2 pass
        // populate symbol table
        // create ast

        let mut result = Vec::new();
        for node in &node.nodes {
            // match node {
            //     Node::Infix(InfixNode { left, right, .. }) => {
            //         let Node::Identifier(object_identifier) = left.deref() else { todo!() };
            //         let Node::Infix(InfixNode { left, operator, right }) = right.deref() else { todo!() };
            //         let Node::Identifier(function_identifier) = left.deref() else { todo!() };
            //         let Node::Tuple(tuple) = right.deref() else { todo!() };
            //         let Node::Literal(LiteralNode::String(value)) = &tuple.nodes[0] else { todo!() };
            //
            //         result.push(
            //             Ast::CallFunctionOfObject {
            //                 object: ast::ObjectIdentifier(object_identifier.value().to_string()),
            //                 function: ast::FunctionIdentifier(function_identifier.value().to_string()),
            //                 arguments: vec![Ast::StringValue(value.value().to_string())],
            //             }
            //         )
            //     }
            //     Node::Let(let_node) => {}
            //     _ => todo!()
            // }
            result.push(self.compile_node(node)?);
        }

        Ok(SourceFile { body: result })
    }

    pub(crate) fn compile_node(&mut self, node: &parse::Node) -> Result<ast::Node> {
        match node {
            parse::Node::Block(block_node) => Ok(self.compile_block(block_node)?),
            parse::Node::Break(break_node) => Ok(self.compile_break(break_node)?),
            parse::Node::Continue(continue_node) => Ok(self.compile_continue(continue_node)?),
            parse::Node::FunctionDeclaration(declaration_node) => Ok(self.compile_declare_function(declaration_node)?),
            parse::Node::PackageDeclaration(declaration_node) => Ok(self.compile_declare_package(declaration_node)?),
            parse::Node::Identifier(identifier_node) => Ok(self.compile_identifier(identifier_node)?),
            parse::Node::Let(let_node) => Ok(self.compile_let(let_node)?),
            parse::Node::If(if_node) => Ok(self.compile_if(if_node)?),
            parse::Node::Infix(infix_node) => Ok(self.compile_infix(infix_node)?),
            parse::Node::Literal(literal_node) => Ok(self.compile_literal(literal_node)?),
            parse::Node::Loop(loop_node) => Ok(self.compile_loop(loop_node)?),
            parse::Node::Return(return_node) => Ok(self.compile_function_return(return_node)?),
            _ => unimplemented!("{:?}", node)
        }
    }
}