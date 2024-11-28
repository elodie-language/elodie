use crate::generate::c;
use crate::generate::c::{BlockStatement, CallFunctionExpression, DefineFunctionNode, Expression, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode, Indent, LiteralExpression, LiteralIntExpression, LiteralStringExpression, ReturnFromFunctionStatement, Statement};
use crate::generate::c::DirectiveNode::{IncludeLocalDirective, IncludeSystemDirective};
use crate::generate::c::Expression::{CallFunction, Literal};
use crate::generate::c::Node::{DefineFunction, Directive};
use crate::generate::c::Statement::ReturnFromFunction;
use crate::ir;
use crate::ir::{CallFunctionOfPackageNode, Node};

pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn generate(ctx: &ir::Context) -> Result<Vec<c::Node>> {
    let mut generator = Generator {};
    generator.generate(ctx)
}

pub(crate) struct Generator {}

impl Generator {

    pub(crate) fn generate(&mut self, ctx: &ir::Context) -> Result<Vec<c::Node>> {
        let mut result = vec![
            Directive(IncludeSystemDirective(IncludeSystemDirectiveNode {
                indent: Indent::none(),
                path: "stdio.h".to_string(),
            })),
            Directive(IncludeLocalDirective(IncludeLocalDirectiveNode {
                indent: Indent::none(),
                path: "std_io.h".to_string(),
            })),
        ];

        let mut statements = vec![];

        for node in &ctx.file.body {
            match node {
                Node::Block(_) => {}
                Node::BreakLoop(_) => {}
                Node::Calculate(_) => {}
                Node::CallFunctionOfObject(_) => {}
                Node::CallFunctionOfPackage(CallFunctionOfPackageNode { package, function, arguments }) => {
                    let std = ctx.string_cache.get(package.segments[0]);
                    let io = ctx.string_cache.get(package.segments[1]);
                    dbg!(package);
                    dbg!(function);
                    let function = ctx.string_cache.get(function.0);
                    dbg!(arguments);

                    statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                        indent: Indent::none(),
                        identifier: format!("{std}_{io}_{function}"),
                        arguments: Box::new([
                            Literal(LiteralExpression::String(LiteralStringExpression{
                             indent: Indent::none(),
                             value: "hi<->fish".to_string() }))
                        ]),
                    })));

                    statements.push( ReturnFromFunction(ReturnFromFunctionStatement {
                        indent: Indent::none(),
                        node: Some(Literal(LiteralExpression::Int(LiteralIntExpression { indent: Indent::none(), value: 0 }))),
                    }));
                }
                Node::CallFunction(_) => {}
                Node::CallFunctionWithLambda(_) => {}
                Node::ExportPackage(_) => {}
                Node::ReturnFromFunction(_) => {}
                Node::ContinueLoop(_) => {}
                Node::Compare(_) => {}
                Node::If(_) => {}
                Node::LoadValue(_) => {}
                Node::LoadValueFromObject(_) => {}
                Node::LoadValueFromSelf(_) => {}
                Node::Loop(_) => {}
                Node::ValueNumber(_) => {}
                Node::ValueString(_) => {}
                Node::ValueBoolean(_) => {}
                Node::ValueUnit => {}
                Node::DeclareVariable(_) => {}
                Node::DeclareFunction(_) => {}
                Node::DeclarePackage(_) => {}
                Node::DeclareType(_) => {}
                Node::InstantiateType(_) => {}
                Node::DefineType(_) => {}
            }
        }

        Ok(
            vec![
                Directive(IncludeSystemDirective(IncludeSystemDirectiveNode {
                    indent: Indent::none(),
                    path: "stdio.h".to_string(),
                })),
                Directive(IncludeLocalDirective(IncludeLocalDirectiveNode {
                    indent: Indent::none(),
                    path: "std_io.h".to_string(),
                })),
                DefineFunction(DefineFunctionNode {
                    indent: Indent::none(),
                    identifier: "main".to_string(),
                    arguments: vec![].into_boxed_slice(),
                    ty: "int".to_string(),

                    statements: BlockStatement {
                        indent: Indent::none(),
                        statements
                    },
                })],
        )
    }
}