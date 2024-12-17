use crate::backend::generate::c;
use crate::backend::generate::c::{CallFunctionStatement, CallFunctionStatementResult, DeclareArrayStatement, DeclareVariableStatement, Expression, Indent, LiteralExpression, LiteralIntExpression, LiteralStringExpression, Statement, VariableExpression};
use crate::backend::generate::c::Expression::{Literal, Variable};
use crate::backend::generate::c::generator::Generator;
use crate::ir::{InterpolateStringNode, LiteralNode, LoadValueFromObjectNode, LoadValueNode, Node};

impl Generator {
    pub(crate) fn interpolate_string(&mut self, node: &InterpolateStringNode) -> c::generator::Result<(Vec<Statement>, Expression)> {
        let mut statements = Vec::new();

        let mut temp_variables = Vec::new();

        for node in &node.nodes {
            if let Node::Literal(LiteralNode::String(string_node)) = &node {
                let temp = self.scope.push_temp();
                statements.push(c::Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: temp.to_string(),
                    r#type: "const char *".to_string(),
                    expression: c::Expression::Literal(LiteralExpression::String(LiteralStringExpression {
                        indent: Indent::none(),
                        value: self.string_table.get(string_node.value).to_string(),
                    })),
                }));
                temp_variables.push(temp)
            } else if let Node::LoadValueFromSelf(node) = &node {
                let temp = self.scope.push_temp();
                statements.push(c::Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: temp.to_string(),
                    r#type: "const char *".to_string(),
                    expression: c::Expression::Variable(VariableExpression {
                        indent: Indent::none(),
                        identifier: format!("self->{}", self.string_table.get(node.property.0)),
                    }),
                }));
                temp_variables.push(temp)
            } else if let Node::LoadValue(LoadValueNode { identifier, ty }) = &node {
                let temp = self.scope.push_temp();

                if self.type_table.is_number(ty) {
                    statements.push(Statement::DeclareArray(DeclareArrayStatement {
                        indent: Indent::none(),
                        identifier: temp.to_string(),
                        r#type: "char".to_string(),
                        size: 20,
                    }));

                    statements.push(Statement::CallFunction(
                        CallFunctionStatement {
                            indent: Indent::none(),
                            identifier: format!("snprintf"),
                            arguments: Box::new([
                                Variable(VariableExpression {
                                    indent: Indent::none(),
                                    identifier: temp.to_string(),
                                }),
                                Literal(LiteralExpression::Int(LiteralIntExpression {
                                    indent: Indent::none(),
                                    value: 20,
                                })),
                                Literal(LiteralExpression::String(LiteralStringExpression {
                                    indent: Indent::none(),
                                    value: "%.0f".to_string(),
                                })),
                                Variable(VariableExpression {
                                    indent: Indent::none(),
                                    identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table),
                                }),
                            ]),
                            result: None,
                        })
                    );
                }
                if self.type_table.is_boolean(ty) {
                    statements.push(Statement::CallFunction(
                        CallFunctionStatement {
                            indent: Indent::none(),
                            identifier: "core_bool_to_string".to_string(),
                            arguments: Box::new([
                                Variable(VariableExpression {
                                    indent: Indent::none(),
                                    identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table),
                                }),
                            ]),
                            result: Some(CallFunctionStatementResult {
                                indent: Indent::none(),
                                identifier: temp.to_string(),
                                r#type: "const char *".to_string(),
                            }),
                        })
                    );
                }

                temp_variables.push(temp);
            } else if let Node::LoadValueFromObject(LoadValueFromObjectNode { object, property, ty }) = &node {
                let temp = self.scope.push_temp();

                statements.push(Statement::DeclareArray(DeclareArrayStatement {
                    indent: Indent::none(),
                    identifier: temp.to_string(),
                    r#type: "char".to_string(),
                    size: 20,
                }));

                statements.push(Statement::CallFunction(
                    CallFunctionStatement {
                        indent: Indent::none(),
                        identifier: format!("snprintf"),
                        arguments: Box::new([
                            Variable(VariableExpression {
                                indent: Indent::none(),
                                identifier: temp.to_string(),
                            }),
                            Literal(LiteralExpression::Int(LiteralIntExpression {
                                indent: Indent::none(),
                                value: 20,
                            })),
                            Literal(LiteralExpression::String(LiteralStringExpression {
                                indent: Indent::none(),
                                value: "%.0f".to_string(),
                            })),
                            Variable(VariableExpression {
                                indent: Indent::none(),
                                identifier: format!("{}.{}", self.scope.get_variable(object).unwrap().to_string(&self.string_table), self.string_table.get(property.0)),
                            }),
                        ]),
                        result: None,
                    })
                );

                temp_variables.push(temp);
            } else if let Node::CallFunction(node) = &node {
                let result_temp = self.scope.push_temp();
                let temp = self.scope.push_temp();

                let s = self.generate_call_function_with_result(node, CallFunctionStatementResult {
                    indent: Indent::none(),
                    identifier: temp.to_string(),
                    r#type: "double".to_string(),
                })?;
                statements.extend(s);

                statements.push(Statement::DeclareArray(DeclareArrayStatement {
                    indent: Indent::none(),
                    identifier: result_temp.to_string(),
                    r#type: "char".to_string(),
                    size: 20,
                }));

                statements.push(Statement::CallFunction(
                    CallFunctionStatement {
                        indent: Indent::none(),
                        identifier: format!("snprintf"),
                        arguments: Box::new([
                            Variable(VariableExpression {
                                indent: Indent::none(),
                                identifier: result_temp.to_string(),
                            }),
                            Literal(LiteralExpression::Int(LiteralIntExpression {
                                indent: Indent::none(),
                                value: 20,
                            })),
                            Literal(LiteralExpression::String(LiteralStringExpression {
                                indent: Indent::none(),
                                value: "%.0f".to_string(),
                            })),
                            Variable(VariableExpression {
                                indent: Indent::none(),
                                identifier: temp.to_string(),
                            }),
                        ]),
                        result: None,
                    })
                );

                temp_variables.push(result_temp);


            }else {
                unimplemented!("{node:#?}")
            }
        }

        let arg_identifier = self.scope.push_argument();

        statements.push(Statement::DeclareArray(DeclareArrayStatement {
            indent: Indent::none(),
            identifier: arg_identifier.to_string(),
            r#type: "char".to_string(),
            size: 100,
        }));

        let mut arguments = vec![
            Variable(VariableExpression {
                indent: Indent::none(),
                identifier: arg_identifier.to_string(),
            }),
            Literal(LiteralExpression::Int(LiteralIntExpression {
                indent: Indent::none(),
                value: 100,
            })),
            Literal(LiteralExpression::String(LiteralStringExpression {
                indent: Indent::none(),
                value: "%s".repeat(temp_variables.len()),
            })),
        ];

        temp_variables.iter().for_each(|t| {
            arguments.push(
                Variable(VariableExpression {
                    indent: Indent::none(),
                    identifier: t.to_string(),
                })
            )
        });

        statements.push(Statement::CallFunction(
            CallFunctionStatement {
                indent: Indent::none(),
                identifier: format!("snprintf"),
                arguments: arguments.into_boxed_slice(),
                result: None,
            })
        );


        Ok((statements, c::Expression::Variable(VariableExpression {
            indent: Indent::none(),
            identifier: arg_identifier.to_string(),
        })))
    }
}