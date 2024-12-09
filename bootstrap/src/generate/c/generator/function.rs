use crate::generate::c;
use crate::generate::c::{CallFunctionStatement, CallFunctionStatementResult, DeclareArrayStatement, DeclareVariableStatement, Expression, Indent, LiteralExpression, LiteralIntExpression, LiteralStringExpression, Statement, VariableExpression};
use crate::generate::c::Expression::{Literal, Variable};
use crate::generate::c::generator::Generator;
use crate::generate::c::Statement::CallFunction;
use crate::ir::{CallFunctionNode, CallFunctionOfPackageNode, DeclareFunctionNode, InterpolateStringNode, LiteralNode, LoadValueNode, Node};

impl Generator {
    pub(crate) fn generate_declare_function(&mut self, node: &DeclareFunctionNode) -> c::generator::Result<DeclareFunctionNode> {
        unimplemented!("{node:#?}")
    }

    pub(crate) fn generate_call_function(&mut self, node: &CallFunctionNode) -> c::generator::Result<Vec<Statement>> {
        let function = self.string_table.get(node.function.0).to_string();

        let mut result = vec![];

        let (statements, arguments) = self.generate_call_arguments(&node.arguments)?;
        result.extend(statements);


        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: function,
                arguments: arguments.into(),
                result: Some(CallFunctionStatementResult {
                    indent: Indent::none(),
                    identifier: "arg_2".to_string(),
                    r#type: "double".to_string(),
                }),
            })
        );

        Ok(result)
    }

    pub(crate) fn generate_call_function_with_result(&mut self, node: &CallFunctionNode, call_result: CallFunctionStatementResult) -> c::generator::Result<Vec<Statement>> {
        let function = self.string_table.get(node.function.0).to_string();

        let mut result = vec![];

        let (statements, arguments) = self.generate_call_arguments(&node.arguments)?;
        result.extend(statements);

        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: function,
                arguments: arguments.into(),
                result: Some(call_result),
            })
        );

        Ok(result)
    }


    pub(crate) fn generate_call_function_of_package(&mut self, node: &CallFunctionOfPackageNode) -> c::generator::Result<Vec<Statement>> {
        let mut result = vec![];

        let std = self.string_table.get(node.package.segments[0]).to_string();
        let io = self.string_table.get(node.package.segments[1]).to_string();
        let function = self.string_table.get(node.function.0).to_string();

        let (statements, arguments) = self.generate_call_arguments(&node.arguments)?;
        result.extend(statements);

        result.push(
            CallFunction(CallFunctionStatement {
                indent: Indent::none(),
                identifier: format!("{std}_{io}_{function}"),
                arguments: arguments.into(),
                result: None,
            })
        );

        return Ok(result);
    }

    fn generate_call_arguments(&mut self, args: &[Node]) -> c::generator::Result<(Vec<Statement>, Vec<Expression>)> {
        let mut statements = vec![];
        let mut arguments = vec![];

        for arg in args {
            let arg_identifier = self.scope.push_argument();

            if let Node::LoadValue(LoadValueNode { identifier, ty }) = arg {
                if self.type_table.is_string(ty) {
                    statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                        indent: Indent::none(),
                        identifier: arg_identifier.to_string(),
                        r#type: "const char *".to_string(),
                        expression: Expression::Variable(VariableExpression { indent: Indent::none(), identifier: self.scope.get_variable(identifier).unwrap().to_string(&self.string_table) }),
                    }));


                    arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                    continue;
                }
            }

            if let Node::Literal(LiteralNode::String(str)) = arg {
                statements.push(Statement::DeclareVariable(DeclareVariableStatement {
                    indent: Indent::none(),
                    identifier: arg_identifier.to_string(),
                    r#type: "const char *".to_string(),
                    expression: Expression::Literal(LiteralExpression::String(LiteralStringExpression {
                        indent: Indent::none(),
                        value: self.string_table.get(str.value).to_string(),
                    })),
                }));

                arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                continue;
            }

            // to_string + concatenation
            if let Node::InterpolateString(InterpolateStringNode { nodes }) = arg {
                for node in nodes {
                    if let Node::LoadValue(LoadValueNode { identifier, ty }) = &node {
                        if self.type_table.is_number(ty) {
                            statements.push(Statement::DeclareArray(DeclareArrayStatement {
                                indent: Indent::none(),
                                identifier: arg_identifier.to_string(),
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
                                            identifier: arg_identifier.to_string(),
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

                            arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                            continue;
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
                                        identifier: arg_identifier.to_string(),
                                        r#type: "const char *".to_string(),
                                    }),
                                })
                            );

                            arguments.push(c::Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                            continue;
                        }
                    }

                    if let Node::CallFunction(node) = &node {
                        let temp = self.scope.push_temp();

                        let s = self.generate_call_function_with_result(node, CallFunctionStatementResult{
                            indent: Indent::none(),
                            identifier: temp.to_string(),
                            r#type: "double".to_string(),
                        })?;
                        statements.extend(s);

                        statements.push(Statement::DeclareArray(DeclareArrayStatement {
                            indent: Indent::none(),
                            identifier: arg_identifier.to_string(),
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
                                        identifier: arg_identifier.to_string(),
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

                        arguments.push(Expression::Variable(VariableExpression { indent: Indent::none(), identifier: arg_identifier.to_string() }));
                        continue;
                    }

                    unimplemented!("{node:#?}")
                }
                continue
            }

            if let Node::CallFunction(node) = arg {
                let s = self.generate_call_function(node)?;
                statements.extend(s);
                arguments.push(Expression::Variable(VariableExpression { indent: Indent::none(), identifier: "arg_2".to_string() }));
                continue;
            }

            unimplemented!("{arg:#?}")
        }

        Ok((statements, arguments))
    }
}