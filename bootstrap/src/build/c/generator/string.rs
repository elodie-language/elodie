use crate::build::c;
use crate::build::c::{CallFunctionStatement, CallFunctionStatementResult, CodeExpression, DeclareArrayStatement, LiteralExpression, LiteralIntExpression, LiteralStringExpression, Statement, VariableExpression};
use crate::build::c::Expression::{Code, Literal, Variable};
use crate::build::c::generator::{Generator, stack};
use crate::build::c::generator::stack::LocalVariable;
use crate::common::GetString;
use crate::common::node::Node::{AccessVariable, LiteralString};
use crate::ir::IrInterpolateStringNode;

impl Generator {
    pub(crate) fn interpolate_string(&mut self, node: &IrInterpolateStringNode) -> c::generator::Result<VariableExpression> {
        let mut variables: Vec<stack::Variable> = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            if let AccessVariable(node) = node.node() {
                let variable = self.symbol_table.variable(node.variable).to_string(&self.string_table);

                // self.statements().push(
                //     c::Statement::DeclareVariable(DeclareVariableStatement {
                //         variable: temp.to_string(),
                //         r#type: "const struct val_str *".to_string(),
                //         expression: c::Expression::Variable(VariableExpression { variable }),
                //     })
                // );
                variables.push(stack::Variable::Variable(LocalVariable(variable)));
            } else if let LiteralString(node) = node.node() {
                let temp = self.stack.push_temp();

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

                variables.push(stack::Variable::Temp(temp))
            }
        }

        let temp = self.stack.push_temp();
        self.statements().push(Statement::DeclareArray(DeclareArrayStatement {
            identifier: temp.to_string(),
            r#type: "char".to_string(),
            size: 100,
        }));

        let mut arguments = vec![
            Variable(VariableExpression {
                variable: temp.to_string(),
            }),
            Literal(LiteralExpression::Int(LiteralIntExpression {
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

        let arg = self.stack.push_argument();

        self.statements().push(Statement::CallFunction(CallFunctionStatement {
            function: "val_str_new_from_c_str".to_string(),
            arguments: Box::new([
                c::Expression::Code(CodeExpression { code: "MEM(tm)".to_string() }),
                c::Expression::Variable(VariableExpression { variable: temp.to_string() })
            ]),
            result: Some(CallFunctionStatementResult {
                identifier: arg.to_string(),
                r#type: "const struct val_str *".to_string(),
            }),
        }));


        Ok(VariableExpression {
            variable: arg.to_string(),
        })
    }
}