use crate::build::c;
use crate::build::c::{CallFunctionStatement, DeclareArrayStatement, DeclareVariableStatement, Indent, LiteralExpression, LiteralIntExpression, LiteralStringExpression, Statement, VariableExpression};
use crate::build::c::Expression::{Literal, Variable};
use crate::build::c::generator::Generator;
use crate::common::GetString;
use crate::common::node::Node::{AccessVariable, LiteralString};
use crate::ir::IrInterpolateStringNode;

impl Generator {

    pub(crate) fn interpolate_string(&mut self, node: &IrInterpolateStringNode) -> c::generator::Result<VariableExpression> {
        let mut temps = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            let temp = self.scope.push_temp();
            if let AccessVariable(node) = node.node() {
                let variable = self.symbol_table.variable(node.variable).to_string(&self.string_table);

                self.statements().push(
                    c::Statement::DeclareVariable(DeclareVariableStatement {
                        indent: Indent::none(),
                        variable: temp.to_string(),
                        r#type: "const char *".to_string(),
                        expression: c::Expression::Variable(VariableExpression { indent: Indent::none(), variable }),
                    })
                );
            } else if let LiteralString(node) = node.node() {
                let value = self.string_table.get_string(node.value);
                self.statements().push(
                    c::Statement::DeclareVariable(DeclareVariableStatement {
                        indent: Indent::none(),
                        variable: temp.to_string(),
                        r#type: "const char *".to_string(),
                        expression: c::Expression::Literal(LiteralExpression::String(LiteralStringExpression { indent: Indent::none(), value  })),
                    })
                );
            }

            temps.push(temp)
        }

        let arg = self.scope.push_argument();

        self.statements().push(Statement::DeclareArray(DeclareArrayStatement {
            indent: Indent::none(),
            identifier: arg.to_string(),
            r#type: "char".to_string(),
            size: 100,
        }));

        let mut arguments = vec![
            Variable(VariableExpression {
                indent: Indent::none(),
                variable: arg.to_string(),
            }),
            Literal(LiteralExpression::Int(LiteralIntExpression {
                indent: Indent::none(),
                value: 100,
            })),
            Literal(LiteralExpression::String(LiteralStringExpression {
                indent: Indent::none(),
                value: "%s".repeat(temps.len()),
            })),
        ];

        temps.iter().for_each(|t| {
            arguments.push(Variable(VariableExpression {
                indent: Indent::none(),
                variable: t.to_string(),
            }))
        });

        self.statements().push(Statement::CallFunction(CallFunctionStatement {
            indent: Indent::none(),
            identifier: format!("snprintf"),
            arguments: arguments.into_boxed_slice(),
            result: None,
        }));


        Ok(VariableExpression {
            indent: Indent::none(),
            variable: arg.to_string(),
        })
    }



}