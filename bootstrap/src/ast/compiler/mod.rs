use std::ops::Deref;
use ast::Ast;

use crate::ast;
use crate::ast::compiler::symbol::SymbolTable;
use crate::ast::parse::node::{InfixNode, LiteralNode, Node, RootNode};
use crate::ast::SourceFile;

mod symbol;

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
        let mut result = Vec::new();

        for node in node.nodes {
            match node {
                Node::Infix(InfixNode { left, operator, right }) => {
                    let Node::Identifier(object_identifier) = left.deref() else { todo!() };
                    let Node::Infix(InfixNode { left, operator, right }) = right.deref() else { todo!() };
                    let Node::Identifier(function_identifier) = left.deref() else { todo!() };
                    let Node::Tuple(tuple) = right.deref() else { todo!() };
                    let Node::Literal(LiteralNode::String(value)) = tuple.nodes.first().unwrap() else { todo!() };

                    result.push(
                        Ast::CallFunctionOfObject {
                            object: object_identifier.identifier().to_string(),
                            function: function_identifier.identifier().to_string(),
                            arguments: vec![Ast::StringValue(value.value().to_string())],
                        }
                    )
                }
                _ => todo!()
            }
        }

        Ok(SourceFile { body: result })
    }
}