use crate::build::c;
use crate::build::c::{CallFunctionStatement, CallFunctionStatementResult, CodeExpression, DeclareArrayStatement, ExpressionStatement, ExpressionStatementResult, LiteralExpression, LiteralInt4Expression, LiteralStringExpression, Statement, VariableExpression};
use crate::build::c::Expression::{Code, Literal, Variable};
use crate::build::c::generator::{Generator, scope};
use crate::build::c::generator::scope::{LocalVariable, Storage};
use crate::build::c::Statement::CallFunction;
use crate::common::{GetString, TypeId};
use crate::common::node::Node::{AccessVariable, LiteralString};
use crate::ir::{IrInterpolateStringNode, IrNode};

impl Generator {
    pub(crate) fn interpolate_string(&mut self, node: &IrInterpolateStringNode) -> c::generator::Result<VariableExpression> {
        let mut variables: Vec<scope::Variable> = Vec::with_capacity(node.nodes.len());

        // turns nodes into strings
        for node in &node.nodes {
            if let AccessVariable(node) = node.node() {
                let symbol = self.symbol_table.variable(node.variable);

                if symbol.type_id.is_some() && TypeId::NUMBER == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_num_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));

                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::BOOLEAN == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_bool_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::FLOAT4 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_f4_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::FLOAT8 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_f8_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::INT1 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i1_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::INT2 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i2_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::INT4 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i4_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::INT8 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i8_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::INT16 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i16_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::UINT1 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i1_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::UINT2 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i2_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::UINT4 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i4_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::UINT8 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i8_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else if symbol.type_id.is_some() && TypeId::UINT16 == symbol.type_id.unwrap() {
                    let temp = self.scope.push_temp(Storage::Memory);

                    let variable = symbol.to_string(&self.string_table);

                    self.statements().push(CallFunction(CallFunctionStatement {
                        function: "val_i16_to_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Variable(VariableExpression { variable, cast: None }),
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    }));
                    variables.push(scope::Variable::Temp(temp, Storage::Memory))
                } else {
                    let variable = symbol.to_string(&self.string_table);

                    // self.statements().push(
                    //     c::Statement::DeclareVariable(DeclareVariableStatement {
                    //         variable: temp.to_string(),
                    //         r#type: "const struct val_str *".to_string(),
                    //         expression: c::Expression::Variable(VariableExpression { variable, cast: None }),
                    //     })
                    // );
                    variables.push(scope::Variable::Variable(LocalVariable(variable), Storage::Memory));
                }
            } else if let LiteralString(node) = node.node() {
                let temp = self.scope.push_temp(Storage::Memory);

                let value = self.string_table.get_string(node.value);

                self.statements().push(
                    Statement::CallFunction(CallFunctionStatement {
                        function: "val_str_new_from_c_str".to_string(),
                        arguments: Box::new([
                            c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                            c::Expression::Literal(c::LiteralExpression::String(
                                c::LiteralStringExpression {
                                    value,
                                },
                            )),
                        ]),
                        result: Some(CallFunctionStatementResult {
                            identifier: temp.to_string(),
                            r#type: "struct val_str *".to_string(),
                        }),
                    })
                );

                // self.statements().push(
                //     c::Statement::DeclareVariable(DeclareVariableStatement {
                //         indent: Indent::none(),
                //         variable: temp.to_string(),
                //         r#type: "const struct val_str *".to_string(),
                //         expression: c::Expression::Literal(LiteralExpression::String(LiteralStringExpression { indent: Indent::none(), value })),
                //     })
                // );

                variables.push(scope::Variable::Temp(temp, Storage::Memory))
            } else {
                match node.node() {
                    IrNode::Calculate(_) => {
                        let expression = self.expression(node)?;

                        let string = self.scope.push_temp(Storage::Stack);

                        let temp = self.scope.push_temp(Storage::Memory);
                        self.statements().push(Statement::Expression(ExpressionStatement {
                            expression,
                            result: Some(ExpressionStatementResult { variable: temp.to_string(), r#type: "struct val_num *".to_string() }),
                        }));

                        self.statements().push(CallFunction(CallFunctionStatement {
                            function: "val_to_str".to_string(),
                            arguments: Box::new([
                                c::Expression::Variable(VariableExpression {
                                    variable: temp.to_string(),
                                    cast: Some("struct val *".to_string()),
                                }),
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                            ]),
                            result: Some(CallFunctionStatementResult {
                                identifier: string.to_string(),
                                r#type: "struct val_str *".to_string(),
                            }),
                        }));

                        variables.push(scope::Variable::Temp(string, Storage::Memory));
                    }
                    IrNode::Compare(_) => {
                        let string = self.scope.push_temp(Storage::Stack);

                        let temp = self.scope.push_temp(Storage::Memory);
                        let expression = self.expression(node)?;
                        self.statements().push(Statement::Expression(ExpressionStatement {
                            expression,
                            result: Some(ExpressionStatementResult { variable: temp.to_string(), r#type: "struct val_bool *".to_string() }),
                        }));


                        self.statements().push(CallFunction(CallFunctionStatement {
                            function: "val_bool_to_str".to_string(),
                            arguments: Box::new([
                                c::Expression::Variable(VariableExpression { variable: temp.to_string(), cast: None }),
                                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() })
                            ]),
                            result: Some(CallFunctionStatementResult {
                                identifier: string.to_string(),
                                r#type: "struct val_str *".to_string(),
                            }),
                        }));

                        variables.push(scope::Variable::Temp(string, Storage::Memory));
                    }
                    _ => unimplemented!("{node:#?}")
                }


                // unimplemented!("{node:#?}")
            }
        }

        let temp = self.scope.push_temp(Storage::Stack);
        self.statements().push(Statement::DeclareArray(DeclareArrayStatement {
            identifier: temp.to_string(),
            r#type: "char".to_string(),
            size: 100,
        }));

        let mut arguments = vec![
            Variable(VariableExpression {
                variable: temp.to_string(),
                cast: None,
            }),
            Literal(LiteralExpression::Int4(LiteralInt4Expression {
                value: 100,
            })),
            Literal(LiteralExpression::String(LiteralStringExpression {
                value: "%s".repeat(variables.len()),
            })),
        ];

        variables.iter().for_each(|t| {
// arguments.push(Variable(VariableExpression {
//     indent: Indent::none(),
//     variable: t.to_string(),
// }))
            let s: String = t.to_string();
            arguments.push(Code(CodeExpression { code: s + "->data" }))
        });

        self.statements().push(Statement::CallFunction(CallFunctionStatement {
            function: format!("snprintf"),
            arguments: arguments.into_boxed_slice(),
            result: None,
        }));

        let arg = self.scope.push_argument(Storage::Memory);

        self.statements().push(Statement::CallFunction(CallFunctionStatement {
            function: "val_str_new_from_c_str".to_string(),
            arguments: Box::new([
                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                c::Expression::Variable(VariableExpression { variable: temp.to_string(), cast: None })
            ]),
            result: Some(CallFunctionStatementResult {
                identifier: arg.to_string(),
                r#type: "const struct val_str *".to_string(),
            }),
        }));


        Ok(VariableExpression {
            variable: arg.to_string(),
            cast: None,
        })
    }
}