mod literal;

use std::ops::Deref;
use crate::common::StringCache;
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

pub(crate) fn generate(ctx: ir::Context) -> Result<Vec<c::Node>> {
    let mut generator = Generator {
        string_cache: ctx.string_cache,
    };
    generator.generate(ctx.file.body)
}

pub(crate) struct Generator {
    string_cache: StringCache
}

impl Generator {
    pub(crate) fn generate(&mut self, nodes: Vec<Node>) -> Result<Vec<c::Node>> {
        let mut statements = vec![];

        for node in &nodes {

            match node {
                Node::Block(_) => {}
                Node::BreakLoop(_) => {}
                Node::Calculate(_) => {}
                Node::CallFunctionOfObject(_) => {}
                Node::CallFunctionOfPackage(CallFunctionOfPackageNode { package, function, arguments }) => {
                    let std = self.string_cache.get(package.segments[0]).to_string();
                    let io = self.string_cache.get(package.segments[1]).to_string();
                    let function = self.string_cache.get(function.0).to_string();

                    if let ir::Node::Literal(n) = arguments.get(0).unwrap() {
                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: format!("{std}_{io}_{function}"),
                            arguments: Box::new([
                                Literal(self.generate_literal(n)?)
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
                                    identifier: self.string_cache.get(identifier.0).to_string(),
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
                }
                Node::CallFunction(CallFunctionNode { function, arguments }) => {
                    let function = self.string_cache.get(function.0);


                    if let ir::Node::Literal(f) = arguments.get(0).unwrap() {
                        statements.push(Statement::Expression(CallFunction(CallFunctionExpression {
                            indent: Indent::none(),
                            identifier: function.to_string(),
                            arguments: Box::new([
                                Literal(self.generate_literal(f)?)
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

                Node::Unit => {}

                Node::DeclareVariable(DeclareVariableNode { identifier, value, value_type }) => {
                    if let CallFunctionOfPackage(CallFunctionOfPackageNode { package, function, arguments }) = value.deref() {
                        if package.segments.len() == 3 {
                            // HACK for core::intrinsics::math
                            let core = self.string_cache.get(package.segments[0]);
                            let intrinsics = self.string_cache.get(package.segments[1]);
                            let math = self.string_cache.get(package.segments[2]);
                            let func = self.string_cache.get(function.0);

                            let ir::Node::Literal(n) = arguments.get(0).unwrap() else { panic!() };

                            statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                                indent: Indent::none(),
                                identifier: "result".to_string(),
                                r#type: "double".to_string(),
                                expression:
                                CallFunction(CallFunctionExpression {
                                    indent: Indent::none(),
                                    identifier: format!("{}_{}_{}_{}", core, intrinsics, math, func),
                                    arguments: Box::new([
                                        Literal(self.generate_literal(n)?)
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
                _ => {}
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