use std::ops::Deref;

use crate::generate::c;
use crate::generate::c::{BlockStatement, CallFunctionExpression, DeclareArrayStatement, DeclareVariableStatement, DefineFunctionNode, IncludeLocalDirectiveNode, IncludeSystemDirectiveNode, Indent, LiteralDoubleExpression, LiteralExpression, LiteralIntExpression, LiteralStringExpression, ReturnFromFunctionStatement, Statement, VariableExpression};
use crate::generate::c::DirectiveNode::{IncludeLocalDirective, IncludeSystemDirective};
use crate::generate::c::Expression::{CallFunction, Literal, Variable};
use crate::generate::c::Node::{DefineFunction, Directive};
use crate::generate::c::Statement::ReturnFromFunction;
use crate::ir;
use crate::ir::{CallFunctionNode, CallFunctionOfPackageNode, DeclareVariableNode, LoadValueNode, Node};
use crate::ir::Node::CallFunctionOfPackage;

pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn generate(ctx: &ir::Context) -> Result<Vec<c::Node>> {
    let mut generator = Generator {};
    generator.generate(ctx)
}

pub(crate) struct Generator {}

impl Generator {
    pub(crate) fn generate(&mut self, ctx: &ir::Context) -> Result<Vec<c::Node>> {
        let mut statements = vec![];

        for node in &ctx.file.body {
            dbg!(node);

            match node {
                Node::Block(_) => {}
                Node::BreakLoop(_) => {}
                Node::Calculate(_) => {}
                Node::CallFunctionOfObject(_) => {}
                Node::CallFunctionOfPackage(CallFunctionOfPackageNode { package, function, arguments }) => {
                    let std = ctx.string_cache.get(package.segments[0]);
                    let io = ctx.string_cache.get(package.segments[1]);
                    let function = ctx.string_cache.get(function.0);

                    if let ir::Node::ValueString(s) = arguments.get(0).unwrap() {
                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: format!("{std}_{io}_{function}"),
                            arguments: Box::new([
                                Literal(LiteralExpression::String(LiteralStringExpression {
                                    indent: Indent::none(),
                                    value: s.to_string(),
                                }))
                            ]),
                        })));
                    }

                    if let ir::Node::ValueNumber(f) = arguments.get(0).unwrap() {
                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: format!("{std}_{io}_{function}"),
                            arguments: Box::new([
                                Literal(LiteralExpression::Double(LiteralDoubleExpression {
                                    indent: Indent::none(),
                                    value: *f,
                                }))
                            ]),
                        })));
                    }


                    if let ir::Node::LoadValue(LoadValueNode { identifier, type_id }) = arguments.get(0).unwrap() {
                        statements.push(Statement::DeclareArray(DeclareArrayStatement {
                            indent: Indent::none(),
                            identifier: "str".to_string(),
                            r#type: "char".to_string(),
                            size: 20,
                            // expression: (),
                        }));

                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: format!("snprintf"),
                            arguments: Box::new([
                                Variable(VariableExpression {
                                    indent: Indent::none(),
                                    identifier: "str".to_string(),
                                }),
                                Literal(LiteralExpression::Int(LiteralIntExpression {
                                    indent: Indent::none(),
                                    value: 20,
                                })),
                                Literal(LiteralExpression::String(LiteralStringExpression {
                                    indent: Indent::none(),
                                    value: "%.1f".to_string(),
                                })),
                                Variable(VariableExpression {
                                    indent: Indent::none(),
                                    identifier: ctx.string_cache.get(identifier.0).to_string(),
                                }),
                            ]),
                        })));

                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: format!("{std}_{io}_{function}"),
                            arguments: Box::new([
                                Variable(VariableExpression {
                                    indent: Indent::none(),
                                    identifier: "str".to_string(),
                                })
                            ]),
                        })));
                    }

                    statements.push(ReturnFromFunction(ReturnFromFunctionStatement {
                        indent: Indent::none(),
                        node: Some(Literal(LiteralExpression::Int(LiteralIntExpression { indent: Indent::none(), value: 0 }))),
                    }));
                }
                Node::CallFunction(CallFunctionNode { function, arguments }) => {
                    let function = ctx.string_cache.get(function.0);


                    if let ir::Node::ValueNumber(f) = arguments.get(0).unwrap() {
                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: function.to_string(),
                            arguments: Box::new([
                                Literal(LiteralExpression::Double(LiteralDoubleExpression {
                                    indent: Indent::none(),
                                    value: *f,
                                }))
                            ]),
                        })));
                    }
                }
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

                Node::DeclareVariable(DeclareVariableNode { identifier, value, value_type }) => {
                    if let CallFunctionOfPackage(CallFunctionOfPackageNode { package, function, arguments }) = value.deref() {
                        if package.segments.len() == 3 {
                            // HACK for core::intrinsics::math
                            let core = ctx.string_cache.get(package.segments[0]);
                            let intrinsics = ctx.string_cache.get(package.segments[1]);
                            let math = ctx.string_cache.get(package.segments[2]);
                            let func = ctx.string_cache.get(function.0);

                            let ir::Node::ValueNumber(f) = arguments.get(0).unwrap() else { panic!() };

                            statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                                indent: Indent::none(),
                                identifier: "result".to_string(),
                                r#type: "double".to_string(),
                                expression:
                                CallFunction(CallFunctionExpression {
                                    indent: Indent::none(),
                                    identifier: format!("{}_{}_{}_{}", core, intrinsics, math, func),
                                    arguments: Box::new([
                                        Literal(LiteralExpression::Double(LiteralDoubleExpression {
                                            indent: Indent::none(),
                                            value: *f,
                                        }))
                                    ]),
                                }),
                            }))
                        }
                    } else {

                        // statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                        //     indent: Indent::none(),
                        //     identifier: "result".to_string(),
                        //     r#type: "double".to_string(),
                        //     expression:
                        //     CallFunction(CallFunctionExpression {
                        //         indent: Indent::none(),
                        //         identifier: "cos".to_string(),
                        //         arguments: Box::new([
                        //             Literal(LiteralExpression::Double(LiteralDoubleExpression {
                        //                 indent: Indent::none(),
                        //                 value: 0.0f64,
                        //             }))
                        //         ]),
                        //     }),
                        // }))
                        unimplemented!()
                    }
                }

                Node::DeclareFunction(_) => {}
                Node::DeclarePackage(_) => {}
                Node::DeclareType(_) => {}
                Node::InstantiateType(_) => {}
                Node::DefineType(_) => {}
                Node::DeclareExternalFunction(_) => {}
            }
        }

        dbg!(&statements);

        Ok(
            vec![
                Directive(IncludeSystemDirective(IncludeSystemDirectiveNode {
                    indent: Indent::none(),
                    path: "stdio.h".to_string(),
                })),
                Directive(IncludeLocalDirective(IncludeLocalDirectiveNode {
                    indent: Indent::none(),
                    path: "core_intrinsics_io.h".to_string(),
                })),
                Directive(IncludeLocalDirective(IncludeLocalDirectiveNode {
                    indent: Indent::none(),
                    path: "std_io.h".to_string(),
                })),
                Directive(IncludeLocalDirective(IncludeLocalDirectiveNode {
                    indent: Indent::none(),
                    path: "core_intrinsics_math.h".to_string(),
                })),
                DefineFunction(DefineFunctionNode {
                    indent: Indent::none(),
                    identifier: "main".to_string(),
                    arguments: vec![].into_boxed_slice(),
                    ty: "int".to_string(),

                    statements: BlockStatement {
                        indent: Indent::none(),
                        statements,
                    },
                })],
        )
    }
}