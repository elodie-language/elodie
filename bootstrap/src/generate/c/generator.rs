use crate::generate::c;
use crate::generate::c::{BlockStatement, DefineFunctionNode, IncludeSystemDirectiveNode, Indent, LiteralExpression, LiteralIntExpression, ReturnFromFunctionStatement};
use crate::generate::c::DirectiveNode::IncludeSystemDirective;
use crate::generate::c::Expression::Literal;
use crate::generate::c::Node::{DefineFunction, Directive};
use crate::generate::c::Statement::ReturnFromFunction;
use crate::ir;

pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn generate(ctx: &ir::Context) -> Result<Vec<c::Node>> {
    let mut generator = Generator {};
    generator.generate(ctx)
}

pub(crate) struct Generator {}

impl Generator {
    pub(crate) fn generate(&mut self, ctx: &ir::Context) -> Result<Vec<c::Node>> {
        Ok(
            vec![
                Directive(IncludeSystemDirective(IncludeSystemDirectiveNode {
                    indent: Indent::none(),
                    path: "stdio.h".to_string(),
                })),
                DefineFunction(DefineFunctionNode {
                    indent: Indent::none(),
                    identifier: "main".to_string(),
                    arguments: vec![].into_boxed_slice(),
                    ty: "int".to_string(),

                    statements: BlockStatement {
                        indent: Indent::none(),
                        statements: vec![
                            ReturnFromFunction(ReturnFromFunctionStatement {
                                indent: Indent::none(),
                                node: Some(Literal(LiteralExpression::Int(LiteralIntExpression { indent: Indent::none(), value: 0 }))),
                            })
                        ],
                    },
                })],
        )
    }
}